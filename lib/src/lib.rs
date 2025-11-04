// static mut IP: String = String::new();

// use std::net::TcpStream;
// use std::str;
// use std::io::{self, BufRead, BufReader, Write};

// pub fn enter() -> String {
//     let mut value = String::new();
//     std::io::stdin().read_line(&mut value).unwrap();
//     let value: String = value.trim().parse().unwrap();
//     return value;
// }

// pub fn initialize(ip_: String) {
//     unsafe {
//         IP = ip_;
//     }
// }

// pub fn begining() {
//     unsafe {
//         let mut stream = TcpStream::connect(&IP[..]).expect("Could not connect to server");
//         loop {
//             let mut input = String::new();
//             let mut buffer: Vec<u8> = Vec::new();
//             io::stdin().read_line(&mut input).expect("Failed to read from stdin");
//             stream.write(input.as_bytes()).expect("Failed to write to server");

//             let mut reader = BufReader::new(&stream);

//             reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
//             print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
//         }
//     }
// }