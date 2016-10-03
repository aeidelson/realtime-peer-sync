// world_store is used to keep state and helpers to access the current or changes to the state of
// the world.
//
// Everything in world_store speaks in terms of the `internal_protocol`. It may make sense to move
// to something internal for efficency (protobufs are pretty big), and to avoid diverging API's.
// But this should be ok for right now.
//
// Note: These stores aren't thread safe, so it's up to the caller to lock them appropriately.

mod store_impl;

use protocol::common;


// A thin wrapper around the actual store, containing client-specific code.
pub struct ClientWorldStore {
    world_store_impl: store_impl::WorldStoreImpl,
}

// Note: The client world store doesn't say anything about versioning. It's pretty difficult to
// bake that into the client store, since it's actually a construct of how the data is stored on
// the server. Clients should keep track of the authoritative version seperately.
impl ClientWorldStore {
    pub fn new() -> ClientWorldStore {
        ClientWorldStore{
            world_store_impl: store_impl::WorldStoreImpl::new(),
        }
    }

    // Updates the world state by applying the given diff. The diff should be coppied before being
    // stored, and won't be directly accessible from the WorldStore after this is called.
    //
    // Note: Right now this speaks in diffs rather than events.
    // This may be worth re-considering if we ever want to surface events
    // to the consumer.
    pub fn update_world_state(&mut self, diff: &common::WorldStateDiff) {
        self.world_store_impl.update_world_state(diff);
    }

    // Returns the state of the world, after every diff has been applied to it (in order).
    pub fn current_world_state(&self) -> common::WorldStateDiff {
        self.world_store_impl.world_state_from_beginning().changes
    }
}

// A thin wrapper around the actual store, containing server-specific code.
pub struct ServerWorldStore {
    world_store_impl: store_impl::WorldStoreImpl,
}

impl ServerWorldStore {
    pub fn new() -> ServerWorldStore {
        ServerWorldStore{
            world_store_impl: store_impl::WorldStoreImpl::new(),
        }
    }

    // Updates the world state by applying the given diff. The diff should be coppied before being
    // stored, and won't be directly accessible from the WorldStore after this is called.
    //
    // Note: Right now this speaks in diffs rather than events.
    // This may be worth re-considering if we ever want to surface events
    // to the consumer.
    pub fn update_world_state(&mut self, diff: &common::WorldStateDiff) {
        self.world_store_impl.update_world_state(diff);
    }

    // Returns the state of the world, after every diff has been applied to it (in order).
    // Also returns versioning info.
    pub fn world_state_from_beginning(&self) -> common::ServerWorldStateDiff {
        self.world_store_impl.world_state_from_beginning()
    }

    // World state after (and not including) the given version.
    // Also returns versioning info.
    //
    // NOTE: Should store version in a way that allows us to merge to merge diffs in the future.
    pub fn world_state_from_version(&self, version: &u64) -> common::ServerWorldStateDiff {
        self.world_store_impl.world_state_from_version(version)
    }
}
