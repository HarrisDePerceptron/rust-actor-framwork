use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner,
    Handler, Message, Recipient, StreamHandler, WrapFuture,
};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

struct WebSocketSession {
    id: usize,
    server: Addr<WebSocketServer>,
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        self.server
            .send(Connect(addr))
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    Err(e) => ctx.stop(),
                }

                fut::ready(())
            })
            .wait(ctx);
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<Addr<WebSocketServer>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        WebSocketSession {
            server: state.get_ref().clone(),
            id: 0,
        },
        &req,
        stream,
    );
    println!("{:?}", resp);
    resp
}

struct WebSocketServer {
    index: usize,
    sessions: Vec<Addr<WebSocketSession>>,
}

struct TextMessage {
    message: String,
}
impl Message for TextMessage {
    type Result = ();
}

impl Handler<TextMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.message);
    }
}

impl Actor for WebSocketServer {
    type Context = Context<Self>;
}

impl WebSocketServer {
    pub fn notify_all(&self, message: &str) {
        for s in &self.sessions {
            let res: Recipient<TextMessage> = s.clone().recipient();
            res.do_send(TextMessage {
                message: message.to_owned(),
            });
        }
    }
}

struct Connect(Addr<WebSocketSession>);
impl Message for Connect {
    type Result = usize;
}

impl Handler<Connect> for WebSocketServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        println!("Connecting new websocket session...");
        self.sessions.push(msg.0);

        self.index += 1;

        return self.index;
    }
}

struct TextMessageAll {
    message: String,
}
impl Message for TextMessageAll {
    type Result = ();
}

impl Handler<TextMessageAll> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: TextMessageAll, ctx: &mut Self::Context) -> Self::Result {
        self.notify_all(&msg.message);
    }
}

#[get("/hi")]
async fn say_hi(state: web::Data<Addr<WebSocketServer>>) -> impl Responder {
    if let Err(e) = state
        .send(TextMessageAll {
            message: "hiya everyone".to_owned(),
        })
        .await
    {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().body("sendding hi to conencted sockets")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = WebSocketServer {
        index: 0,
        sessions: vec![],
    };

    let server_addr = server.start();

    let state = web::Data::new(server_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/ws/", web::get().to(index))
            .service(say_hi)
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}
