

pub mod Game {
    use actix::{self, Actor, Context, Handler, Message, Recipient};
    use actix::dev::{MessageResponse, OneshotSender};


    struct Game {
        counter: usize,
        name: String,
        recepient: Option<Recipient<Ping>>
    
    }
    
    
    struct Ping {
        pub id: usize
    }
    
    
    impl  Message for Ping {
        type Result = Ping;
    }
    
    
    impl Actor for Game {
        type Context = Context<Self>;
    
    }
    
    impl Handler<Ping> for Game {
        type Result = Ping;
    
        fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
            self.counter += 1;
    
    
            println!("Counter: {}", self.counter);
            Ping{id: 10}
        }
        
    }
    
    
    
    impl<A, M>  MessageResponse<A, M> for Ping 
    where 
        A: Actor,
        M: Message<Result = Ping>
    {
        fn handle(self, ctx: &mut <A as Actor>::Context, tx: Option<actix::dev::OneshotSender<<M as Message>::Result>>) {
            if let Some(tx) = tx {
                tx.send(self);
            }
        }
    }
}
