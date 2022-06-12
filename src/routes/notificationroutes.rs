use std::time::Instant;

use actix::{Actor, StreamHandler, Addr, Recipient};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use rand::{self, rngs::ThreadRng, Rng};
use actix::prelude::*;
use crate::server::{message::Message, disconnect::Disconnect, clientmessage::ClientMessage, notificationsession::NotificationSession};

#[derive(Message)] //makes 'Connect' impl Message
#[rtype(usize)] //designates the return type of it's 'Handler' impl will be
pub struct Connect {
    pub addr: Recipient<Message>,
}



pub struct NotificationServer {
    client: Recipient<Message>,
    rng: ThreadRng,
}

impl NotificationServer {
    pub fn send_notification(&self, message: &str) {
        self.client.do_send(Message(message.to_owned()));
    }
}

impl Actor for NotificationServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for NotificationServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        println!("Someone connected");
        self.send_notification("Client connected!");
        let id = self.rng.gen::<usize>();
        id
    }
}

impl Handler<Disconnect> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("The client disconnected");
    }
}

impl Handler<ClientMessage> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_notification(msg.msg.as_str());
    }
}

pub async fn create_notification_connection(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<NotificationServer>>,
) -> Result<HttpResponse, Error> {
    // let resp = ws::start(MyWs {}, &req, stream);
    // println!("{:?}", resp);
    // resp
    ws::start(
        NotificationSession {
            id: 0,
            heart_beat: Instant::now(),
            notification_server: srv.get_ref().clone(),

        },
        &req,
        stream,
    )
}
