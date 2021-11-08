use serde_json::json;
#[test]
fn ping() {
    let p = json!({"ping":1234567});
    assert_eq!(p["ping"], json!(1234567));
    let p = json!({"pin":1234567});
    assert_eq!(p["ping"], json!(null));
    let p = json!({"ping":1234567});
    assert_eq!(p["ping"], json!(1234567));
    if p["ping"] != json!(null) {
        assert_eq!(0, 0);
    } else {
        assert_eq!(0, 1);
    }
}
