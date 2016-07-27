use std::time;
use std::io;
use std::thread;

use ::utils;

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
    // TODO: Confirm only one client created.
    return Server{};
}

pub struct Server {
}

impl Server {
    // Starts asynchronously accepting incoming requests and broadcasting updates to connected clients.
    // This function is non-blocking.
    pub fn start(&self) -> io::Result<()> {
        Ok(())
    }

    // Shuts down and cleans up the server.
    pub fn shutdown(self) -> io::Result<()> {
        Ok(())
    }
}
