mod server_discovery_sync_task;

use std::net;
use std::time;
use std::io;
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use uuid::Uuid;
use bincode;

use ::consumer_api::{
    WorldState,
    UserEvent,
    CalculationEvent,
};
use protocol::common;
use protocol::connect;
use protocol::message_wrapper;
use ::world_store::ClientWorldStore;
use utils::converters;

#[derive(Clone)]
pub struct DiscoveredServerInfo {
    pub server_name: String,
    pub tcp_server_location: net::SocketAddr,
}

// Used to keep track of running user interactions.
pub type UserInteractionToken = String;

// Configuration options, used when instantiating a new client.
pub struct ClientConfig {
    // Port specifically for listening for udp broadcasts for server discovery. Corresponding
    // server field must be configured with same port.
    pub udp_server_broadcast_listener_port: u16,
  
    // Port to listen for most important messages from server. Corresponding server field must be
    // configured with same port.
    pub udp_server_message_listener_port: u16,

    // Transforms the world state on a loop.
    // This is used for physics, AI, or other calculations.
    pub calculate_updates: fn(
        initial_world_state: WorldState,
        time_since_last_update: time::Duration,
    ) -> Vec<CalculationEvent>,

    // Desired number of times to call calculate_updates per second.
    pub desired_calculate_updates_frequency_hz: u32,
}

pub fn new(config: ClientConfig) -> Client {
    // TODO: Confirm only one client created.
    return Client {
        udp_server_message_listener_port: config.udp_server_message_listener_port,
        udp_server_broadcast_listener_port: config.udp_server_broadcast_listener_port,
        calculated_store_lock: Arc::new(RwLock::new(ClientWorldStore::new())),
        client_id: Uuid::new_v4().to_string(),
        connected_server: None,
    };
}

pub struct Client {
    udp_server_broadcast_listener_port: u16,
    udp_server_message_listener_port: u16,

    // Contains the real-time state of the world. Note that this can include optimistic state not
    // povided by the server, like local server calculations. This is what is queried when
    // `get_world_state()` is called.
    calculated_store_lock: Arc<RwLock<ClientWorldStore>>,

    client_id: String,

    // The server that the client (thinks it) is connected to.
    // TODO(aeidelson): We should better handle the case where the connect fails and the server
    // doesn't know about the client.
    connected_server: Option<DiscoveredServerInfo>
}

impl Client {

    // A blocking function that listens for servers for `discovery_time`, before reporting the
    // results.
    pub fn find_servers(&self, discovery_time: time::Duration) -> io::Result<Vec<DiscoveredServerInfo>> {
        Ok(server_discovery_sync_task::start(&self.udp_server_broadcast_listener_port, discovery_time))
    }

    // Connect to the specified server.
    pub fn connect_to_server(&mut self, server: &DiscoveredServerInfo) -> io::Result<()> {
        // TODO(aeidelson): Start listening for world updates from the server.
        self.connected_server = Some(server.clone());

        let client_info = self.create_client_info();
        return self.send_message_to_server(&message_wrapper::ClientServerMessage {
            client_info: client_info,
            payload: message_wrapper::ClientServerMessagePayload::Connect(
                connect::ClientServerConnect { } 
            ),
        });

    }

    // Returns a copy of the current (local) state of the world.
    // TODO(aeidelson): If a copy is too slow, we can explore using a R/W lock or something clever.
    pub fn get_world_state(&self) -> WorldState {
        let calculated_store = self.calculated_store_lock.read().unwrap();

        return converters::internal_to_public::convert_world_state(
            &calculated_store.current_world_state(),
        )
    }

    // Called to tell the system that a user interaction has started or ended.
    // This can be used to prevent clobbering of local state by remote state, as we
    // need to be careful to not mess with the user experience.
    // TODO(aeidelson): This should probably accept object/field so we can be extra smart about
    // things.
    pub fn continuous_user_interaction_started(&self) -> UserInteractionToken {
        String::from("token")
    }

    pub fn continuous_user_interaction_ended(&self, token: &UserInteractionToken) {
    }

    pub fn new_user_events(&self, user_events: Vec<UserEvent>) {
        let internal_events: Vec<common::Event> = user_events.iter().map(
            |event| converters::public_to_internal::convert_event(
                event,
                &Uuid::new_v4().to_string(),
                &self.client_id,
            )
        ).collect();

        // Apply the changes locally.
        let mut calculated_store = self.calculated_store_lock.write().unwrap();
        for internal_event in internal_events {
            calculated_store.update_world_state(
                &internal_event.changes,
            );
        }

        // TODO: Send the events to the actual server.
    }

    // Shuts down and cleans up the client.
    pub fn shutdown(self) -> io::Result<()> {
        Ok(())
    }

    fn send_message_to_server(&mut self, message: &message_wrapper::ClientServerMessage)  -> io::Result<()> {

        // TODO: Return result and handle error.
        let message_byte_vector =
            bincode::rustc_serialize::encode(message, bincode::SizeLimit::Infinite).unwrap();
        let message_byte_slice: &[u8] = message_byte_vector.as_slice();

        // TODO(aeidelson): This would be better if we didn't setup a tcp stream for each message.
        // Should come up with some kind of message framing so we can keep the stream open.
        let mut server_connection = try!(net::TcpStream::connect(
                self.connected_server.as_ref().unwrap().tcp_server_location));
        server_connection.write_all(message_byte_slice).unwrap();
        server_connection.shutdown(net::Shutdown::Both).unwrap();

        Ok(())
    }

    fn create_client_info(&self) -> common::ClientInfo {
        common::ClientInfo {
            public_info: common::PublicClientInfo {
                name: String::from("client_name"),
                client_id: self.client_id.clone(),
            },
        }
    }
}
