use hb_ffi::Market;
use serde_json::json;
#[test]
fn string_to_char() {
    let value = json!(
	{"ch":"market.btcusdt.bbo","ts":1636625370389 as i64,"tick":{"seqId":141391088807 as i64,"ask":65321.43,"askSize":0.04526391131853666 as i64,"bid":65321.42,"bidSize":0.432766,"quoteTime":1636625370388 as i64,"symbol":"btcusdt"}});
    let _m = Market::new(value.as_object().unwrap());
    assert_eq!(1,1);
}