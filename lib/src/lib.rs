//use std::ffi::CStr;
use std::boxed::Box;

// TODO: Make private
pub mod peer_connection;
pub mod state_manager;
mod constants;
mod api;
mod client;

extern crate protobuf;



// These are unsafe C-compatible wrappers around the actual rust versions.
// They should be as thin as possible.
//
// Some learnings so far:
// - For supported types, see: https://github.com/Sean1708/rusty-cheddar/blob/master/src/types.rs
// - Need to use package names in-line when defining types (i.e. `std::os::raw::c_char`)
// - Functions inside of impl don't work, functions need to be defined top-level
// - Functions dealing with raw pointers need to be unsafe
// - Any time you would put `fn`, use `extern fn`

/** Common API **/
// Represents a snapshot of the state of the world.
#[repr(C)]
pub struct RpsWorldState {
}

// Something that happened which can modify world state.
#[repr(C)]
pub struct RpsEvent {
}

// An event which is the result of a user action.
pub type RpsUserEvent = RpsEvent;
// An event which is the result of a calculation.
pub type RpsCalculationEvent = RpsEvent;

/** Client API **/
#[repr(C)]
pub struct RpsClientConfig {
    // Transforms the world state on a loop.
    // This is used for physics, AI, or other calculations.
    pub calculation_updater: extern fn(world_state: RpsWorldState, timeSinceLastUpdateMs: u64) -> RpsCalculationEvent,

    // Desired number of times to call `calculation_updater` per second.
    pub desired_calculation_frequency_hz: i32,
}

// We wrap the client so it isn't visible outside of the library.
#[repr(C)]
pub struct RpsClient(client::Client);

fn c_to_rust_client(c_client: *const RpsClient) -> &client::Client {
    let &RpsClient(ref client) = unsafe { &*c_client };
    &client
}

#[no_mangle]
pub extern fn rps_new_client() -> *const RpsClient {
    Box::into_raw(Box::new(RpsClient(client::Client::new())));
}

#[no_mangle]
pub extern fn dbg(c_client: *const RpsClient) {
    c_to_rust_client(c_client).dbg();
}

/** Server API **/

#[repr(C)]
pub struct RpsServerConfig {
    pub calculate_updater: extern fn(world_state: RpsWorldState, timeSinceLastUpdateMs: u64) -> RpsCalculationEvent,

    // Desired number of times to call `calculation_updater` per second.
    pub desired_calculation_frequency_hz: i32,
}
