extern crate piston_window;
extern crate RPS;
extern crate uuid;

use std::time;
use std::thread;
use std::collections::HashMap;

use uuid::Uuid;

use piston_window::{PistonWindow, WindowSettings, Event, MouseCursorEvent};

use RPS::{client, common, server};
use RPS::common::{WorldState, CalculationEvent};


fn main() {
    let client = client::new(client::ClientConfig {
        calculate_updates: calculate_updates_handler,
        desired_calculate_updates_frequency_hz: 5,
    });

    let mut server = server::new(server::ServerConfig {
        calculate_updates: calculate_updates_handler,
        desired_calculate_updates_frequency_hz: 5,
    });

    server.start().unwrap();


    let discovered = client.find_servers(time::Duration::from_secs(10)).unwrap();
    for discovered_server in discovered {
        println!("{} {}", discovered_server.server_name, discovered_server.tcp_server_location)
    }

    /*
    // Start piston to draw and handle user input.
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        // TODO(aeidelson): I was having a lot of trouble gating this on clicks. It doesn't seem
        // like clicks and cursor locations are passed in the same event?
        if let Some(cursor_location) = e.mouse_cursor_args() {
            let x_cursor_loc: [u8; 8] = unsafe { std::mem::transmute(cursor_location[0]) };
            let y_cursor_loc: [u8; 8] = unsafe { std::mem::transmute(cursor_location[1]) };

            // Create an event to create a new object.
            let mut updates = HashMap::new();
            updates.insert(Uuid::new_v4().to_string(),
                           common::EventObjectUpdate {
                               delete: None,
                               fields_to_upsert: vec![common::Field {
                                                          key: String::from("x"),
                                                          value: x_cursor_loc.to_vec(),
                                                      },
                                                      common::Field {
                                                          key: String::from("y"),
                                                          value: y_cursor_loc.to_vec(),
                                                      }],
                               fields_to_remove: vec![],
                           });
            client.new_user_events(vec![common::Event { object_updates: updates }]);
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
                    for field in &object.object_fields {
                        match field.key.as_str() {
                            "x" => {
                                let mut ary = [0u8; 8];
                                for i in 0..ary.len() {
                                    if i < field.value.len() {
                                        ary[i] = field.value[i];
                                    }
                                }
                                x = unsafe { std::mem::transmute(ary) }
                            }
                            "y" => {
                                let mut ary = [0u8; 8];
                                for i in 0..ary.len() {
                                    if i < field.value.len() {
                                        ary[i] = field.value[i];
                                    }
                                }
                                y = unsafe { std::mem::transmute(ary) }
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

*/
    server.shutdown();
    client.shutdown();
}

fn calculate_updates_handler(initial_world_state: WorldState,
                             time_since_last_update: time::Duration)
                             -> Vec<CalculationEvent> {
    Vec::new()
}
