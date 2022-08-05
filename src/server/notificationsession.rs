use std::time::{Duration, Instant};

use crate::server;
use crate::server::clientmessage::ClientMessage;
use crate::server::disconnect::Disconnect;
use crate::server::message::Message;
use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, WrapFuture};
use actix_web_actors::ws;

use crate::routes::notificationroutes::{Connect, NotificationServer};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct NotificationSession {
    pub id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub heart_beat: Instant,
    /// Notification Server
    pub notification_server: Addr<NotificationServer>,
}

impl Actor for NotificationSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.heart_beat(context);
        let addr = context.address();
        self.notification_server
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(context);
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<Message> for NotificationSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl NotificationSession {
    fn heart_beat(&self, context: &mut ws::WebsocketContext<Self>) {
        context.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heart_beat) > CLIENT_TIMEOUT {
                println!("Client heartbeat failed - disconnecting");

                act.notification_server.do_send(Disconnect { id: act.id });

                ctx.stop();

                return;
            }

            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for NotificationSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        println!("Websocket message: {:?}", msg);

        match msg {
            ws::Message::Text(text) => {
                let m = text.trim();

                self.notification_server.do_send(ClientMessage {
                    id: self.id,
                    msg: m.to_string(),
                });
            }
            ws::Message::Binary(_) => println!("Unexpected binary!"),
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Ping(msg) => {
                self.heart_beat = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.heart_beat = Instant::now();
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
