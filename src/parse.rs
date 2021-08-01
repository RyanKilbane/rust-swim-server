pub mod parse{
    use std::io::{Write};
    use std::{format};
    use std::net::TcpStream;
    use std::rc::Rc;
    use crate::token::token::token::*;
    use crate::counter::counter::counter::Counter;
    use std::cell::RefCell;

    pub fn parse_tokens<'a>(commands: &Vec<Token>, counter: &RefCell<Counter>, subs: &mut Vec<TcpStream>, stream: &Rc<RefCell<TcpStream>>) -> Option<String>{
        // Token array should look something like [{, COMMAND, COLON, ACTION, COMMA, VALUE, COLON, INT, }]
        if commands[3].token == GET{
            Some(format!("{{value: {}}}", counter.borrow().state))
        }
        else if commands[3].token == SET {
            counter.borrow_mut().alter_state(commands[7].literal.parse::<i64>().unwrap());
            let message = format!("{{value: {}}}\n", counter.borrow().state);
            // let refs = subs.borrow_mut();
            for stream in subs.into_iter(){
                println!("Writing");
                println!("{:?}", stream);
                println!("{}", message);
                stream.write(message.as_bytes()).unwrap();
                stream.flush().unwrap();

            }
            Some(message)
        }
        else{
            // let mut refs = subs.borrow_mut();
            let s = stream.borrow_mut();
            subs.push(s.try_clone().unwrap());
            println!("{:?}", subs);
            None
        }
    }
}