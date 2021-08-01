pub mod parse{
    use std::borrow::BorrowMut;
    use std::io::{Write};
    use std::{format};
    use std::net::TcpStream;
    use std::rc::Rc;
    use crate::token::token::token::*;
    use crate::counter::counter::counter::Counter;
    use std::cell::RefCell;

    use std::sync::{Arc, RwLock, Mutex};

    pub fn parse_tokens<'a>(commands: &Vec<Token>, counter: &Arc<RwLock<Counter>>, subs: &mut Arc<Mutex<Option<Vec<TcpStream>>>>, stream: &Rc<RefCell<TcpStream>>) -> Option<String>{
        // Token array should look something like [{, COMMAND, COLON, ACTION, COMMA, VALUE, COLON, INT, }]
        if commands[3].token == GET{
            let x = counter.clone();
            let val = x.try_read().unwrap();
            Some(format!("{{value: {}}}", &val.state))
        }
        else if commands[3].token == SET {
            let x = counter.clone();
            let mut val = x.try_write().unwrap();
            val.alter_state(commands[7].literal.parse::<i64>().unwrap());
            let message = format!("{{value: {}}}\n", &val.state);
            let refs = subs.lock().unwrap().take().unwrap();
            for mut stream in refs.into_iter(){
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
            let s = stream.clone().borrow().try_clone().unwrap();
            let mut refs = subs.lock().unwrap().take().unwrap();
            refs.push(s);
            println!("{:?}", subs);
            None
        }
    }
}