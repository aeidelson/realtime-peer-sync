use std::net;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::collections::HashMap;
use std::iter::FromIterator;
use protobuf::Message;

use ::constants::NETWORK_BUFFER_SIZE_BYTES;
use ::api::gen::server_lifeline_ping::ServerLifelinePing;

// Used po pass around known information about discovered (but not connected) servers.
#[derive(Clone, Debug)]
pub struct DiscoveredServer {
    // TCP server address.
    // Based on combining request ip address and provided tcp port number.
    tcp_addr: net::SocketAddr,
}

pub struct ClientConfig {
    // Port for the client to listen for server alive broadcasts.
    // This needs to be the same port that the server broadcasts to.
    // Note: All other ports will be dynamically assigned.
    pub alive_broadcast_port: u16,
}

pub struct Client {
    config: ClientConfig,
    server_discovery_shutdown_channel_sender: Option<mpsc::Sender<bool>>,
    discovered_servers: Arc<Mutex<HashMap<net::SocketAddr, DiscoveredServer>>>,
}

// Public interface for the client.
impl Client {
    pub fn new(config: ClientConfig) -> Client {
        return Client {
            config: config,
            server_discovery_shutdown_channel_sender: None,
            discovered_servers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Functions for managing discovery of servers.
    pub fn start_discovering_servers(&mut self) {
        assert!(self.server_discovery_shutdown_channel_sender.is_none());
        let (shutdown_sender, shutdown_receiver) = mpsc::channel::<bool>();
        self.server_discovery_shutdown_channel_sender = Some(shutdown_sender);
        let discovered = self.discovered_servers.clone();

        let port = self.config.alive_broadcast_port.clone();
        thread::spawn(move || {
            // Before we start listening for servers, clear the current set of known servers.
            discovered.lock().unwrap().clear();

            // Set up the udp socket for listening.
            let addr = net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), port);
            let socket = net::UdpSocket::bind(addr).unwrap();
            socket.set_nonblocking(true).unwrap();

            let mut buf = [0; NETWORK_BUFFER_SIZE_BYTES];
            loop {
                // Check for server shutdown.
                match shutdown_receiver.try_recv() {
                    // Either shutdown was requested or the channel was disconnected. In either
                    // case, shutdown the server.
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => { break; }
                    // Shutdown wasn't called.
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                match socket.recv_from(&mut buf) {
                    Ok((read, server_udp_broadcast_addr)) => {
                        if read > 0 {
                            let mut broadcast_message = ServerLifelinePing::new();
                            match broadcast_message.merge_from_bytes(&buf[..read]) {
                                Ok(_) => {
                                    // We have a message from the server to handle.
                                    //
                                    // Now we construct a tcp socket addr from udp broadcast ip
                                    // address and the specified tcp port.

                                    let tcp_addr = net::SocketAddr::new(
                                        server_udp_broadcast_addr.ip(),
                                        // TODO: This is potentially unsafe, should find a better
                                        // way to do this.
                                        broadcast_message.get_tcp_port() as u16,
                                    );
                                    discovered.lock().unwrap().insert(
                                        tcp_addr,
                                        DiscoveredServer {
                                            tcp_addr: tcp_addr,
                                        },
                                    );
                                }
                                // Something went wrong.
                                Err(e) => {
                                    println!("something went wrong! {}", e);
                                }
                            }
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

    pub fn discovered_servers(&self) -> Vec<DiscoveredServer> {
        return Vec::from_iter(
            self.discovered_servers.lock().unwrap().values().cloned());

        //let mut discovered_servers: Vec<DiscoveredServer> = Vec::new();
        //for server in self.discovered_servers.lock().unwrap().values() {
        //    discovered_servers.push(server.clone());
        //}
    }

    // TODO: Functions for connecting to a server.
    // TODO: Functions for interacting with a connected server.
}

// Private interface for the client.
