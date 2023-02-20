

mod Calculator {
    use actix::{self, Actor, Context, Handler, Message};



    struct Sum(usize, usize);

    impl Message for Sum {
        type Result = usize;
    }

    struct Calculator {}

    impl Actor for Calculator {
        type Context = Context<Self>;

        fn started(&mut self, ctx: &mut Self::Context) {
            println!("Started the calculator actor");
        }
    }

    impl Handler<Sum> for Calculator {
        type Result = usize;

        fn handle(&mut self, msg: Sum, ctx: &mut Self::Context) -> Self::Result {
            msg.0 + msg.1
        }
    }
}
