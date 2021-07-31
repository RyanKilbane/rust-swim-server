mod token;
mod lexer;
mod counter;
mod process_commands;
mod parse;
use std::io::prelude::*;
use std::cell::RefCell;
use std::{io::{Read, Error}, net::{TcpListener, TcpStream}};
use counter::counter::counter::Counter;
use crate::parse::parse::parse_tokens;
use crate::process_commands::process_commands::process_commands as process;

fn main() {
    let mut global_state: RefCell<Counter> = RefCell::new(Counter::new(0));
    let listen = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listen.incoming(){
        let stream = stream.unwrap();
        println!("Connection established!");
        handler(stream, &global_state);
    }
}

fn handler(mut stream: TcpStream, counter: &RefCell<Counter>){
    loop{
        let mut buffer = vec![0; 50];
        stream.read(&mut buffer).unwrap();
        let request = String::from_utf8(buffer[..].to_vec()).unwrap();
        println!("{:?}", request.as_bytes());
        let commands = process::process_commands(request);
        let value = parse_tokens(&commands, counter).unwrap();
        stream.write(value.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
}
