//-------------------------------------------------------------------
// @author yangcancai
// Copyright (c) 2021 by yangcancai(yangcancai0112@gmail.com), All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// @doc
//
// @end
// Created : 2021-10-21T03:08:56+00:00
//-------------------------------------------------------------------
use crate::client::ChatClient;
use crate::client::ClientCommand;
use crate::client::HookCallback;
use crate::client::HookStart;
use crate::client::HookStop;
use crate::client::WsMsg;
use actix::io::SinkWrite;
use actix::Actor;
use actix::Arbiter;
use actix::StreamHandler;
use actix::System;
use awc::ws::Message;
use futures::StreamExt;
use serde_json::json;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::thread::JoinHandle;
type Pid = Sender<Reply>;
pub type WsResult = Result<(), WsError>;
pub enum Call {
    Ping(Pid),
    Close(Pid),
    WsMsg(WsMsg),
    WsMsgCall(Pid, WsMsg),
}
pub enum WsError {
    NonSender,
    SenderError(std::sync::mpsc::SendError<Call>),
}
pub enum Reply {
    Ok,
    Pong,
    Value(serde_json::Value),
}
pub struct Ws {
    sender: Option<Sender<Call>>,
    pub url: &'static str,
    handle: Option<JoinHandle<()>>,
    hook_start: Option<HookStart>,
    hook_stop: Option<HookStop>,
    hook_callback: Option<HookCallback>,
}
fn block_on(
    url: &'static str,
    rx: std::sync::mpsc::Receiver<Call>,
    hook_start: Option<HookStart>,
    hook_stop: Option<HookStop>,
    hook_callback: Option<HookCallback>,
) {
    let sys = System::new("websocket-client");
    Arbiter::spawn(async move {
        let client = awc::Client::builder()
            .max_http_version(awc::http::Version::HTTP_11)
            .finish();
        if let Ok((response, framed)) = client.ws(url).connect().await.map_err(|e| {
            println!("error {}", e);
        }) {
            println!("{:?}", response);
            let (sink, stream) = framed.split();
            let addr = ChatClient::create(|ctx| {
                ChatClient::add_stream(stream, ctx);
                ChatClient(
                    SinkWrite::new(sink, ctx),
                    hook_start,
                    hook_stop,
                    hook_callback,
                    json!({}),
                )
            });
            std::thread::spawn(move || loop {
                match rx.try_recv() {
                    Err(TryRecvError::Empty) => {
                        continue;
                    }
                    Err(_e) => {
                        addr.do_send(ClientCommand(WsMsg::Close));
                        return;
                    }
                    Ok(Call::Close(pid)) => {
                        let _ = pid.send(Reply::Ok);
                        addr.do_send(ClientCommand(WsMsg::Close));
                        return;
                    }
                    Ok(Call::Ping(pid)) => {
                        let _ = pid.send(Reply::Pong);
                    }
                    Ok(Call::WsMsgCall(pid, wsg)) => {
                        if let Ok(Some(reply)) =
                            futures::executor::block_on(addr.send(ClientCommand(wsg)))
                        {
                            let _ = pid.send(Reply::Value(reply));
                        }
                    }
                    Ok(Call::WsMsg(wsmsg)) => {
                        addr.do_send(ClientCommand(wsmsg));
                    }
                }
                thread::sleep(std::time::Duration::from_nanos(1));
            });
        } else {
            drop(rx);
            System::current().stop();
        }
    });
    sys.run().unwrap();
}

impl Ws {
    pub fn add_hook(&mut self, c: HookCallback) -> WsResult {
        self.hook_callback = Some(c);
        self.do_send_wsmsg(WsMsg::Hook(c))
    }
    pub fn add_hook_start(&mut self, c: HookStart) -> WsResult {
        self.hook_start = Some(c);
        self.do_send_wsmsg(WsMsg::HookStart(c))
    }
    pub fn add_hook_stop(&mut self, c: HookStop) -> WsResult {
        self.hook_stop = Some(c);
        self.do_send_wsmsg(WsMsg::HookStop(c))
    }
    pub fn state(&mut self) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert("url".into(), String::from(self.url));
        h.insert("is_alive".into(), self.is_alive().to_string());
        h
    }
    pub fn new(url: &'static str) -> Self {
        Ws {
            sender: None,
            url,
            handle: None,
            hook_start: None,
            hook_stop: None,
            hook_callback: None,
        }
    }
    pub fn connect(&mut self) {
        let (tx, rx) = channel();
        let url = self.url;
        let s = self.hook_start;
        let stop = self.hook_stop;
        let h = self.hook_callback;
        let handle = std::thread::spawn(move || {
            block_on(url, rx, s, stop, h);
        });
        self.sender = Some(tx);
        self.handle = Some(handle);
    }

    pub fn is_alive(&mut self) -> bool {
        self.ping()
    }

    fn ping(&mut self) -> bool {
        match self.call(|pid| Call::Ping(pid)) {
            Ok(Reply::Pong) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
    pub fn get_data(&mut self, key: &str) -> serde_json::Value {
        match self.call(|pid| Call::WsMsgCall(pid, WsMsg::Get(key.to_string()))) {
            Ok(Reply::Value(v)) => v,
            _ => json!(null),
        }
    }
    fn call<F>(&mut self, hook: F) -> Result<Reply, ()>
    where
        F: FnOnce(Pid) -> Call,
    {
        let (pid, rx) = channel();
        if let Some(send) = &self.sender {
            if let Ok(()) = send.send(hook(pid)) {
                match rx.recv() {
                    Ok(reply) => {
                        return Ok(reply);
                    }
                    _ => {
                        return Err(());
                    }
                }
            } else {
                return Err(());
            }
        } else {
            return Err(());
        }
    }
    pub fn send_msg(&mut self, msg: &str) -> WsResult {
        self.send(Message::Text(msg.to_string()))
    }
    fn send(&mut self, msg: Message) -> WsResult {
        self.do_send_wsmsg(WsMsg::Message(msg))
    }
    fn do_send_wsmsg(&mut self, msg: WsMsg) -> WsResult {
        self.do_send(Call::WsMsg(msg))
    }
    fn do_send(&mut self, msg: Call) -> WsResult {
        if let Some(send) = &self.sender {
            match send.send(msg) {
                Ok(()) => Ok(()),
                Err(e) => Err(WsError::SenderError(e)),
            }
        } else {
            Err(WsError::NonSender)
        }
    }
    pub fn close(&mut self) {
        let _ = self.call(|pid| Call::Close(pid));
        let _ = self.send(Message::Close(None));
        if let Some(thread) = self.handle.take() {
            if let Ok(_) = thread.join() {};
        }
    }
}
impl Drop for Ws {
    fn drop(&mut self) {
        self.close();
    }
}
