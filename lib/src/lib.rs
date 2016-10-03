extern crate uuid;
extern crate rustc_serialize;
extern crate bincode;

// Code to set up a client.
pub mod client;

// Code to set up a server.
pub mod server;

// Api definitions required to interact with the library.
pub mod consumer_api;

// The following are internal to the library.
mod world_store;
mod utils;
mod protocol;
