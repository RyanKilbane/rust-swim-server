pub mod subscribe{
    use std::{io::Write, net::TcpStream};
    use std::cell::RefCell;
    pub struct Subscribers{
        subscribers: RefCell<Vec<TcpStream>>
    }

    impl Subscribers{
        pub fn new() -> Self{
            Subscribers{
                subscribers: RefCell::new(Vec::new())
            }
        }

        pub fn add_subscriber(&mut self, sub: TcpStream){
            let mut refs = self.subscribers.borrow_mut();
            refs.push(sub);
        }

        pub fn send_message(self, message: &str){
            let mut refs = self.subscribers.borrow_mut();
            for stream in refs.iter_mut(){
                stream.write(message.as_bytes()).unwrap();
            }
        }
    }
}