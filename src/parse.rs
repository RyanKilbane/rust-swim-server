pub mod parse{
    use std::io::{Write};
    use std::{format};
    use std::net::TcpStream;
    use crate::{MutCount, Subs};
    use crate::token::token::token::*;

    pub fn parse_tokens<'a>(commands: &Vec<Token>, counter: &MutCount, subs: &mut Subs, stream: &TcpStream) -> Option<String>{
        // Token array should look something like [{, COMMAND, COLON, ACTION, COMMA, VALUE, COLON, INT, }]
        if commands[3].token == GET{
            let count = counter.try_read().unwrap();
            Some(format!("{{value: {}}}", count.state))
        }
        else if commands[3].token == SET {
            let mut count = counter.try_write().unwrap();
            count.alter_state(commands[7].literal.parse::<i64>().unwrap());
            let message = format!("{{value: {}}}", count.state);
            let sub = subs.lock().unwrap();
            let x = sub.as_slice();
            for mut stream in x.into_iter(){
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
            // let s = stream.clone().borrow().try_clone().unwrap();
            // let mut refs = subs.lock().unwrap().take().unwrap();
            let mut sub = subs.lock().unwrap();
            sub.push(TcpStream::try_clone(stream).unwrap());
            println!("{:?}", subs);
            None
        }
    }
}