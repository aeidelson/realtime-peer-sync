use std::vec::Vec;
use std::collections::HashMap;

#[derive(RustcDecodable, RustcEncodable)]
pub struct PublicClientInfo {
    // A random uuid.
    pub client_id: String,
    pub name: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PublicServerInfo {
    pub tcp_port: u16,
    pub name: String,
}

// All information about the requesting client. Contains both public information and
// information that is privileged to just the client and server.
#[derive(RustcDecodable, RustcEncodable)]
pub struct ClientInfo {
    // Information about the client that can be shared with other clients.
    pub public_info: PublicClientInfo,
}

// All information about a server that is sharable to connected clients.
#[derive(RustcDecodable, RustcEncodable)]
pub struct ServerInfo {
}

// Due to the constraints of rustc-serialize, it's a lot easier to use an enum for values than any.
#[derive(RustcDecodable, RustcEncodable)]
pub enum FieldValue {
    StringValue(String),
    Int64Value(i64),
    Float64Value(f64),
}

// Represents a collapsed set of modifications to a single object.
// TODO(aeidelson): It's unclear how specific this API should be. Should re-evaluate this in the
// future and see if it makes sense to be more restrictive (like seperating updates and adds).
#[derive(RustcDecodable, RustcEncodable)]
pub enum ObjectDiff {
    // Used whenever the fields of an object are updated, including when the object is new.
    Upsert(
        // Fields to update/insert (key, value)
        HashMap<String, FieldValue>,
        // Fields to delete (by key)
        Vec<String>),

    // Signals that the object is deleted.
    Delete,
}

// Represents some set of changes to the world state. Can be created by either client or
// server.
#[derive(RustcDecodable, RustcEncodable)]
pub struct WorldStateDiff {
    // A map from object id to changes to that object.
    // If an object doesn't already exist, it will be considered a new object.
    object_change: HashMap<String, ObjectDiff>,
}

// Represents an authoritative view of a change in the world, from one world version to another.
#[derive(RustcDecodable, RustcEncodable)]
pub struct ServerWorldStateDiff {
    // The new version of the world after the diff is applied.
    // If this is less than the actual client version, the diff should be ignored.
    new_version: u64,

    // Changes for the client to apply locally.
    changes: WorldStateDiff,
}

// A single event sent from client to server, used for modifying world state.
#[derive(RustcDecodable, RustcEncodable)]
pub struct Event {
    // A random uuid generated on the client.
    event_id: String,

    // The client that generated the event.
    acting_client_id: String,

    // The set of changes the client wants to apply.
    changes: WorldStateDiff,
}
