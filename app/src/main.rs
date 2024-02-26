#[macro_use] extern crate rocket;

use rocket::State;
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;

mod checksum;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    api_secret: String
}

#[get("/")]
fn index() -> &'static str {
    "success"
}

#[get("/getMeetings?<checksum>")]
fn get_meetings(checksum: &str, config: &State<Config>) -> String {
    let secret = &config.api_secret;
    if checksum::validate_checksum(checksum, "getMeetings", "", secret) {
        format!("checksum {checksum} is valid")
    }
    else {
        format!("checksum {checksum} is invalid")
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/bigbluebutton/api", routes![index, get_meetings]).attach(AdHoc::config::<Config>())
}
