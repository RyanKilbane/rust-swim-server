mod token;
mod lexer;
mod counter;
mod process_commands;
mod parse;
mod subscribe;
use std::rc::Rc;
use std::io::prelude::*;
use std::cell::RefCell;
use std::thread::spawn;
use std::{io::{Read}, net::{TcpListener, TcpStream}};
use counter::counter::counter::Counter;
use crate::parse::parse::parse_tokens;
use crate::process_commands::process_commands::process_commands as process;

use std::sync::{RwLock, Arc, Mutex}
;
fn main() {
    let global_state: Arc<RwLock<Counter>> = Arc::new(RwLock::new(Counter::new(0)));
    // let subscribers: Rc<RefCell<Subscribers>> = Rc::new(RefCell::new(Subscribers::new()));
    let mut subscribers: Arc<Mutex<Option<Vec<TcpStream>>>> = Arc::new(Mutex::new(Some(Vec::new())));
    let listen = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listen.incoming(){
        println!("Connection established!");
        let y = global_state.clone();
        let mut subs = subscribers.clone();
        spawn( move ||{
                let stream: Rc<RefCell<TcpStream>> = Rc::new(RefCell::new(stream.unwrap()));
                loop{
                    let input_str = handler(&stream);
                    let commands = process::process_commands(input_str);
                    let value = match parse_tokens(&commands, &y, &mut subs, &stream){
                        Some(v) => writer(&stream, v),
                        None => writer(&stream, String::from("subscribed"))
                    };
                }
            }
        );
    }
}

fn handler(stream: &Rc<RefCell<TcpStream>>) -> String{
        let mut buffer = vec![0; 50];();
        let mut s = stream.borrow_mut();
        s.read(&mut buffer).unwrap();
        let request = String::from_utf8(buffer[..].to_vec()).unwrap();
        request
}

fn writer(stream: &Rc<RefCell<TcpStream>>, value: String){
    let mut s = stream.borrow_mut();
    s.write(value.as_bytes()).unwrap();
    s.flush().unwrap();

}


// fn handler(mut stream: &Rc<RefCell<TcpStream>>, counter: &RefCell<Counter>, sub: &Rc<RefCell<Subscribers>>){
//     loop{
//         let mut buffer = vec![0; 50];
//         let mut s = stream.borrow_mut();
//         s.read(&mut buffer).unwrap();
//         let request = String::from_utf8(buffer[..].to_vec()).unwrap();
//         let commands = process::process_commands(request);
//         let value = parse_tokens(&commands, counter, &sub, stream).unwrap();
//         s.write(value.as_bytes()).unwrap();
//         s.flush().unwrap();
//     }
    
// }
