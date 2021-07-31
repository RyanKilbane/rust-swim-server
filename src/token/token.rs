pub mod token{
    use std::collections::HashMap;
    #[derive(Debug)]
    pub struct Token{
        pub token: String,
        pub literal: String
    }
    impl PartialEq for Token {
        fn eq(&self, other: &Self) -> bool{
            self.token == other.token && self.literal == other.literal
        }
        
    }

    // keywords
    pub const ILLEGAL: &str = "ILLEGAL";
    pub const COMMAND: &str = "COMMAND";
    pub const VALUE: &str = "VALUE";
    pub const GET: &str = "GET";
    pub const SET: &str = "SET";
    pub const INT: &str = "INT";
    pub const SUBSCRIBE: &str = "SUBSCRIBE";

    // delimiters
    pub const COLON: &str = ":";
    pub const L_BRACE: &str = "{";
    pub const R_BRACE: &str = "}";
    pub const COMMA: &str = ",";
    pub const EOF: &str = "EOF";

    pub fn ident_lookup(lookup: &str) -> String{
        let mut keyword_map: HashMap<&str, &str> = HashMap::new();
        keyword_map.insert("command", COMMAND);
        keyword_map.insert("value", VALUE);
        keyword_map.insert("get", GET);
        keyword_map.insert("set", SET);
        keyword_map.insert("subscribe", SUBSCRIBE);
        if keyword_map.contains_key(lookup){
            keyword_map.get(lookup).unwrap().to_owned().to_string()
        }
        else{
            ILLEGAL.to_owned()
        }

    }
}