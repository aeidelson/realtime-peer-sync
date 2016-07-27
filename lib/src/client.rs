use std::time;
use std::io;

use ::common::{
    WorldState,
    UserEvent,
    CalculationEvent,
};

pub struct DiscoveredServerInfo {
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
    // TODO: Confirm only one client created.
    return Client{};
}

pub struct Client {
}

impl Client {

    // A blocking function that listens for servers for `discovery_time`, before reporting the
    // results.
    pub fn find_servers(&self, discovery_time: time::Duration) -> io::Result<Vec<DiscoveredServerInfo>> {
        Ok(Vec::new())
    }

    // Connect to the specified server.
    pub fn connect_to_server(&self, server: &DiscoveredServerInfo) -> io::Result<()> {
        Ok(())
    }

    // Returns a copy of the current (local) state of the world.
    // TODO(aeidelson): If a copy is too slow, we can explore using a R/W lock or something clever.
    pub fn get_world_state(&self) -> WorldState {
        return WorldState{}
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
    }

    // Shuts down and cleans up the client.
    pub fn shutdown(self) -> io::Result<()> {
        Ok(())
    }
}
