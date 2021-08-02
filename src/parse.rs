pub mod parse{
    use std::io::{Write};
    use std::{format};
    use std::net::TcpStream;
    use crate::{MutCount, Subs};
    use crate::token::token::token::*;
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
            let sub = subs.lock().unwrap();
            let x = sub.as_slice();
            println!("{:?}", x);
            for mut stream in x.into_iter(){
                stream.flush().unwrap();
                println!("Writing");
                stream.write_all(message.as_bytes()).unwrap();
                stream.flush().unwrap();

            }
            Ok(Some(message))
        }
        else{
            let mut sub = subs.lock().unwrap();
            sub.push(TcpStream::try_clone(stream).unwrap());
            println!("{:?}", subs);
            Ok(None)
        }
    }

    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn test_parse_token(){

        }
    }
}