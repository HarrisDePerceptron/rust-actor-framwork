use actor_framework::order::Order::*;
use actix::{Actor};

#[actix::main]
async fn main() {
    println!("Hello, world!");

    let order_event = OrderEventsActor::new().start();
    let email_sub = EmailSubscriberActor{}.start();
    let sms_sub  = SmsSubsctiberActor{}.start();


    let email_subs = SubscribeMessage(email_sub.clone().recipient());

    let sms_subs = SubscribeMessage(sms_sub.clone().recipient());



    order_event.send(email_subs).await.unwrap();
    order_event.send(sms_subs).await.unwrap();

    let o1 = order_event.send(ShipMessage(1));

    let o2 = order_event.send(ShipMessage(2));


    o1.await.unwrap();
    o2.await.unwrap();


    order_event.send(ShipMessage(3)).await.unwrap();


    let myaddr = MyActor{counter: 0}.start();

    
    while let Ok(Ok(whoami_res)) =  myaddr.send(WhoAmI { }).await {
        println!("got the address from actor");
    }

}

