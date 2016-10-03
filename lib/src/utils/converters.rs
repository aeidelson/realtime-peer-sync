

// Converters to go from the internal (protobuf) API to the public (rusty) API.

pub mod internal_to_public {
    use common as public_common;
    use protocol::common as internal_common;
    use std::collections::HashMap;

    pub fn convert_world_state(
        diff: &internal_common::WorldStateDiff,
    ) -> public_common::WorldState {
        let mut world_state = public_common::WorldState {
            objects: HashMap::new(),
        };

        for (object_id, object_diff) in &diff.object_change {
            let obj = convert_object(object_id, object_diff);
            let _ = world_state.objects.insert(object_id.clone(), obj);
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
        if let &internal_common::ObjectDiff::Upsert(ref upserted_fields, _) = diff {
            for (field_name, field_value) in upserted_fields {
                new_obj.object_fields.push(public_common::Field {
                    key: field_name.clone(),
                    // TODO: USE ACTUAL VALUE
                    value: vec![],
                })
            }
        }

        new_obj
    }
}


// Converters to go from the public (rusty) API to the internal (protobuf) API.

pub mod public_to_internal {
    use common as public_common;
    use protocol::common as internal_common;
    use std::collections::HashMap;

    pub fn convert_event(
        public_event: &public_common::Event,
        event_id: &String,
        acting_client_id: &String,
    ) -> internal_common::Event {

        let mut object_change_map: HashMap<String, internal_common::ObjectDiff> = HashMap::new();
        for (object_id, event_object_update) in &public_event.object_updates {
            object_change_map.insert(object_id.clone(), convert_object_change(event_object_update));
        }

        internal_common::Event {
            event_id: event_id.clone(),
            acting_client_id: acting_client_id.clone(),
            changes: internal_common::WorldStateDiff {
                object_change: object_change_map,
            },
        }
    }

    fn convert_object_change(
        public_object_update: &public_common::EventObjectUpdate,
    ) -> internal_common::ObjectDiff {

        // Check for the deleted flag being set.
        if public_object_update.delete == Some(true) {
            return internal_common::ObjectDiff::Delete
        }

        // If the deleted field isn't set, we want to populate the diff with the added/removed
        // fields.
        internal_common::ObjectDiff::Upsert(
            // Upserted fields.
            public_object_update.fields_to_upsert.iter().map(|field| {
                // TODO: Use real value
                (field.key.clone(), internal_common::FieldValue::StringValue(String::from("FIX ME!")))
            }).collect(),
            
            // Fields to delete.
            public_object_update.fields_to_remove.iter().map(|field_name| {
                field_name.clone()
            }).collect())
    }
}

