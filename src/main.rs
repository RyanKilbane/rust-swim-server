mod token;
mod lexer;
mod counter;
mod process_commands;
mod parse;
mod subscribe;
mod exceptions;


use std::io::prelude::*;
use std::cell::RefCell;
use std::thread::{self};
use std::{io::{Read}, net::{TcpListener, TcpStream, Shutdown}};
use counter::counter::counter::Counter;
use token::token::token::Token;
use crate::parse::parse::parse_tokens;
use crate::subscribe::subscribe::subscribe::{Subscribers};
use process_commands::process_commands::process_commands::*;

use std::sync::{Arc, Mutex};
type Subs = Arc<Mutex<Subscribers>>;
type MutCount = Arc<Mutex<Counter>>;

fn main() {
    let global_state: MutCount = Arc::new(Mutex::new(Counter::new(0)));
    // let subscribers: Rc<RefCell<Subscribers>> = Rc::new(RefCell::new(Subscribers::new()));
    let subscribers: Subs = Arc::new(Mutex::new(Subscribers::new()));
    let listen = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listen.incoming(){
        let y = global_state.clone();
        let mut subs = subscribers.clone();
        thread::spawn(move || handle_connection(&stream.unwrap(), &mut subs, &y));
    }
}


fn handle_connection(stream: &TcpStream, mut subs: &mut Subs, counter: &MutCount){
    println!("Connected!");
    loop{
        let mut mutable_stream: RefCell<&TcpStream> = RefCell::new(stream);
        let input_stream = handler(&stream);
        let commands = process_commands(input_stream);
        if commands.len() == 0{
            println!("shutting down");
            stream.shutdown(Shutdown::Both).unwrap();
        }
        match scan_for_illegal(&commands){
            true => {writer(&mut mutable_stream, String::from("There was an illegal token in your command"))},
            false => {
                match parse_tokens(&commands, counter, &mut subs, stream){
                    Err(e) => writer(&mut mutable_stream, e.to_string()),
                    Ok(val) => match val{
                        Some(v) => writer(&mut mutable_stream, v),
                        None => writer(&mut mutable_stream, String::from("subscribed"))
                    }
                }
            }
        }
    }
}

fn handler(stream: &TcpStream) -> String{
        let mut buffer = vec![0; 100];();
        let mut s = stream;
        s.read(&mut buffer).unwrap();
        let request = String::from_utf8(buffer[..].to_vec()).unwrap();
        request.chars().filter(|char| !char.is_whitespace()).collect()
}

fn writer(stream: &mut RefCell<&TcpStream>, value: String){
    stream.try_borrow_mut().unwrap().write(value.as_bytes()).unwrap();
    stream.try_borrow_mut().unwrap().flush().unwrap();
}

fn scan_for_illegal(commands: &Vec<Token>) -> bool{
    for token in commands.into_iter(){
        println!("{}", token.literal);
        if token.token == "ILLEGAL"{
            return true
        }
        else {
            continue;
        }
    }
    return false
}