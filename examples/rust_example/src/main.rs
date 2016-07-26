extern crate RPS;

use std::time;

use RPS::{client, common, server};
use RPS::common::{
    WorldState,
    CalculationEvent,
};

fn main() {
    let c = client::new(client::ClientConfig{
        calculate_updates: calculate_updates_handler,
        desired_calculate_updates_frequency_hz: 5,
    });

    let s = server::new(server::ServerConfig{
        calculate_updates: calculate_updates_handler,
        desired_calculate_updates_frequency_hz: 5,
    });

    s.start_server().unwrap();
}

fn calculate_updates_handler(
    initial_world_state: WorldState,
    time_since_last_update: time::Duration,
) -> Vec<CalculationEvent> {
    Vec::new()
}
