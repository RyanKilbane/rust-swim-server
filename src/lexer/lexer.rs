pub mod lexer{

    use crate::token::token::token::{*, self};

    pub struct Lexer<'a>{
        input: Option<&'a str>,
        position: Option<u8>,
        read_position: Option<u8>,
        chr: Option<u8>
    }

    impl<'a>Lexer<'a>{
        pub fn new(input: &'a str) -> Self{
            Lexer{
                input: Some(input),
                position: Some(0),
                read_position: Some(1),
                chr: Some(input.as_bytes().to_owned()[0])
            }
        }

        pub fn read_char(&mut self){
            if self.read_position.unwrap() >= self.input.unwrap().len() as u8{
                self.chr = Some(0);
            }
            else{
                self.chr = Some(self.input.unwrap().as_bytes()[self.read_position.unwrap() as usize]);
            }
            self.position = self.read_position;
            self.read_position = Some(self.read_position.unwrap() + 1);
        }

        pub fn _skip_whitespace(&mut self){
            if self.chr.unwrap() == 32 || self.chr.unwrap() == 10 || self.chr.unwrap() == 13 || self.chr.unwrap() == 9{
                self.read_char()
            }
        }

        fn read_ident(&mut self) -> String{
            let position = self.position.unwrap() as usize;
            while Lexer::is_letter(&self.chr.unwrap()){
                self.read_char()
            }
            let slice = &self.input.unwrap().as_bytes()[position..self.position.unwrap() as usize];
            let vector = slice.to_vec();
            String::from_utf8(vector).unwrap()
        }

        pub fn read_number(&mut self) -> String{
            let position = self.position.unwrap() as usize;
            while Lexer::is_number(&self.chr.unwrap()){
                self.read_char()
            }
            let slice = &self.input.unwrap().as_bytes()[position..self.position.unwrap() as usize];
            let vector = slice.to_vec();
            String::from_utf8(vector).unwrap()
        }

        fn is_letter(token: &u8) -> bool{
            let x = if token < &91 && token > &64 || token == &95{ 
                true
            }
            else if token > &96 && token < &123{
                true
            }
            else{
                false
            };
            
            x
        }

        fn is_number(token: &u8) -> bool{
            let x = if token < &57 && token > &47{ 
                true
            }
            else{
                false
            };
            x
        }

        pub fn next_token(&mut self) -> Token{
            let current_token = self.chr.unwrap();
            let token_as_str = std::str::from_utf8(&[current_token]).unwrap().to_owned();
            let matched_token = match &*token_as_str{
                "{" => {
                    self.read_char();
                    Token{
                        token: L_BRACE.to_owned(),
                        literal: String::from(token_as_str)
                    }
                },
                "}" => {
                    self.read_char();
                    Token{
                        token: R_BRACE.to_owned(),
                        literal: String::from(token_as_str)
                    }
                },
                ":" => {
                    self.read_char();
                    Token{
                        token: COLON.to_owned(),
                        literal: String::from(token_as_str)
                    }
                },
                "," => {
                    self.read_char();
                    Token{
                        token: COMMA.to_owned(),
                        literal: String::from(token_as_str)
                    }
                },
                "\u{0}" => Token{
                    token: EOF.to_owned(),
                    literal: String::from(token_as_str)
                },
                _ => {
                    if Lexer::is_letter(&current_token){
                        let x: String = self.read_ident();
                        let ident = token::ident_lookup(&x);
                        Token{
                            token: ident,
                            literal: x
                        }
                    }
                    else if Lexer::is_number(&current_token) {
                        Token{
                            token: INT.to_owned(),
                            literal: self.read_number()
                        }
                    }
                    else{
                        self.read_char();
                        Token{
                            token: ILLEGAL.to_owned(),
                            literal: String::from(token_as_str)
                        }
                    }
                }
            };
            matched_token
        }
    }

    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn test_next_token(){
            let commands = "{command:get}";
            let mut l = Lexer::new(commands);
            assert_eq!(l.next_token().literal, "{");
            assert_eq!(l.next_token().token, "COMMAND");
            assert_eq!(l.next_token().token, ":");
            assert_eq!(l.next_token().token, "GET");
            assert_eq!(l.next_token().token, "}");

        }

    }
}