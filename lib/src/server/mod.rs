mod lifeline_ping_async_task;
mod world_update_async_task;
mod tcp_responder_async_task;
mod common;

use std::time;
use std::io;
use std::net;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use ::world_store::ServerWorldStore;
use utils::thread;
use ::consumer_api::{
    WorldState,
    CalculationEvent,
};
use ::protocol::common::ClientInfo;
use self::common::ConnectedClientInfo;


// Configuration options, used when instantiating a new server.
// Note: The make things as in-sync as possible, it's best for this to be called with the same
// options as the ClientConfig is.
pub struct ServerConfig {
    // Transforms the world state on a loop.
    // This is used for physics, AI, or other calculations.
    pub calculate_updates: fn(
        initial_world_state: WorldState,
        time_since_last_update: time::Duration,
    ) -> Vec<CalculationEvent>,


    // Desired number of times to call calculate_updates per second.
    pub desired_calculate_updates_frequency_hz: u32,

    // The port that the client is expected to be listing on for server udp messages.
    pub client_message_udp_port: u16,
    pub client_broadcast_udp_port: u16,
}

pub fn new(config: ServerConfig) -> Server {
    // TODO: Confirm only one server created.
    return Server{
        client_message_udp_port: config.client_message_udp_port,
        client_broadcast_udp_port: config.client_broadcast_udp_port,
        connected_clients_lock: Arc::new(RwLock::new(HashMap::new())),
        store_lock: Arc::new(RwLock::new(ServerWorldStore::new())),
        running_cancelable_threads: vec![],
    };
}

pub struct Server {
    client_message_udp_port: u16,
    client_broadcast_udp_port: u16,

    // Contains info about all connected clients.
    connected_clients_lock: Arc<RwLock<HashMap<String, ConnectedClientInfo>>>,

    // Contains the current state of the world.
    // This is considered to be the authoritative representation of the world.
    store_lock: Arc<RwLock<ServerWorldStore>>,

    running_cancelable_threads: Vec<thread::CancelSender>,
}

impl Server {
    // Starts asynchronously accepting incoming requests and broadcasting updates to connected clients.
    // This function is non-blocking.
    pub fn start(&mut self) -> io::Result<()> {

        // Start listening for tcp requests.
        let (cancel_sender, server_tcp_port) = tcp_responder_async_task::start(
            tcp_responder_async_task::TcpHandlerContext {
                client_message_udp_port: self.client_message_udp_port,
                connected_clients_lock: self.connected_clients_lock.clone(),
                store_lock: self.store_lock.clone(),
            });

        self.running_cancelable_threads.push(cancel_sender);

        println!("Server tcp port: {:?}", server_tcp_port);

        // Send world state updates to connected clients.
        self.running_cancelable_threads.push(world_update_async_task::start(
            self.connected_clients_lock.clone(),
            self.store_lock.clone(),
            // TODO(aeidelson): This should be provided in the config.
            5,
        ));
 
        // Start broadcasting server to clients in same network.
        self.running_cancelable_threads.push(lifeline_ping_async_task::start(
            // TODO(aeidelson): These values should be (optionally?) provided in the config.
            &self.client_broadcast_udp_port, // udp broadcast port.
            "My server", // server name.
            &server_tcp_port, // Server tcp port
        ));

        Ok(())
    }

    // Shuts down and cleans up the server.
    pub fn shutdown(self) -> io::Result<()> {
        for cancelable_sender in self.running_cancelable_threads {
            cancelable_sender.cancel_thread();
        }
        Ok(())
    }
}
