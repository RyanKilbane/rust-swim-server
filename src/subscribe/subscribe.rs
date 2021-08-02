pub mod subscribe{
    use std::{io::Write, net::TcpStream};
    use std::sync::{Arc, Mutex};

    type Subs = Arc<Mutex<Vec<TcpStream>>>;

    #[derive(Clone, Debug)]
    pub struct Subscribers{
        subscribers: Subs
    }

    impl SubsTrait for Subscribers{
        fn new() -> Self{
            Subscribers{
                subscribers: Arc::new(Mutex::new(Vec::new()))
            }
        }

        fn add_subscriber(&mut self, sub: TcpStream){
            let mut refs = self.subscribers.lock().unwrap();
            refs.push(sub);
        }

        fn send_message(self, message: &str){
            let refs = self.subscribers.lock().unwrap();
            for mut stream in refs.iter(){
                stream.write(message.as_bytes()).unwrap();
            }
        }
    }
    pub trait SubsTrait {
        fn new() -> Self;
        fn add_subscriber(&mut self, sub: TcpStream);
        fn send_message(self, message: &str);
    }
}