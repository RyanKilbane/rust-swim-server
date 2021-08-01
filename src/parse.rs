pub mod parse{
    use std::borrow::Borrow;
    use std::io::Read;
    use std::{format, str};
    use std::net::TcpStream;
    use crate::subscribe::subscribe::subscribe::Subscribers;
    use crate::token::token::token::*;
    use crate::counter::counter::counter::Counter;
    use std::cell::RefCell;

    pub fn parse_tokens(commands: &Vec<Token>, counter: &RefCell<Counter>, subs: &RefCell<Subscribers>) -> Option<String>{
        // Token array should look something like [{, COMMAND, COLON, ACTION, COMMA, VALUE, COLON, INT, }]
        if commands[3].token == GET{
            Some(format!("{{value: {}}}", counter.borrow().state))
        }
        else if commands[3].token == SET {
            counter.borrow_mut().alter_state(commands[7].literal.parse::<i64>().unwrap());
            let message = format!("{{value: {}}}", counter.borrow().state);
            // subs.clone().send_message(&message);
            Some(message)
        }
        else{
            // let x = stream.borrow();
            //subs.borrow_mut().add_subscriber(x.try_clone().unwrap());
            None
        }
    }

    #[cfg(test)]
    mod test{

        use crate::process_commands::process_commands::process_commands::process_commands;

        use super::*;
        #[test]
        fn test_parse(){
            let subs = RefCell::new(Subscribers::new());
            let stream = TcpStream::connect("127.0.0.1");
            let counter: RefCell<Counter> = RefCell::new(Counter::new(10));
            let command = "{command:get}";
            let commands = process_commands(String::from(command));
            let x = parse_tokens(&commands, &counter, &subs);
            assert_eq!(x.unwrap(), "{value: 10}");
        }
    }
}