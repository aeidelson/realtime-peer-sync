extern crate piston_window;
extern crate RPS;

use std::time;
use std::thread;

use piston_window::{
    PistonWindow,
    WindowSettings,
    Event,
    MouseCursorEvent,
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

    let mut obj_locations: Vec<(f64, f64)> = Vec::new();

    // Start piston to draw and handle user input.
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        let borrowed_locations = &mut obj_locations;

        // Note(aeidelson): I was having a lot of trouble gating this on clicks. It doesn't seem
        // like clicks and cursor locations are passed in the same event?
        if let Some(cursor_location) = e.mouse_cursor_args() {
            borrowed_locations.push((cursor_location[0], cursor_location[1]));
        }

        if let Event::Render(_) = e {
            window.draw_2d(&e, |c, g| {
                // Note: The client can be accessed here and is borrowed.
                // i.e.: let _ = client.user_interaction_started();

                piston_window::clear([1.0; 4], g);

                for &mut (x, y) in borrowed_locations {
                    piston_window::rectangle(
                        [1.0, 0.0, 0.0, 1.0], // red
                        [x - 5.0, y - 5.0, 10.0, 10.0],
                        c.transform,
                        g,
                    );
                }
            });
        }
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
