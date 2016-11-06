extern crate piston_window;
extern crate RPS;
extern crate uuid;

use std::time;
use std::thread;
use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use piston_window::{PistonWindow, WindowSettings, Event, MouseCursorEvent};

use RPS::{client, consumer_api, server};
use RPS::consumer_api::{WorldState, CalculationEvent};


fn main() {
    let mut client = client::new(client::ClientConfig {
        calculate_updates: calculate_updates_handler,
        desired_calculate_updates_frequency_hz: 5,
    });

    let mut server = server::new(server::ServerConfig {
        client_listen_udp_port: 8889u16,
        calculate_updates: calculate_updates_handler,
        desired_calculate_updates_frequency_hz: 5,
    });

    server.start().unwrap();


    let discovered = client.find_servers(time::Duration::from_secs(3)).unwrap();
    if discovered.len() == 0 {
        panic!("No servers discovered")
    }

    let chosen_server = &discovered[0];
    println!(
        "Connecting to first discovered server: {} {}",
         chosen_server.server_name,
         chosen_server.tcp_server_location);

    client.connect_to_server(&chosen_server);

    // Start piston to draw and handle user input.
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        // TODO(aeidelson): I was having a lot of trouble gating this on clicks. It doesn't seem
        // like clicks and cursor locations are passed in the same event?
        if let Some(cursor_location) = e.mouse_cursor_args() {

            // Create an event to create a new object.
            let mut upserted_fields = HashMap::new();
            upserted_fields.insert(
                String::from("x"),
                consumer_api::FieldValue::Float64Value(cursor_location[0]));
            upserted_fields.insert(
                String::from("y"),
                consumer_api::FieldValue::Float64Value(cursor_location[1]));

            let mut updates = HashMap::new();
            updates.insert(Uuid::new_v4().to_string(),
                           consumer_api::EventObjectUpdate {
                               delete: None,
                               fields_to_upsert: upserted_fields,
                               fields_to_remove: HashSet::new(),
                           });
            client.new_user_events(vec![consumer_api::Event { object_updates: updates }]);
        }

        if let Event::Render(_) = e {
            window.draw_2d(&e, |c, g| {
                // Note: The client can be accessed here and is borrowed.
                // i.e.: let _ = client.user_interaction_started();

                piston_window::clear([1.0; 4], g);

                let world_state = client.get_world_state();
                for object in world_state.objects.values() {
                    let mut x = 0f64;
                    let mut y = 0f64;

                    // TODO(aeidelson): All of this is incredibly aweful and is just here to unblock
                    // working on network syncing. We need a better way to deal with field values.
                    for (k, v) in &object.object_fields {
                        match k.as_str() {
                            "x" => {
                                if let &consumer_api::FieldValue::Float64Value(xValue) = v {
                                    x = xValue;
                                }
                            }
                            "y" => {
                                if let &consumer_api::FieldValue::Float64Value(yValue) = v {
                                    y = yValue;
                                }
                            }
                            _ => {}
                        }
                    }
                    piston_window::rectangle([1.0, 0.0, 0.0, 1.0], // red
                                             [x - 5.0, y - 5.0, 10.0, 10.0],
                                             c.transform,
                                             g);
                }
            });
        }
    }

    server.shutdown();
    client.shutdown();
}

fn calculate_updates_handler(initial_world_state: WorldState,
                             time_since_last_update: time::Duration)
                             -> Vec<CalculationEvent> {
    Vec::new()
}
