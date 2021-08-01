pub mod parse_error{
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub struct ParseError{
        details: String
    }

    impl ParseError {
        pub fn new(msg: &str) -> Self{
            ParseError{details: msg.to_owned()}
        }
    }

    impl fmt::Display for ParseError{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "{}", self.details)
        }
    }

    impl Error for ParseError{
        fn description(&self) -> &str{
            &self.details
        }
    }
}