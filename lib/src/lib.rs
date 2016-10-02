extern crate protobuf;
extern crate uuid;
extern crate rustc_serialize;
extern crate bincode;

// Code to set up a client.
pub mod client;

// Code to set up a server.
pub mod server;

// Common definitions required to setup the client and server.
pub mod common;

// The following are internal to the library.
mod world_store;
mod utils;
mod internal_protocol;
mod protocol;
