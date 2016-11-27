use std;
use std::time::Duration;
use std::collections::HashMap;
use std::net;
use std::sync::{Arc, RwLock};

use utils::thread;
use ::world_store::ServerWorldStore;
use ::protocol::common::ClientInfo;
use super::common::ConnectedClientInfo;

// An async task that on a regular interval:
// - Updates the state of the world (locally)
// - Keeps track of what the client has been sent so far
// - Pushes unseen changes to each client.
pub fn start(
    connected_clients_lock: Arc<RwLock<HashMap<String, ConnectedClientInfo>>>,
    store_lock: Arc<RwLock<ServerWorldStore>>,
    desired_frequency_hz: u64,
) -> thread::CancelSender {
    let network_update_frequency_hz = 5;
    let sleep_duration = Duration::from_millis(1000/network_update_frequency_hz);
    thread::spawn_cancelable(move |cancel_receiver: thread::CancelReceiver| {
        while !cancel_receiver.has_been_canceled() {
            let connected_clients = connected_clients_lock.read().unwrap();
            let store = store_lock.read().unwrap();
            for client_info in connected_clients.values() {
                let world_state_diff = store.world_state_from_version(
                    &client_info.acked_world_version);
                if world_state_diff.new_version == client_info.acked_world_version {
                    println!("Nothing new here!");
                    continue
                }
                println!("some new data to send to the client!");
                // TODO: Queue up message (but don't send it yet so we don't hold onto lock for too
                // long)
            }
            std::thread::sleep(sleep_duration);
        }
    })
}
