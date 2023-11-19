use rand::Rng;
use std::fmt;
use std::io::{ Write, BufRead};
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
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            " Socket name: {}\n Status: {},\n Current power consumption: {}W ",
            self.name,
            self.status(),
            self.print_power_consumption(),
        )
    }
}

fn handle_client(stream: TcpStream, smart_socket: &mut SmartSocket) {
    let mut stream = std::io::BufReader::new(stream);
    dbg!(&stream);
    let mut buffer = String::new();
    dbg!(&buffer);
    println!("Reading client");
    dbg!(&buffer);
    let command = stream.read_line(&mut buffer).unwrap();
    println!("End reading command");
    dbg!(&stream);
    dbg!(command);
        match command {
            1 => {
                stream.get_mut().write_all(format!("{}\n", smart_socket).as_bytes()).unwrap();
            }
            2 => {
                smart_socket.socket_on();
                stream.get_mut().write_all(b"Socket is turned on\n").unwrap();
            }
            3 => {
                smart_socket.socket_off();
                stream.get_mut().write_all(b"Socket is turned off\n").unwrap();
            }
            _ => {
                stream.get_mut().write_all(b"Invalid command\n").unwrap();
            }
        }
        buffer.clear();
        dbg!(&buffer);
}
fn main() {
    let mut args = std::env::args();
    let addres = args.nth(1).expect("listener must have");

    let listener = TcpListener::bind(addres).unwrap();
    let mut smart_socket = SmartSocket::new("MySocket".to_string());


    for conection in listener.incoming() {
        let connection = conection.unwrap();

        println!(
            "Connection established with {}",
            connection.peer_addr().unwrap()
        );
        handle_client(connection, &mut smart_socket)
    }
}
