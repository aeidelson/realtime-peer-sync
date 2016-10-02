mod server_discovery_sync_task;

use std::net;
use std::time;
use std::io;
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use uuid::Uuid;
use protobuf::Message;

use ::common::{
    WorldState,
    UserEvent,
    CalculationEvent,
};

use internal_protocol::gen::common as internal_common;
use internal_protocol::gen::message_wrapper;
use internal_protocol::gen::message_wrapper::ClientServerMessage;
use internal_protocol::gen::common::ClientInfo;
//use internal_protocol::gen::connect::ClientServerConnect;
use ::world_store::ClientWorldStore;
use utils::converters;

// testing
use protocol::common as protocol_common;
use bincode;

pub struct DiscoveredServerInfo {
    pub server_name: String,
    pub tcp_server_location: net::SocketAddr,
}

// Used to keep track of running user interactions.
pub type UserInteractionToken = String;

// Configuration options, used when instantiating a new client.
pub struct ClientConfig {
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
    let t = protocol_common::PublicClientInfo{
        client_id: String::from(""),
        name: String::from(""),
    };
    let encoded = bincode::rustc_serialize::encode(&t, bincode::SizeLimit::Infinite).unwrap();

    // TODO: Confirm only one client created.
    return Client {
        calculated_store_lock: Arc::new(RwLock::new(ClientWorldStore::new())),
        client_id: Uuid::new_v4().to_string(),
        server_connection: None,
    };
}

pub struct Client {
    // Contains the real-time state of the world. Note that this can include optimistic state not
    // povided by the server, like local server calculations. This is what is queried when
    // `get_world_state()` is called.
    calculated_store_lock: Arc<RwLock<ClientWorldStore>>,

    client_id: String,

    server_connection: Option<net::TcpStream>,
}

impl Client {

    // A blocking function that listens for servers for `discovery_time`, before reporting the
    // results.
    pub fn find_servers(&self, discovery_time: time::Duration) -> io::Result<Vec<DiscoveredServerInfo>> {
        Ok(server_discovery_sync_task::start(&8888u32, discovery_time))
    }

    // Connect to the specified server.
    pub fn connect_to_server(&mut self, server: &DiscoveredServerInfo) -> io::Result<()> {
        self.server_connection = Some(try!(net::TcpStream::connect(server.tcp_server_location)));

        let mut message = ClientServerMessage::new();
        {
            message.set_client_info(self.create_client_info());
        }
        // We get the mut connect just to ensure it's set, even though it's empty.
        let _ = message.mut_connect();

        self.send_message_to_server(&message);

        Ok(())
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
        let internal_events: Vec<internal_common::Event> = user_events.iter().map(
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
                internal_event.get_changes(),
            );
        }

        // TODO: Send the events to the actual server.
    }

    fn send_message_to_server(&mut self, message: &message_wrapper::ClientServerMessage) {
        let server_stream = self.server_connection.as_mut().unwrap();

        // TODO: Return result and handle error.
        let message_byte_vector  = message.write_to_bytes().unwrap();
        let message_byte_slice: &[u8] = message_byte_vector.as_slice();

        server_stream.write_all(message_byte_slice).unwrap();
    }

    fn create_client_info(&self) -> ClientInfo {
        let mut client_info = ClientInfo::new();
        {
            let mut public_client_info = client_info.mut_public_info();
            public_client_info.set_client_id(self.client_id.clone());
        }
        client_info
    }

    // Shuts down and cleans up the client.
    pub fn shutdown(self) -> io::Result<()> {
        Ok(())
    }
}
