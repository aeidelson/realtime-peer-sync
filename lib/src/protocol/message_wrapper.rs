use super::common;
use super::connect;
use super::disconnect;
use super::ack_updates;
use super::request_updates;
use super::send_new_events;
use super::world_update;


#[derive(RustcDecodable, RustcEncodable)]
pub enum ClientServerMessagePayload {
    Connect(connect::ClientServerConnect),
    Disconnect(disconnect::ClientServerDisconnect),
    AckUpdates(ack_updates::ClientServerAckUpdates),
    RequestUpdates(request_updates::ClientServerRequestUpdates),
    SendNewEvents(send_new_events::ClientServerSendNewEvents),
}

// A wrapper for messages between the client and the server. Sent over TCP.
#[derive(RustcDecodable, RustcEncodable)]
pub struct ClientServerMessage {
    // All information the server needs to know about the client.
    pub client_info: common::ClientInfo,

    pub payload: ClientServerMessagePayload,
}

#[derive(RustcDecodable, RustcEncodable)]
pub enum ServerClientMessagePayload {
    WorldUpdate(world_update::ServerClientWorldUpdate),
}

// A wrapper for messages between the server and an authenticated client. Sent over UDP.
#[derive(RustcDecodable, RustcEncodable)]
pub struct ServerClientMessage {
    // All information a connected client needs to know about the server.
    pub server_info: common::ServerInfo,

    pub payload: ServerClientMessagePayload,
}
