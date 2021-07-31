mod token;
mod lexer;
mod counter;
mod process_commands;
mod parse;
use std::io::prelude::*;
use std::{io::{Read, Error}, net::{TcpListener, TcpStream}};
use counter::counter::counter::Counter;
use crate::parse::parse::parse_tokens;
use crate::process_commands::process_commands::process_commands as process;

fn main() {
    let global_state: Counter = Counter::new(0);
    let listen = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listen.incoming(){
        let stream = stream.unwrap();
        println!("Connection established!");
        handler(stream, global_state);
    }
}

fn handler(mut stream: TcpStream, counter: Counter){
    loop{
        let mut buffer = vec![0; 50];
        stream.read(&mut buffer).unwrap();
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        let request = String::from_utf8(buffer[..].to_vec()).unwrap();
        println!("{:?}", request.as_bytes());
        let commands = process::process_commands(request);
        let value = parse_tokens(&commands, &counter);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
}
