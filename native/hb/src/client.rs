use std::time::Duration;

use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;
use actix_web::client::WsProtocolError;
use awc::{
    ws::{Codec, Frame, Message},
    BoxedSocket,
};
use bytes::Bytes;
use futures::stream::SplitSink;

pub type HookCallback = fn(
    Result<Frame, WsProtocolError>,
    &mut Context<ChatClient>,
) -> Result<serde_json::Value, WsProtocolError>;

pub type HookStart = fn(&mut Context<ChatClient>);
pub type HookStop = fn(&mut Context<ChatClient>);
pub struct ChatClient(
    pub SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>,
    pub Option<HookStart>,
    pub Option<HookStop>,
    pub Option<HookCallback>,
    pub serde_json::Value,
);
pub enum WsMsg {
    Get(String),
    Message(Message),
    Hook(HookCallback),
    HookStart(HookStart),
    HookStop(HookStop),
    Close,
}
pub type ClientReply = Option<serde_json::Value>;
#[derive(Message)]
#[rtype(result = "ClientReply")]
pub struct ClientCommand(pub WsMsg);

impl Actor for ChatClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx);
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Disconnected");
        if let Some(hook) = self.2 {
            hook(ctx);
        }
        // Stop application on disconnect
        System::current().stop();
    }
}

impl ChatClient {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(10, 0), |act, ctx| {
            act.0.write(Message::Ping(Bytes::from_static(b"")));
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

/// Handle stdin commands
impl Handler<ClientCommand> for ChatClient {
    type Result = ClientReply;

    fn handle(&mut self, msg: ClientCommand, ctx: &mut Context<Self>) -> Self::Result {
        match msg.0 {
            WsMsg::Message(text) => {
                println!("sub...");
                self.0.write(text);
            }
            WsMsg::Hook(h) => {
                self.3 = Some(h);
            }
            WsMsg::HookStart(h) => {
                self.1 = Some(h);
            }
            WsMsg::HookStop(h) => {
                self.2 = Some(h);
            }
            WsMsg::Close => {
                self.stopped(ctx);
            }
            WsMsg::Get(key) => {
                println!("get...");
                if let Some(value) = self.4.get(key) {
                    return Some(value.clone());
                }
            }
        }
        Some(serde_json::Value::Null)
    }
}
/// Handle server websocket messages
impl StreamHandler<Result<Frame, WsProtocolError>> for ChatClient {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, ctx: &mut Context<Self>) {
        if let Some(hook) = self.3 {
            match hook(msg, ctx) {
                Ok(serde_json::Value::Null) => {}
                Ok(value) => {
                    if let Some(key) = value["id"].as_str() {
                        if let Some(obj) = self.4.get_mut(key) {
                            *obj = value["value"].clone();
                        } else if let Some(obj) = self.4.as_object_mut() {
                            let v = &value["value"];
                            obj.insert(key.into(), v.clone());
                        }
                    }
                }
                Err(_e) => {}
            }
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        if let Some(hook) = self.1 {
            hook(ctx);
        }
        println!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("Server disconnected");
        ctx.stop()
    }
}

impl actix::io::WriteHandler<WsProtocolError> for ChatClient {}
