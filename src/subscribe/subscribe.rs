pub mod subscribe{
    use std::{io::Write, net::TcpStream, ops::Sub};
    pub struct Subscribers{
        subscribers: Vec<TcpStream>
    }

    impl Subscribers{
        pub fn new() -> Self{
            Subscribers{
                subscribers: Vec::new()
            }
        }

        pub fn add_subscriber(&mut self, sub: TcpStream){
            self.subscribers.push(sub);
        }

        pub fn send_message(self, message: &str){
            for mut stream in self.subscribers.into_iter(){
                stream.write(message.as_bytes()).unwrap();
            }
        }
    }
}