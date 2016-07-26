extern crate piston_window;
extern crate RPS;

use std::time;
use std::thread;

use piston_window::{
    PistonWindow,
    WindowSettings,
};

use RPS::{client, common, server};
use RPS::common::{
    WorldState,
    CalculationEvent,
};


fn main() {
    let client = client::new(client::ClientConfig{
        calculate_updates: calculate_updates_handler,
        desired_calculate_updates_frequency_hz: 5,
    });

    let server = server::new(server::ServerConfig{
        calculate_updates: calculate_updates_handler,
        desired_calculate_updates_frequency_hz: 5,
    });

    server.start().unwrap();

    // Start piston to draw and handle user input.
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            // Note: The client can be accessed here and is borrowed.
            let _ = client.user_interaction_started();
            //client.shutdown();

            piston_window::clear([1.0; 4], g);
            piston_window::rectangle([1.0, 0.0, 0.0, 1.0], // red
                                      [0.0, 0.0, 100.0, 100.0],
                                      c.transform, g);
        });
    }
    
    server.shutdown();
    client.shutdown();
}

fn calculate_updates_handler(
    initial_world_state: WorldState,
    time_since_last_update: time::Duration,
) -> Vec<CalculationEvent> {
    Vec::new()
}
