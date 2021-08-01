mod token;
mod lexer;
mod counter;
mod process_commands;
mod parse;
mod subscribe;
mod exceptions;


use std::rc::Rc;
use std::io::prelude::*;
use std::cell::RefCell;
use std::thread::{self};
use std::{io::{Read}, net::{TcpListener, TcpStream}};
use counter::counter::counter::Counter;
use crate::parse::parse::parse_tokens;
use process_commands::process_commands::process_commands::*;

use std::sync::{RwLock, Arc, Mutex};
type Subs = Arc<Mutex<Vec<TcpStream>>>;
type MutCount = Arc<RwLock<Counter>>;
fn main() {
    let global_state: Arc<RwLock<Counter>> = Arc::new(RwLock::new(Counter::new(0)));
    // let subscribers: Rc<RefCell<Subscribers>> = Rc::new(RefCell::new(Subscribers::new()));
    let subscribers: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
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
        let mut mutable_stream: Rc<RefCell<&TcpStream>> = Rc::new(RefCell::new(stream));
        let input_stream = handler(&stream);
        let commands = process_commands(input_stream);
        match parse_tokens(&commands, counter, &mut subs, stream){
            Err(e) => writer(&mut mutable_stream, e.to_string()),
            Ok(val) => match val{
                Some(v) => writer(&mut mutable_stream, v),
                None => writer(&mut mutable_stream, String::from("subscribed"))
            }
        };
    }
}

fn handler(stream: &TcpStream) -> String{
        let mut buffer = vec![0; 100];();
        let mut s = stream;
        s.read(&mut buffer).unwrap();
        let request = String::from_utf8(buffer[..].to_vec()).unwrap();
        request
}

fn writer(stream: &mut Rc<RefCell<&TcpStream>>, value: String){
    stream.try_borrow_mut().unwrap().write(value.as_bytes()).unwrap();
    stream.try_borrow_mut().unwrap().flush().unwrap();
}
