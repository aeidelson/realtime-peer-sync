
// This struct is the common backend for both ClientWorldStore and ServerWorldStore.
// Note that this isn't thread-safe, so it should be wrapped in a mutex.

use std::collections::HashMap;
use std::collections::HashSet;


use ::internal_protocol::gen::common;

// This struct has most of the shared logic used by both the client and server world states.
pub struct WorldStoreImpl {
    // Keeps track of all changes to the world. World version is determined by position in vector.
    // TODO(aeidelson): This is doesn't scale at all. We will need something smarter soon.
    changes: Vec<common::WorldStateDiff>,
}

impl WorldStoreImpl {
    pub fn new() -> WorldStoreImpl {
        WorldStoreImpl{
            changes: Vec::new(),
        }
    }

    pub fn update_world_state(&mut self, diff: &common::WorldStateDiff) {
        self.changes.push(diff.clone())
    }

    // This function returns the server world state diff, so the client wrapper should extract
    // just the diff.
    pub fn world_state_from_version(&self, version: &u64) -> common::ServerWorldStateDiff {
        common::ServerWorldStateDiff::new()
    }

}

// TODO(aeidelson): It would probably make sense to move these into a util.
// Given a set of diffs, combines them into one mega-diff.
fn combine_changes(diffs: &[common::WorldStateDiff]) -> common::WorldStateDiff {

    // Keyed by object id.
    let mut combined_object_diffs: HashMap<String, common::ObjectDiff> = HashMap::new();

    // Build the set of changes
    for diff in diffs {
        for object_change in diff.get_object_change() {
            let object_id = String::from(object_change.get_key());
            let mut new_object_diff = object_change.get_value().clone();
            if let Some(old_object_diff) = combined_object_diffs.get(&object_id) {
                new_object_diff = combine_object_diffs(old_object_diff, &new_object_diff);
            }
            combined_object_diffs.insert(object_id, new_object_diff);
        }
    }

    // We we build a final world state based on the diffs we collected.
    let mut new_diff = common::WorldStateDiff::new();
    {
        let mut object_changes = new_diff.mut_object_change();
        for (object_id, object_diff) in combined_object_diffs {
            let mut entry = common::WorldStateDiff_ObjectChangeEntry::new();
            entry.set_key(object_id);
            entry.set_value(object_diff);
            object_changes.push(entry);
        }
    }
    new_diff
}

// Combines two diffs for the same object id.
// Note that `first` is assumed to have occured earlier in time than `second`.
fn combine_object_diffs(
    first: &common::ObjectDiff,
    second: &common::ObjectDiff,
) -> common::ObjectDiff {
    // If either the first or second was deleted, we want to just retrun the final state.
    if first.has_deleted_object() || second.has_deleted_object() {
        return second.clone();
    }

    // Otherwise we need to figure out what changed and how to combine the diffs.

    // 1. Populate initially removed and added fields (from `first`)
    let mut upsert_fields: HashMap<String, &[u8]> = HashMap::new();
    let mut deleted_field_keys: HashSet<String> = HashSet::new();

    if first.has_upserted_object() {
        let upserted = first.get_upserted_object();

        for field_key in upserted.get_field_to_delete() {
            deleted_field_keys.insert(field_key.clone());
        }

        for field in upserted.get_field_to_upsert() {
            upsert_fields.insert(String::from(field.get_key()), field.get_value());
        }
    }

    // 2. Look through removed fields in `second`. For each:
    //  1. Remove from added map if there
    //  2. Add to removed set
    // 3. Look through added fields in `second`. For each:
    //  1. Check if in removed set. Remove from there if it is
    //  2. Set in added map. 
    if second.has_upserted_object() {
        let upserted = second.get_upserted_object();

        for field_key in upserted.get_field_to_delete() {
            deleted_field_keys.insert(field_key.clone());
            // In case the field was added in `first`, remove it.
            let _ = upsert_fields.remove(field_key);
        }

        for field in upserted.get_field_to_upsert() {
            upsert_fields.insert(String::from(field.get_key()), field.get_value());
        }
    }
 
    // Build the final object to return.
    let mut new_diff = common::ObjectDiff::new();
    let mut new_upserted = new_diff.take_upserted_object();
    
    {
        let mut fields_to_upsert = new_upserted.mut_field_to_upsert();
        for (key, value) in upsert_fields {
            let mut entry = common::ObjectDiff_UpsertObjectModification_FieldToUpsertEntry::new();
            entry.set_key(key);
            entry.set_value(value.to_vec());
            fields_to_upsert.push(entry)
        }
    }

    {
        let mut fields_to_delete = new_upserted.mut_field_to_delete();
        for key in deleted_field_keys {
            fields_to_delete.push(key);
        }
    }

    new_diff.set_upserted_object(new_upserted);

    new_diff
}
