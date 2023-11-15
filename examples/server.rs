use std::fmt;
use std::io::{self, Read, Write};
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
        self.power_consumption = 100.0;
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

fn handle_client(mut stream: TcpStream, smart_socket: &mut SmartSocket) -> io::Result<()> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;

    match buffer.trim() {
        "1" => {
            stream.write_all(format!("{}\n", smart_socket).as_bytes())?;
        }
        "2" => {
            smart_socket.socket_on();
            stream.write_all(b"Socket is turned on\n")?;
        }
        "3" => {
            smart_socket.socket_off();
            stream.write_all(b"Socket is turned off\n")?;
        }
        "4" => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Client requested to exit",
            ));
        }
        _ => {
            stream.write_all(b"Invalid command\n")?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Запуск сервера, порт: 8080...");

    let mut smart_socket = SmartSocket::new("MySocket".to_string());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut smart_socket_clone = smart_socket.clone();
                std::thread::spawn(move || {
                    if let Err(err) = handle_client(stream, &mut smart_socket_clone) {
                        if err.kind() != io::ErrorKind::Other {
                            eprintln!("Error: {}", err);
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Error while accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
