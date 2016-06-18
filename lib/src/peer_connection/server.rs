use std::net;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::io::Read;

pub struct Message {
}

pub type ClientId = String;

pub struct ClientInfo {
}

// Options for configuring the server.
pub struct ServerConfig {
    // The broadcast address that server ping messages will be sent to.
    // TODO(aeidelson): 255.255.255.255 may work?
    pub broadcast_address: net::SocketAddr,

    // Indicates if the server should accept the provided client's request to join the server.
    // pub should_accept_client: fn(client_info: &ClientInfo) -> bool,

    // Handle a (tcp) message from a connected client.
    // pub tcp_message_handler: fn(client_info: &ClientInfo, message: Message) -> Message,
}

// The server is responsible for:
// - Broadcasting its existence for clients to join
// - Deserializing incoming requests and handles them:
//   - Join/Leave requests are handled by the server
//   - Any other request is passed through to the appropriate handler
// - Enables querying connected clients and broadcasting to a subset of them
pub struct Server {
    config: ServerConfig,

    // Is used to coordinate the shutdown of the server. A value of true means all threads should
    // stop.
    shutdown_channel_senders: Vec<mpsc::Sender<bool>>,

    // Is used for sending messages directly to clients.
    direct_udp_socket: net::UdpSocket,
}

// Contains the public interface for Server.
impl Server {
    pub fn new(config: ServerConfig) -> Server {
        return Server {
            config: config,
            shutdown_channel_senders: Vec::new(),
            direct_udp_socket: net::UdpSocket::bind("0.0.0.0:0").unwrap(),
        };
    }

    pub fn start(&mut self) {
        // Start thread listening for tcp connections.
        let tcp_port = self.start_tcp_responder();

        // Start broadcasting to udp
        self.start_udp_alive_broadcast(tcp_port);
    }

    // Sends a (udp) message directly to a single connected client.
    pub fn direct_udp_message(&mut self, client_id: &ClientId, message: &Message) {
        // TODO: Lookup client address, serialize and send message.
        let mut msg= String::from("Direct message!");
        self.direct_udp_socket.send_to(msg.as_bytes(), "0.0.0.0:0").unwrap();
    }

    // Shuts down the server and consumes itself.
    pub fn shutdown(mut self) {
        for shutdown_channel_sender in &self.shutdown_channel_senders {
            // Shutdown the thread.
            shutdown_channel_sender.send(true).unwrap();
        }
    }
}

// Contains private methods.
impl Server {
    // Start listening for tcp connections in a thread and return the port number
    fn start_tcp_responder(&mut self) -> u16 {
        let listener = net::TcpListener::bind("0.0.0.0:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        // We create a channel to listen for the shutdown signal.
        let (shutdown_sender, shutdown_receiver) = mpsc::channel::<bool>();
        self.shutdown_channel_senders.push(shutdown_sender);

        thread::spawn(move || {
            loop {
                // Will block until there's an incoming request.
                let (mut stream, addr) = listener.accept().unwrap();

                // Check if shutdown was requested.
                // TODO(aeidelson): Unfortunately there's no good way to automatically run this now,
                // aside from waiting for an incoming tcp connection.
                match shutdown_receiver.try_recv() {
                    // Either shutdown was requested or the channel was disconnected. In either
                    // case, shutdown the server.
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => { break; }
                    // Shutdown wasn't called.
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                // Handle the incoming request.
                let mut result_str = String::new();
                stream.read_to_string(&mut result_str).unwrap();
            }
        });
        return port;
    }

    // Starts a thread to signal to clients that the server is alive.
    fn start_udp_alive_broadcast(&mut self, tcp_server_port: u16) {
        // We create a channel to listen for the shutdown signal.
        let (shutdown_sender, shutdown_receiver) = mpsc::channel::<bool>();
        self.shutdown_channel_senders.push(shutdown_sender);

        let broadcast_address = self.config.broadcast_address.clone();

        thread::spawn(move || {
            // Note: This socket is private to this thread because it shouldn't be needed
            // externally. (Direct broadcasts happen over their own socket).
            let mut udp_broadcast_socket = net::UdpSocket::bind("0.0.0.0:0").unwrap();
            udp_broadcast_socket.set_broadcast(true).unwrap();

            // TODO: remove multicast_loop when not testing locally
            udp_broadcast_socket.set_multicast_loop_v4(true).unwrap();

            loop {
                // Check for server shutdown.
                match shutdown_receiver.try_recv() {
                    // Either shutdown was requested or the channel was disconnected. In either
                    // case, shutdown the server.
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => { break; }
                    // Shutdown wasn't called.
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                // Build a message
                // TODO(aeidelson): This message should be structured and contain the tcp port
                // address, so clients know how to reach the server.
                let mut msg= String::from("Server is alive!");
                udp_broadcast_socket.send_to(msg.as_bytes(), broadcast_address).unwrap();
                println!("braodcast to {}", broadcast_address);
                thread::sleep(Duration::from_millis(500));
            }
        });
    }
}
