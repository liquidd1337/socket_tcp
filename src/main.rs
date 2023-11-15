use rand::Rng;
use std::fmt;
use std::io;

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
        self.update_power_consumption();
    }

    fn socket_off(&mut self) {
        self.status = false;
        self.update_power_consumption();
    }

    fn update_power_consumption(&mut self) {
        self.power_consumption = if self.status {
            rand::thread_rng().gen_range(0.1..=100.0)
        } else {
            0.0
        };
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



fn main() {
    let mut socket = SmartSocket::new("MySocket".to_string());

    
    loop {
        println!("Выберите действие: 
        1. Запросить информацию о текущем состоянии розетки
        2. Включить розетку
        3. Выключить розетку
        4. Выход из программы"
        );

        //Меню выбора действия
        let mut socket_operation = String::new();
        io::stdin()
        .read_line(&mut socket_operation)
        .expect("Failed to read line");
        
        let socket_operation = socket_operation.trim().parse().expect("Failed to parse socket operation");
        
        match socket_operation {
            1 => println!("{}",socket),
            2 => {
                socket.socket_on();
                println!("Розетка включена");
            },
            3 => {socket.socket_off(); 
                println!("Розетка выключена");}
            4 => break,
            _ => println!("Неверная команда"),

        }
    
    }
}


