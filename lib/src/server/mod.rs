mod lifeline_ping_async_task;
mod tcp_responder_async_task;

use std::time;
use std::io;
use std::sync::{Arc, RwLock};

use ::world_store::ServerWorldStore;

use utils::thread;

use ::common::{
    WorldState,
    CalculationEvent,
};

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
}

pub fn new(config: ServerConfig) -> Server {
    // TODO: Confirm only one server created.
    return Server{
        store_lock: Arc::new(RwLock::new(ServerWorldStore::new())),
        running_cancelable_threads: vec![],
    };
}

pub struct Server {
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
        let (cancel_sender, server_tcp_port) = tcp_responder_async_task::start();
        self.running_cancelable_threads.push(cancel_sender);

        println!("Server tcp port: {:?}", server_tcp_port);

        // TODO(aeidelson): Start sending updates to connected clients
 
        // Start broadcasting server to clients in same network.
        self.running_cancelable_threads.push(lifeline_ping_async_task::start(
            // TODO(aeidelson): These values should be (optionally?) provided in the config.
            &8888u32, // udp broadcast port.
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
