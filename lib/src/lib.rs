// TODO: Make private
pub mod peer_connection;
pub mod state_manager;
mod constants;
mod api;

extern crate protobuf;

#[no_mangle]
pub extern fn hello_world() {
    println!("Hello wolrd!")
}
