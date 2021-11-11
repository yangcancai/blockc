#![allow(clippy::missing_safety_doc, clippy::not_unsafe_ptr_arg_deref)]

use actix::*;
use actix_web::client::WsProtocolError;
use actix_web_actors::ws::Frame;
use allo_isolate::Isolate;
use atomic::Atomic;
use awc::ws::Message;
use ffi_helpers::null_pointer_check;
use hb::client::*;
use hb::ws::Ws;
use lazy_static::lazy_static;
use serde_json::json;
use std::slice;
use std::sync::atomic::Ordering;
use std::{ffi::CStr, io, os::raw};
use tokio::runtime::{Builder, Runtime};

static mut PORT_COBJECT: Atomic<Option<i64>> = Atomic::new(None);

lazy_static! {
    static ref RUNTIME: io::Result<Runtime> = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .core_threads(4)
        .thread_name("flutterust")
        .build();
}

macro_rules! error {
    ($result:expr) => {
        error!($result, 0);
    };
    ($result:expr, $error:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                ffi_helpers::update_last_error(e);
                return $error;
            }
        }
    };
}

macro_rules! cstr {
    ($ptr:expr) => {
        cstr!($ptr, 0);
    };
    ($ptr:expr, $error:expr) => {{
        null_pointer_check!($ptr);
        error!(unsafe { CStr::from_ptr($ptr).to_str() }, $error)
    }};
}

macro_rules! runtime {
    () => {
        match RUNTIME.as_ref() {
            Ok(rt) => rt,
            Err(_) => {
                return 0;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn last_error_length() -> i32 {
    ffi_helpers::error_handling::last_error_length()
}

#[no_mangle]
pub unsafe extern "C" fn error_message_utf8(buf: *mut raw::c_char, length: i32) -> i32 {
    ffi_helpers::error_handling::error_message_utf8(buf, length)
}

#[no_mangle]
pub extern "C" fn load_page(port: i64, url: *const raw::c_char) -> i32 {
    let rt = runtime!();
    let url = cstr!(url);
    let t = Isolate::new(port).task(hb::load_page(url));
    rt.spawn(t);
    1
}
pub async fn do_task() -> i32 {
    let mut i = 0;
    while i < 10 {
        i = i + 1;
        unsafe {
            post_dart(i.to_string());
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn start_timer(port: i64) -> i32 {
    PORT_COBJECT.store(Some(port), Ordering::Relaxed);
    let rt = runtime!();
    rt.spawn(do_task());
    1
}
// callback
fn hb_rec(
    msg: Result<Frame, WsProtocolError>,
    ctx: &mut Context<ChatClient>,
) -> Result<serde_json::Value, WsProtocolError> {
    use flate2::read::GzDecoder;
    use std::io::prelude::*;
    match msg {
        Ok(Frame::Binary(txt)) => {
            let mut d = GzDecoder::new(&*txt);
            let mut s = String::new();
            d.read_to_string(&mut s)?;
            if let Ok(r) = serde_json::from_str::<serde_json::Value>(&s) {
                if r["ping"] != json!(null) {
                    let pong = json!({"pong":r["ping"]}).to_string();
                    ctx.notify(ClientCommand(WsMsg::Message(Message::Text(pong))));
                    unsafe {
                        post_dart(s);
                    };
                    Ok(serde_json::Value::Null)
                } else {
                    unsafe {
                        post_dart(s);
                    };
                    Ok(json!({"id":r["ch"],"value":r}))
                }
            } else {
                Ok(serde_json::Value::Null)
            }
        }
        Err(e) => Err(e),
        _other => Ok(serde_json::Value::Null),
    }
}
fn hb_start(_ctx: &mut Context<ChatClient>) {
    unsafe { post_dart("ws start".into()) };
}
fn hb_stop(_ctx: &mut Context<ChatClient>) {
    unsafe { post_dart("ws stop".into()) };
}
unsafe fn post_dart(msg: String) -> i32 {
    if let Some(port) = PORT_COBJECT.load(Ordering::Relaxed) {
        let rt = runtime!();
        let t = Isolate::new(port).task(async move { msg });
        rt.spawn(t);
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn start_ws(url: *const raw::c_char) -> *mut Ws {
    let url = CStr::from_ptr(url).to_str().unwrap();
    let mut ws = Ws::new(url);
    let _ = ws.add_hook_start(hb_start);
    let _ = ws.add_hook_stop(hb_stop);
    let _ = ws.add_hook(hb_rec);
    ws.connect();
    let _ = ws.send_msg("{\"sub\":\"market.btcusdt.bbo\",\"id\":\"id1\"}");
    if ws.is_alive() {
        post_dart("new ws ssl ok".into());
        let b = Box::new(ws);
        Box::into_raw(b)
    } else {
        post_dart("start ws error".into());
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn is_alive(ws: *mut Ws) -> bool {
    (*ws).is_alive()
}
#[no_mangle]
pub unsafe extern "C" fn stop_ws(ws: *mut Ws) {
    (*ws).close()
}
#[no_mangle]
pub unsafe extern "C" fn send_msg(ws: *mut Ws, msg: *const raw::c_char) -> bool {
    let msg = CStr::from_ptr(msg).to_str().unwrap();
    if let Ok(()) = (*ws).send_msg(msg) {
        true
    } else {
        false
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_symbols() {}
