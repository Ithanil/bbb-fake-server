#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::fairing::AdHoc;
use rocket::http::RawStr;

struct ApiSecret(String);

#[get("/")]
fn index() -> &'static str {
    "success"
}

#[get("/getMeetings?<checksum>")]
fn get_meetings(checksum: &RawStr, api_secret: State<ApiSecret>) -> String {
    let secret = &api_secret.0;
    format!("checksum: {checksum}, secret: {secret}")
}

fn main() {
    rocket::ignite()
        .mount("/bigbluebutton/api", routes![index, get_meetings])
        .attach(AdHoc::on_attach("Secret Config", |rocket| {
            let api_secret = rocket.config()
                .get_str("api_secret")
                .unwrap()
                .to_string();

            Ok(rocket.manage(ApiSecret(api_secret)))
        }))
        .launch();
}
