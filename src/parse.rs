pub mod parse{
    use std::format;
    use crate::token::token::token::*;
    use crate::counter::counter::counter::Counter;
    use std::cell::RefCell;

    pub fn parse_tokens(commands: &Vec<Token>, counter: &RefCell<Counter>) -> Option<String>{
        // Token array should look something like [{, COMMAND, COLON, ACTION, COMMA, VALUE, COLON, INT, }]
        if commands[3].token == GET{
            Some(format!("{{value: {}}}", counter.borrow().state))
        }
        else if commands[3].token == SET {
            counter.borrow_mut().alterState(commands[7].literal.parse::<i64>().unwrap());
            Some(format!("{{value: {}}}", counter.borrow().state))
        }
        else{
            None
        }
    }

    #[cfg(test)]
    mod test{
        use std::process;

        use crate::{lexer::lexer::lexer::Lexer, process_commands::process_commands::process_commands::process_commands};

        use super::*;
        #[test]
        fn test_parse(){
            let counter: RefCell<Counter> = RefCell::new(Counter::new(10));
            let command = "{command:get}";
            let commands = process_commands(String::from(command));
            let x = parse_tokens(&commands, &counter);
            assert_eq!(x.unwrap(), "{value: 10}");
        }
    }
}