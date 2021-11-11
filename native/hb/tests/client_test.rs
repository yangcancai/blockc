use actix::*;
use actix_web::client::WsProtocolError;
use actix_web_actors::ws::Frame;
use awc::ws::Message;
use hb::client::*;
use hb::ws::Ws;
#[test]
fn echo() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let mut ws = Ws::new("wss://api.huobi.pro/ws");
    let _ = ws.add_hook(hb);
    let _ = ws.add_hook_start(hb_start);
    let _ = ws.add_hook_stop(hb_stop);
    ws.connect();
    assert_eq!(ws.is_alive(), true);
    let s = ws.state();
    assert_eq!(s["url"], "wss://api.huobi.pro/ws");
    assert_eq!(s["is_alive"], "true");
    let _ = ws.send_msg("{\"sub\":\"market.btcusdt.bbo\",\"id\":\"id1\"}");
    std::thread::sleep(std::time::Duration::from_secs(1));
    assert_ne!(ws.get_data("market.btcusdt.bbo"), serde_json::Value::Null);
    ws.close();
    let s = ws.state();
    assert_eq!(s["url"], "wss://api.huobi.pro/ws");
    assert_eq!(s["is_alive"], "false");
    assert_eq!(ws.is_alive(), false);
    let mut ws = Ws::new("ss://api.huobi.pro/ws");
    assert_eq!(ws.is_alive(), false);
}

fn hb_start(_ctx: &mut Context<ChatClient>) {
    println!("hb_start");
}
fn hb_stop(_ctx: &mut Context<ChatClient>) {
    println!("hb_stop");
}
fn hb(
    msg: Result<Frame, WsProtocolError>,
    ctx: &mut Context<ChatClient>,
) -> Result<serde_json::Value, WsProtocolError> {
    use flate2::read::GzDecoder;
    use serde_json::json;
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
                    println!("ping:{}", s);
                    Ok(json!(null))
                } else {
                    println!("{:?}", s);
                    Ok(json!({"id":r["ch"],"value":r}))
                }
            } else {
                Ok(json!(null))
            }
        }
        Err(e) => {
            println!("{:?}", e);
            Err(e)
        }
        _other => Ok(serde_json::Value::Null),
    }
}
