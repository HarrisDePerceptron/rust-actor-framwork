pub mod Order {


use actix::{self, Actor, Context, Message, Handler, Recipient, AsyncContext, ActorContext};


pub struct OrderShippedMessage(pub usize);

pub struct ShipMessage(pub usize);


pub struct SubscribeMessage(pub Recipient<OrderShippedMessage>);




impl  Message for OrderShippedMessage {
    type Result = ();
}

impl Message for ShipMessage {
    type Result= ();
}

impl  Message for SubscribeMessage {
    type Result = ();

}



pub struct  OrderEventsActor{
    subscribers: Vec<Recipient<OrderShippedMessage>>

}


impl OrderEventsActor {
    pub fn new () -> Self {
        Self { subscribers: vec![] }
    }

    fn notify(&mut self, order_id: usize){  
        for s in &self.subscribers {
            s.do_send(OrderShippedMessage(order_id))
        }
    }
}




pub struct EmailSubscriberActor{

}

pub struct SmsSubsctiberActor {

}

impl  Actor for OrderEventsActor {
    type Context = Context<Self>;
}

impl  Actor for EmailSubscriberActor {
    type Context = Context<Self>;
}


impl  Actor for SmsSubsctiberActor {
    type Context = Context<Self>;
}



impl Handler<SubscribeMessage> for OrderEventsActor {
    type Result = ();


    fn handle(&mut self, msg: SubscribeMessage, ctx: &mut Self::Context) -> Self::Result {
        self.subscribers.push(msg.0);

    }
}

impl Handler<ShipMessage> for OrderEventsActor {
    type Result= ();

    fn handle(&mut self, msg: ShipMessage, ctx: &mut Self::Context) -> Self::Result {
        self.notify(msg.0);
        // System::current().stop();

    }
}

impl Handler<OrderShippedMessage> for EmailSubscriberActor {
    type Result = ();

    fn handle(&mut self, msg: OrderShippedMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("sending email to send order {}", msg.0);

    }
}


impl  Handler<OrderShippedMessage> for SmsSubsctiberActor {
    type Result = ();

    fn handle(&mut self, msg: OrderShippedMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("sending sms to {}", msg.0);
    }
}


pub struct MyActor {
    pub counter: usize
}

impl Actor for MyActor {
    type Context  = Context<Self>;
}


pub struct WhoAmI{}


impl Message for WhoAmI {
    type Result = Result<actix::Addr<MyActor>, ()>;
}


impl Handler<WhoAmI> for MyActor {
    type Result= Result<actix::Addr<MyActor>, ()>;

    fn handle(&mut self, msg: WhoAmI, ctx: &mut Self::Context) -> Self::Result {
        let addr = ctx.address();
        self.counter += 1;

        if self.counter > 5 {
            println!("shutting down context meow");
            ctx.stop();
        }
        Ok(addr)
    }
}

}