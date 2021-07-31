pub mod parse{
    use std::format;
    use crate::token::token::token::*;
    use crate::counter::counter::counter::Counter;

    pub fn parse_tokens(commands: &Vec<Token>, counter: &Counter) -> Option<String>{
        // Token array should look something like [{, COMMAND, COLON, ACTION, COMMA, VALUE, COLON, INT, }]
        if commands[2].token == GET{
            Some(format!("value: {}", counter.state))
        }
        else if commands[2].token == SET {
            counter.alterState(commands[7].literal.parse::<i64>().unwrap());
            Some(format!("value: {}", counter.state))
        }
        else{
            None
        }
    }
}