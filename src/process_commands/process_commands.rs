pub mod process_commands{
    use crate::lexer::lexer::lexer::Lexer;
    use crate::token::token::token::*;

    pub fn process_commands(commands: String) -> Vec<Token>{
        println!("{}", commands);
        let mut command_array: Vec<Token> = Vec::new();
        let mut lexer = Lexer::new(&commands);
        loop{
            let tok = lexer.next_token();
            println!("{}", tok.token);
            if tok.token == "EOF"{
                break
            }
            command_array.push(tok);
        };
        command_array
    }

    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn test_process_commands(){
            let command = "{command:get}";
            let commands = process_commands(String::from(command));
            let mut expected: Vec<Token> = Vec::new();
            expected.push(Token{token: L_BRACE.to_string(), literal: "{".to_owned()});
            expected.push(Token{token: COMMAND.to_string(), literal: "command".to_owned()});
            expected.push(Token{token: COLON.to_string(), literal: ":".to_owned()});
            expected.push(Token{token: GET.to_string(), literal: "get".to_owned()});
            expected.push(Token{token: R_BRACE.to_string(), literal: "}".to_owned()});
            assert_eq!(expected, commands);

        }
    }
}