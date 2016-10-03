
// This struct is the common backend for both ClientWorldStore and ServerWorldStore.
// Note that this isn't thread-safe, so it should be wrapped in a mutex.

use std::collections::HashMap;
use std::collections::HashSet;

use protocol::common;

// This struct has most of the shared logic used by both the client and server world states.
pub struct WorldStoreImpl {
    // Keeps track of all changes to the world. World version is determined by position in vector
    // (plus 1).
    // TODO(aeidelson): This is doesn't scale at all. We will need something smarter soon.
    changes: Vec<common::WorldStateDiff>,
}

pub const BEGINNING_OF_TIME_VERSION: u64 = 0;

impl WorldStoreImpl {
    pub fn new() -> WorldStoreImpl {
        WorldStoreImpl{
            changes: Vec::new(),
        }
    }

    pub fn update_world_state(&mut self, diff: &common::WorldStateDiff) {
        self.changes.push(diff.clone())
    }

    // A convenience method for getting the world state from beginning of time.
    pub fn world_state_from_beginning(&self) -> common::ServerWorldStateDiff {
        self.world_state_from_version(&BEGINNING_OF_TIME_VERSION)
    }

    // Returns the world state diff, not including the provided version index.
    //
    // This function returns the server world state diff, so the client wrapper should extract
    // just the diff.
    pub fn world_state_from_version(&self, version: &u64) -> common::ServerWorldStateDiff {
        let head_version = self.changes.len();

        // We need to special-case the version of 0 so the index doesn't end up in negative.
        let mut from_index: usize = 0;
        if version > &BEGINNING_OF_TIME_VERSION {
            // HACK(aeidelson): This is definitely going to cause problems eventually and crash.
            from_index = (version - 1) as usize;
        }

        let world_state_diff = combine_changes(&self.changes[from_index..]);

        common::ServerWorldStateDiff {
            new_version: head_version as u64,
            changes: world_state_diff,
        }
    }

}

// TODO(aeidelson): It would probably make sense to move these into a util.
// Given a set of diffs, combines them into one mega-diff.
fn combine_changes(diffs: &[common::WorldStateDiff]) -> common::WorldStateDiff {

    // Keyed by object id.
    let mut combined_object_diffs: HashMap<String, common::ObjectDiff> = HashMap::new();

    // Build the set of changes
    for diff in diffs {
        for (object_id, object_diff) in &diff.object_change {
            let mut new_object_diff = object_diff.clone();

            if let Some(old_object_diff) = combined_object_diffs.get(object_id) {
                new_object_diff = combine_object_diffs(old_object_diff, &new_object_diff);
            }
            combined_object_diffs.insert(object_id.clone(), new_object_diff);
        }
    }

    // We we build a final world state based on the diffs we collected.
    common::WorldStateDiff {
        object_change: combined_object_diffs,
    }
}

// Combines two diffs for the same object id.
// Note that `first` is assumed to have occured earlier in time than `second`.
fn combine_object_diffs(
    first: &common::ObjectDiff,
    second: &common::ObjectDiff,
) -> common::ObjectDiff {
    // If either the first or second was deleted, we want to just retrun the final state.
    if first == &common::ObjectDiff::Delete || second == &common::ObjectDiff::Delete {
        return second.clone();
    }

    // Otherwise we need to figure out what changed and how to combine the diffs.

    // 1. Populate initially removed and added fields (from `first`)
    let mut upsert_fields: HashMap<String, common::FieldValue> = HashMap::new();
    let mut delete_fields: HashSet<String> = HashSet::new();

    if let &common::ObjectDiff::Upsert(ref actually_upserted_fields, ref actually_deleted_fields) = first {
        upsert_fields = actually_upserted_fields.clone();
        delete_fields = actually_deleted_fields.clone();
    }

    // 2. Look through removed fields in `second`. For each:
    //  1. Remove from added map if there
    //  2. Add to removed set
    // 3. Look through added fields in `second`. For each:
    //  1. Check if in removed set. Remove from there if it is
    //  2. Set in added map. 
    if let &common::ObjectDiff::Upsert(ref second_upserted_fields, ref second_deleted_fields) = first {

        for key in second_deleted_fields {
            delete_fields.insert(key.clone());
            let _ = upsert_fields.remove(key);
        }

        for (k, v) in second_upserted_fields {
            upsert_fields.insert(k.clone(), v.clone());
        }
    }
 
    // Build the final object to return.
    common::ObjectDiff::Upsert(upsert_fields, delete_fields)
}
