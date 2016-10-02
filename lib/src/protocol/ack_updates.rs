
#[derive(RustcDecodable, RustcEncodable)]
pub struct ClientServerAckUpdates {
    pub world_version: u64,
}
