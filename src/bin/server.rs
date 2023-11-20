use rand::Rng;
use std::fmt::{self, format};
use std::io::{ Write, Read};
use std::net::{TcpListener, TcpStream};

#[derive(Debug, Clone)]
struct SmartSocket {
    name: String,
    status: bool,
    power_consumption: f32,
}

impl SmartSocket {
    fn new(name: String) -> Self {
        SmartSocket {
            name,
            status: false,
            power_consumption: 0.0,
        }
    }

    fn socket_on(&mut self) {
        self.status = true;
        self.power_consumption = rand::thread_rng().gen_range(0.0..=100.0);
    }

    fn socket_off(&mut self) {
        self.status = false;
        self.power_consumption = 0.0;
    }

    fn print_power_consumption(&self) -> String {
        format!("{:.2}", self.power_consumption)
    }

    fn status(&self) -> &str {
        if self.status {
            "on"
        } else {
            "off"
        }
    }

    fn get_display_string(&self) -> String {
        format!(
            " Socket name: {} Status: {}, Current power consumption: {}W \n",
            self.name,
            self.status(),
            self.print_power_consumption(),
        )
    }
}



impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_display_string())
    }
}

fn handle_client(mut stream: TcpStream, socket : &mut SmartSocket) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) if size > 0 => {
            let request = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
            println!("Received request: {}", request);

            let response = match request.as_str() {
                "1" => "lol",
                "2" => {
                    socket.socket_on();
                    "Socket turned ON\n"
                }
                "3" => {
                    socket.socket_off();
                    "Socket turned OFF\n"
                }
                _ => "Invalid command\n"
            };

            stream.write_all(response.as_bytes()).expect("Failed to write to client");

            true
        }
        _ => false,
    } {}
}
fn main() {
    let mut args = std::env::args();
    let addres = args.nth(1).expect("listener must have");

    let listener = TcpListener::bind(addres).unwrap();
    let mut smart_socket = SmartSocket::new("MySocket".to_string());


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {   
                    handle_client(stream, &mut smart_socket); 
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
