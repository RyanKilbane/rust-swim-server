mod token;
mod lexer;
mod counter;
mod process_commands;
mod parse;
mod subscribe;
use std::rc::Rc;
use std::io::prelude::*;
use std::cell::RefCell;
use std::{io::{Read}, net::{TcpListener, TcpStream}};
use counter::counter::counter::Counter;
use subscribe::subscribe::subscribe::Subscribers;
use crate::parse::parse::parse_tokens;
use crate::process_commands::process_commands::process_commands as process;

fn main() {
    let global_state: RefCell<Counter> = RefCell::new(Counter::new(0));
    let subscribers: Rc<RefCell<Subscribers>> = Rc::new(RefCell::new(Subscribers::new()));
    let listen = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listen.incoming(){
        let stream: Rc<RefCell<TcpStream>> = Rc::new(RefCell::new(stream.unwrap()));
        loop{
            println!("Connection established!");
            let input_str = handler(&stream);
            let commands = process::process_commands(input_str);
            let value = match parse_tokens(&commands, &global_state, &subscribers, &stream){
                Some(v) => writer(&stream, v),
                None => writer(&stream, String::from("subscribed"))
            };
        }
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
