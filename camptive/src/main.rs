extern crate actix;
extern crate actix_web;
use actix::{Actor, ActorContext, AsyncContext, Running, StreamHandler};
use actix_web::{http, server, ws, App, Error, HttpRequest, HttpResponse};

use std::time::{Instant, Duration};


fn hello(_req: &HttpRequest) -> &'static str {
    "{\"message\": \"Hello, world!\"}"
}

fn ws_echo(r: &HttpRequest) -> Result<HttpResponse, Error> {
    ws::start(r, Ws::new())
}

struct Ws {
    hb: Instant,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started websocket");
        self.hb(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("Stopped websocket");
        Running::Stop
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("Handling: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
        }
    }
}

const HEARTBEAT: Duration = Duration::from_secs(10);
const TIMEOUT: Duration = Duration::from_secs(60);

impl Ws {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT, |act, ctx| {
            if Instant::now().duration_since(act.hb) > TIMEOUT {
                println!("No heartbeat. Closing.");
                ctx.stop();
                return;
            }
            ctx.ping("");
        });
    }
}

fn main() {
    let address = "127.0.0.1:8888";
    println!("Server running on http://{}", address);
    server::new(|| App::new()
        .resource("/", |r| r.f(hello))
        .resource("/ws/", |r| r.method(http::Method::GET).f(ws_echo)))
        .bind(address)
        .unwrap()
        .run();
    println!("Shutting down...");
}
