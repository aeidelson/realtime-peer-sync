use std::net;
use std::thread;
use std::sync::mpsc;

// Options for configuring the server.
pub struct ServerConfig {
    pub broadcast_address: net::SocketAddr,
}

pub struct Server {
    config: ServerConfig,
    kill_channel_sender: mpsc::Sender<bool>,
    kill_channel_receiver: mpsc::Receiver<bool>,
}

impl Server {
    pub fn new(config: ServerConfig) -> Server {
        let (s, r) = mpsc::channel();
        return Server{
            config: config,
            kill_channel_sender: s,
            kill_channel_receiver: r,
        };
    }

    pub fn start(&mut self) {
        // Start thread listening for tcp connections.
        let tcp_port = self.start_tcp_responder();
        println!("listening on port {}", tcp_port)
        
        // Start broadcasting to udp
    }

    pub fn stop(mut self) {
    }

    // Start listening for tcp connections in a thread and return the port number
    fn start_tcp_responder(&mut self) -> u16 {
        let listener = net::TcpListener::bind("0.0.0.0:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(_) => {
                        println!("Got request!");
                    },
                    Err(e) => {
                        println!("Got error! {}", e)
                    },
                }
            }
        });
        return port;
    }
}
