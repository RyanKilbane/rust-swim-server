pub mod parse{
    use std::{format};
    use std::net::TcpStream;
    use crate::{MutCount, Subs};
    use crate::token::token::token::*;
    use crate::subscribe::subscribe::subscribe::SubsTrait;
    use crate::exceptions::parse_error::parse_error::ParseError;

    pub fn parse_tokens<'a>(commands: &Vec<Token>, counter: &MutCount, subs: &mut Subs, stream: &TcpStream) -> Result<Option<String>, ParseError>{
        // Token array should look something like [{, COMMAND, COLON, ACTION, COMMA, VALUE, COLON, INT, }]
        if commands[3].token == GET{
            let count = counter.lock().unwrap();
            Ok(Some(format!("{{value: {}}}", count.state)))
        }
        else if commands[3].token == SET {
            let mut count = counter.lock().unwrap();

            let value = match commands.get(7){
                Some(val) => match val.literal.parse::<i64>(){
                    Ok(value) => value,
                    Err(_e) => return Err(ParseError::new("An integer was expected, ensure you command followed the form: {command: action, value: integer}"))
                },
                None => return Err(ParseError::new("Miss match between expected and actual length"))
            };

            count.alter_state(value);
            let message = format!("{{value: {}}}", count.state);
            let x = subs.lock().unwrap().clone();
            x.send_message(&message);
            Ok(Some(message))
        }
        else{
            let stream_clone = TcpStream::try_clone(stream).unwrap();
            let mut x = subs.lock().unwrap();
            x.add_subscriber(stream_clone);
            println!("{:?}", subs);
            Ok(None)
        }
    }
}