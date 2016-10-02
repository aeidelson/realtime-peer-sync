use std::vec::Vec;

use super::common;

// Notifies the server of new client-originated changes (events).
#[derive(RustcDecodable, RustcEncodable)]
pub struct ClientServerSendNewEvents {
    // Required. The latest authoritative world verstion the client has received.
    // Can eventually be used to detect conflicts.
    pub client_world_version: u64,

    // A set of events to notify the server about.
    // Note: events with an id already in use will be ignored.
    pub events: Vec<common::Event>,
}
