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
        2. Выключить рощетку
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

        let mut response = String::new();
        stream.read_to_string(&mut response).expect("Ошибка чтения ответа от сервера");

        println!("{}", response);

        if socket_operation == 4 {
            break;
        }
    }


}
