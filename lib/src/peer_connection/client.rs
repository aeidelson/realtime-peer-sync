use std::net;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use protobuf::Message;

use ::api::gen::server_lifeline_ping::ServerLifelinePing;

pub struct ClientConfig {
    // Port for the client to listen for server alive broadcasts.
    // This needs to be the same port that the server broadcasts to.
    // Note: All other ports will be dynamically assigned.
    pub alive_broadcast_port: u16,
}

pub struct Client {
    config: ClientConfig,
    server_discovery_shutdown_channel_sender: Option<mpsc::Sender<bool>>,
}

// Public interface for the client.
impl Client {
    pub fn new(config: ClientConfig) -> Client {
        return Client {
            config: config,
            server_discovery_shutdown_channel_sender: None,
        }
    }

    // Functions for managing discovery of servers.
    pub fn start_discovering_servers(&mut self) {
        assert!(self.server_discovery_shutdown_channel_sender.is_none());
        let (shutdown_sender, shutdown_receiver) = mpsc::channel::<bool>();
        self.server_discovery_shutdown_channel_sender = Some(shutdown_sender);

        let port = self.config.alive_broadcast_port.clone();
        thread::spawn(move || {
            let addr = net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), port);
            let socket = net::UdpSocket::bind(addr).unwrap();
            socket.set_nonblocking(true);
            let mut buf = [0; 1000];
            loop {
                // Check for server shutdown.
                match shutdown_receiver.try_recv() {
                    // Either shutdown was requested or the channel was disconnected. In either
                    // case, shutdown the server.
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => { break; }
                    // Shutdown wasn't called.
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                // TODO: Only clear what was read.
                for i in 0..buf.len() {
                    buf[i] = 0;
                }

                match socket.recv_from(&mut buf) {
                    Ok((read, addr)) => {
                        if read > 0 {
                            let mut broadcast_message = ServerLifelinePing::new();
                            broadcast_message.merge_from_bytes(&buf);
                            println!(
                                "got message from addr {} containing port {}",
                                addr,
                                broadcast_message.get_tcp_port(),
                            );
                        }
                    }
                    // An error happened or, more likely, there weren't any messages to process
                    // (since the socket was set as non-blocking).
                    Err(_) => {
                        // We don't need to check the socket super frequently, especially becuase this
                        // isn't the main data socket and is used just for discovery.
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        });
    }

    pub fn stop_discovering_servers(&mut self) {
        assert!(self.server_discovery_shutdown_channel_sender.is_some());
        
        self.server_discovery_shutdown_channel_sender.as_ref().unwrap().send(true).unwrap();
    }

    pub fn discovered_servers(&self) {
    }

    // Functions for connecting to a server.
    // Functions for interacting with a connected server.
}

// Private interface for the client.
