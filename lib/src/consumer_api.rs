use std::collections::{HashMap, HashSet};

// For convenience, we re-export some core types from our internal API:
pub use protocol::common::{
    ObjectId,
    FieldName,
    FieldValue
};

// Represents a single field in an object.
pub struct Field {
    pub key: FieldName,

    // Note that the value will be opaque to the system.
    pub value: FieldValue,
}

// Represents a single object in the "world".
pub struct Object {
    pub object_id: ObjectId,

    // Fields stored in the object.
    pub object_fields: HashMap<FieldName, FieldValue>,
}

// Represents the state of the world.
pub struct WorldState {
    // Contains all objects that are in the world, keyed by object id.
    pub objects: HashMap<ObjectId, Object>,
}

// Events represent something occuring that modifies the world state.
// This should be able to encode any changes to the world state that a consumer may want to make:
// - Add objects
// - Delete objects
// - Add fields to objects
// - Remove fields from objects
// - Modify fields in objects

pub struct Event {
    // Updates to be applied to objects.
    // Note that if an object with the corresponding id doesn't exist, one will be created.
    // TODO(aeidelson): Should consider if a more explicit API would be better. Current concern is
    // if the object upsert may lead to bugs if accidentially using objects before explicitly
    // "creating" them, or if the create failed.
    pub object_updates: HashMap<ObjectId, EventObjectUpdate>,
}

// Represents an update to a single object.
// TODO(aeidelson): This may be cleaner as wrapped enums? This API also isn't the best.
pub struct EventObjectUpdate {
    // Should the object be deleted? If set, the following fields are ignored.
    pub delete: Option<bool>,

    pub fields_to_upsert: HashMap<FieldName, FieldValue>,
    pub fields_to_remove: HashSet<FieldName>,
}

// An event which is the result of a user action.
pub type UserEvent = Event;

// An event which is the result of a calculation.
pub type CalculationEvent = Event;

