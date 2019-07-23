
use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};



#[derive(Debug, FromForm)]
// #[derive(Copy)]
pub struct SingleSendData {
    apikey: String,
    mobile: String,
    text: String,
    extend: String,
    uid: String,
    callback_url: String,
    register: bool,
    mobile_stat: bool,
}

#[post("/single_send.json", data = "<send>")]
pub fn single_send(send: Form<SingleSendData>) -> JsonValue {
    println!("{:?}", send);
    json!({
        "code": 0,
        "msg": "发送成功",
        "count": 1,
        "fee": 0.05,
        "unit": "RMB",
        "mobile": "13200000000",
        "sid": 3310228982 as u64
    })
}
