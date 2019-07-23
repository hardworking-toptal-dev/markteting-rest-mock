#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[post("/single_send.json")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/v2/sms", routes![index]).launch();
}
