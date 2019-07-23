#![feature(proc_macro_hygiene, decl_macro)]

#[deny(overflowing_literals)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;



mod yunpian;
// use yunpian::single_send;

fn main() {
    rocket::ignite()
        .mount("/v2/sms", routes![yunpian::single_send])
        .launch();
}
