use super::common;

// A udp broadcast to notify clients that the server is alive and running.
#[derive(RustcDecodable, RustcEncodable)]
pub struct ServerLifelinePing {
    // Note that the server address should be determined by the incoming request on the client.
    pub public_info: common::PublicServerInfo,
}
