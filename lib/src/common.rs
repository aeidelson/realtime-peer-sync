use std::collections::HashMap;

// Note: This is intentionally pretty flat / basic, to make it easer when we support C
// bindings.

pub type FieldKey = String;
pub type FieldValue = Vec<u8>;

// Represents a single field in an object.
pub struct Field {
    pub key: FieldKey,

    // Note that the value will be opaque to the system.
    pub value: FieldValue,
}

// An random UUIDv4, to be provided by the client.
pub type ObjectId = String;

// Represents a single object in the "world".
pub struct Object {
    pub object_id: ObjectId,

    // Fields stored in the object.
    // Note: The system dedupes based on key, so there theoretically shouldn't be any
    // duplicates.
    pub object_fields: Vec<Field>,
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

    pub fields_to_upsert: Vec<Field>,
    pub fields_to_remove: Vec<FieldKey>,
}

// An event which is the result of a user action.
pub type UserEvent = Event;

// An event which is the result of a calculation.
pub type CalculationEvent = Event;

