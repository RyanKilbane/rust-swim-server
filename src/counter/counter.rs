pub mod counter{
    #[derive(Clone)]
    pub struct Counter{
        pub state: i64
    }
impl Counter{
        pub fn new(inital_state: i64) -> Self {
            Counter{
                state: inital_state
            }
        }

        pub fn alter_state(&mut self, new_state: i64){
            self.state = new_state;
        }
    }

    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn test_increment(){
            let mut counter = Counter::new(10);
            counter.alter_state(123);
            assert_eq!(counter.state, 123);
        }
    }

}