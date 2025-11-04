use std::{
    net::TcpStream,
    thread,
    io::{self, Read, Write},
    sync::mpsc,
};

#[allow(dead_code)]
fn encryption(value: &[u8]) -> Vec<u8> {
    let ans = Vec::from(value);
    return ans;
}
#[allow(dead_code)]
fn decryption(value: &[u8]) -> Vec<u8> {

    let ans = Vec::from(value);
    return ans;
}


fn main() {
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    ip = ip.trim().parse().unwrap();

    let stream = match TcpStream::connect(&ip[..]) {
        Ok(stream) => {
            stream
        },
        Err(error) => {
            println!("Failed to connect: {}", error.kind());
            return;
        }
    };
    let stream_clone = match stream.try_clone() {
        Ok(stream) => {
            stream
        },
        Err(error) => {
            println!("Failed to clone stream: {}", error.kind());
            return;
        }
    };

    let (sender, receiver) = mpsc::channel();
    let (sinterw, rinterw) = mpsc::channel();
    let (sinterr, rinterr) = mpsc::channel();
    let read = move || {
        reading(stream_clone, rinterr);
    };
    let write = move || {
        writing(stream, receiver, rinterw);
    };
    let read_handle = thread::spawn(read);
    let write_handle = thread::spawn(write);
    
    let mut input = String::new();

    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            },
            Ok(_) => {
                let message = input.trim().to_string();
                if message.is_empty() {
                    continue;
                }
                if sender.send(message + "\n").is_err() {
                    break;
                }
            },
            Err(error) => {
                println!("Input error: {}", error.kind());
                break;
            }
        }
    }
    sinterr.send(true).unwrap();
    sinterw.send(true).unwrap();
    let _ = read_handle.join();
    let _ = write_handle.join();

    println!("Client work finished");
}

fn reading(mut stream: TcpStream, interruption: mpsc::Receiver<bool>) {
    let mut buffer = [0; 2048];
    loop {
        if let Ok(value) = interruption.try_recv() {
            if value == true {
                break;
            }
        }
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Connection lost");
                break;
            },
            Ok(buffer_size) => {
                let message = String::from_utf8(decryption(&buffer[..buffer_size])).unwrap();
                println!("{}", message);
            },
            Err(error) => {
                println!("Connection error: {}", error.kind());
                break;
            }
        }
    }
}

fn writing(mut stream: TcpStream, receiver: mpsc::Receiver<String>, interruption: mpsc::Receiver<bool>) {
    for message in receiver {
        if let Ok(value) = interruption.try_recv() {
            if value == true {
                break;
            }
        }
        if let Err(error) = stream.write(message.as_bytes()) {
            println!("Failed to send message due to: {}", error.kind());
            break;
        }
    }
}