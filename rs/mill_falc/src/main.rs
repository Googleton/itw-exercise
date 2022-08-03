mod routes;

#[macro_use] extern crate rocket;

use std::fs;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method};
use rocket::{Request, Response};
use commons::MilleniumFalconData;
use crate::routes::sim::post_sim;

pub struct Config {
    millenium_falcon: MilleniumFalconData
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS headers",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
fn rocket() -> _ {
    let config_data = fs::read_to_string("./millenium_falcon.json")
        .expect("Unable to read file millenium_falcon.json");

    let mill_falc: MilleniumFalconData = serde_json::from_str(config_data.as_str())
        .expect("Could not parse file millenium_falcon.json");

    rocket::build()
        .manage(Config { millenium_falcon: mill_falc})
        .attach(Cors)
        .mount("/", routes![post_sim, all_options])
}