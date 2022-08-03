use commons::{EmpireData, get_odds};
use rocket::serde::{json::Json};
use rocket::State;
use crate::Config;

#[post("/sim", data = "<data>")]
pub fn post_sim(data: Json<EmpireData>, config: &State<Config>) -> String {
    format!("{}", get_odds(config.millenium_falcon.clone(), data.0.clone()))
}