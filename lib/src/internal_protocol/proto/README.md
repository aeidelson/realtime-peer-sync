# Protocol Overview

Here is a brief overview of the network protocol.

## Server -> Client UDP packets

* `server_lifeline_ping` is used by the server to tell all clients listending that it's online. This is done via udp broadcast on predetermined port
* `server_world_update` is used to send updates to connected clients. This sent to a port specified
  by the client in `connect`

## Client -> Server TCP requests (and responses)
Note that all requests/responses are wrapped in a `RequestWrapper` or `ResponseWrapper`. This makes
it easier to ensure things like session and client id are sent with (most) requests.

* `connect` is used by the client to a server (discovered through the UDP broadcast). It provides
  the server with the udp address to use for subsequent udp updates. The server also takes this
  oppertunity to return the authoritative world state at the time the client connected
* `disconnect` disconnects the client from the server
* `send_new_events` is used by the client to send modifications to world state to the server. This
  should primarily be user-initiated updates, as other (calculated) updates should be entirely
  client-side.
* `request_updates` can be used by the client to ask the server to push updates. This shouldn't
  usually be needed, and may be removed if there isn't a strong need for it.
