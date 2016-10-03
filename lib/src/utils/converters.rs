

// Converters to go from the internal (protobuf) API to the public (rusty) API.

pub mod internal_to_public {
    use consumer_api;
    use protocol::common;
    use std::collections::HashMap;

    pub fn convert_world_state(
        diff: &common::WorldStateDiff,
    ) -> consumer_api::WorldState {
        let mut world_state = consumer_api::WorldState {
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
        diff: &common::ObjectDiff,
    ) -> consumer_api::Object {
        let mut new_obj = consumer_api::Object {
            object_id: object_id.clone(),
            object_fields: HashMap::new(),
        };

        // If the object has fields, add them. We ignore any deleted fields.
        if let &common::ObjectDiff::Upsert(ref upserted_fields, _) = diff {
            new_obj.object_fields = upserted_fields.clone();
        }

        new_obj
    }
}


// Converters to go from the public (rusty) API to the internal (protobuf) API.

pub mod public_to_internal {
    use consumer_api;
    use protocol::common;
    use std::collections::HashMap;

    pub fn convert_event(
        public_event: &consumer_api::Event,
        event_id: &String,
        acting_client_id: &String,
    ) -> common::Event {

        let mut object_change_map: HashMap<String, common::ObjectDiff> = HashMap::new();
        for (object_id, event_object_update) in &public_event.object_updates {
            object_change_map.insert(object_id.clone(), convert_object_change(event_object_update));
        }

        common::Event {
            event_id: event_id.clone(),
            acting_client_id: acting_client_id.clone(),
            changes: common::WorldStateDiff {
                object_change: object_change_map,
            },
        }
    }

    fn convert_object_change(
        public_object_update: &consumer_api::EventObjectUpdate,
    ) -> common::ObjectDiff {

        // Check for the deleted flag being set.
        if public_object_update.delete == Some(true) {
            return common::ObjectDiff::Delete
        }

        // If the deleted field isn't set, we want to populate the diff with the added/removed
        // fields.
        common::ObjectDiff::Upsert(
            // Upserted fields.
            public_object_update.fields_to_upsert.clone(),
            
            // Fields to delete.
            public_object_update.fields_to_remove.clone())
    }
}

