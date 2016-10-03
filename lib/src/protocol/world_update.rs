use super::common;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ServerClientWorldUpdate {
    // The version of the world that the server believes the client has. The client should
    // re-connect if this is greater than the actual client version (since something went horribly
    // wrong). Will be a version of 0 until the client acks an update.
    pub expected_client_version: u64,

    // Required. Contains what has changed since the client's last ack'd version.
    pub modified_world_state: common::ServerWorldStateDiff,
  
}
