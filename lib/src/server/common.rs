use std::net;

use ::protocol::common::ClientInfo;

// All information the server knows about a connected client. Is used to store client-specific
// state date (like which versions they have seen).
pub struct ConnectedClientInfo {
    pub client_info: ClientInfo,
    pub socket_addr: net::SocketAddr,
    // Starts at 0 before the client has acked anything.
    pub acked_world_version: u64,
}
