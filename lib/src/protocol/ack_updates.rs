
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ClientServerAckUpdates {
    pub world_version: u64,
}
