

// Converters to go from the internal (protobuf) API to the public (rusty) API.

mod internal_to_public {
    use common as public_common;
    use internal_protocol::gen::common as internal_common;
    use std::collections::HashMap;

    pub fn convert_world_state(
        diff: &internal_common::WorldStateDiff,
    ) -> public_common::WorldState {
        let mut world_state = public_common::WorldState {
            objects: HashMap::new(),
        };

        for change_entity in diff.get_object_change() {
            let object_id = String::from(change_entity.get_key());
            let obj = convert_object(&object_id, change_entity.get_value());
            let _ = world_state.objects.insert(object_id, obj);
        }

        world_state
    }

    fn convert_object(
        object_id: &String,
        diff: &internal_common::ObjectDiff,
    ) -> public_common::Object {
        let mut new_obj = public_common::Object {
            object_id: object_id.clone(),
            object_fields: Vec::new(),
        };

        // If the object has fields, add them. We ignore any deleted fields.
        if diff.has_upserted_object() {
            for field_entry in diff.get_upserted_object().get_field_to_upsert() {
                new_obj.object_fields.push(public_common::Field {
                    key: String::from(field_entry.get_key()),
                    value: field_entry.get_value().to_vec(),
                })
            }
        }

        new_obj
    }
}


// Converters to go from the public (rusty) API to the internal (protobuf) API.

mod public_to_internal {
    use protobuf::RepeatedField;

    use common as public_common;
    use internal_protocol::gen::common as internal_common;

    pub fn convert_event(
        public_event: &public_common::Event,
        event_id: &String,
        acting_client_id: &String,
    ) -> internal_common::Event {
        let mut internal_event = internal_common::Event::new();
        internal_event.set_event_id(event_id.clone());
        internal_event.set_acting_client_id(acting_client_id.clone());

        {
            let mut changes = internal_event.mut_changes();
            let mut object_changes = changes.mut_object_change();
            for (object_id, event_object_update) in &public_event.object_updates {
                let mut entry = internal_common::WorldStateDiff_ObjectChangeEntry::new();
                entry.set_key(object_id.clone());
                entry.set_value(convert_object_change(event_object_update));
                object_changes.push(entry);
            }
        }

        internal_event
    }

    fn convert_object_change(
        public_object_update: &public_common::EventObjectUpdate,
    ) -> internal_common::ObjectDiff {

        let mut object_diff = internal_common::ObjectDiff::new();

        // Check for the deleted flag being set.
        if public_object_update.delete == Some(true) {
            // Will cause the deleted field to be populated.
            let _ = object_diff.mut_deleted_object();
            return object_diff;
        }

        // If the deleted field isn't set, we want to populate the diff with the added/removed
        // fields.
        {
            let mut object_modification = object_diff.mut_upserted_object();
            let upserted_fields: Vec<internal_common::ObjectDiff_UpsertObjectModification_FieldToUpsertEntry> =
                public_object_update.fields_to_upsert.iter().map(|field| {
                    let mut entry = internal_common::ObjectDiff_UpsertObjectModification_FieldToUpsertEntry::new();
                    entry.set_key(field.key.clone());
                    entry.set_value(field.value.clone());

                    entry
                }).collect();
            object_modification.set_field_to_upsert(RepeatedField::from_vec(upserted_fields));

            let removed_fields: Vec<String> =
                public_object_update.fields_to_remove.iter().map(|field_name| {
                    field_name.clone()
                }).collect();
            object_modification.set_field_to_delete(RepeatedField::from_vec(removed_fields));
        }


        object_diff
    }
}

