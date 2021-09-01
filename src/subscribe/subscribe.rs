pub mod subscribe{
    use std::fmt::Debug;
    use std::{io::Write, net::TcpStream};
    use std::sync::{Arc, Mutex};

    type Subs = Arc<Mutex<Vec<TcpStream>>>;

    #[derive(Clone, Debug)]
    pub struct Subscribers{
        subscribers: Subs
    }

    impl Subscribers{
        pub fn new() -> Self{
            Subscribers{
                subscribers: Arc::new(Mutex::new(Vec::new()))
            }
        }
    }

    impl SubsTrait for Subscribers{

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

        fn unsubscribe(self, sub: &TcpStream) {
            let mut refs = self.subscribers.lock().unwrap();
            let loc = refs.iter().position(|x| x.peer_addr().unwrap() == sub.peer_addr().unwrap()).unwrap();
            refs.remove(loc);
            
        }
    }
    pub trait SubsTrait: Clone + Debug {
        fn add_subscriber(&mut self, sub: TcpStream);
        fn send_message(self, message: &str);
        fn unsubscribe(self, sub: &TcpStream);
    }
}