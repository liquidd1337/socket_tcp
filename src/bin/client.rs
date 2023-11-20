use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main()  {
    let mut args = std::env::args();
    let addres = args.nth(1).expect("listener must have");

    let mut stream = TcpStream::connect(addres).unwrap();
    loop {
        println!(
            "Выберите действие:
        1. Показать текущую информацию о розетке
        2. Включить рощетку
        3. Выключить розетку
        4. Выход из программы"
        );

        let mut socket_operation = String::new();
        io::stdin().read_line(&mut socket_operation).expect("Ошибка чтения команды");

        let socket_operation = socket_operation
            .trim()
            .parse::<usize>()
            .expect("Неправильная команда");
      
        stream.write_all(socket_operation.to_string().as_bytes()).expect("Ошибка отправки ответа на сервер");

        if socket_operation == 4 {
            println!("Выход...");
            break;
        }

        let mut response = [0; 32];
        stream.read(&mut response).expect("Ошибка чтения ответа от сервера");

        match bytes_to_string(&response) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("Ошибка чтения: {}", e),
        }
    }
}
fn bytes_to_string(bytes: &[u8]) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(bytes.to_vec())
}
