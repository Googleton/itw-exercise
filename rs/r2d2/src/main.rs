extern crate core;

use std::{env, fs};
use commons::{EmpireData, get_odds, MilleniumFalconData};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Wrong args. Format: r2d2 <millenium falcon json path> <empire data json path>");
        return;
    }

    let millenium_falcon_path = args.get(1).expect("Millenium Falcon path not found");
    let empire_path = args.get(2).expect("Empire path not found");

    let mf_data = fs::read_to_string(millenium_falcon_path)
        .expect(format!("Unable to read file {}", millenium_falcon_path).as_str());
    let mill_falc: MilleniumFalconData = serde_json::from_str(mf_data.as_str())
        .expect(format!("Could not parse file {}", millenium_falcon_path).as_str());

    let empire_data = fs::read_to_string(empire_path)
        .expect(format!("Unable to read file {}", empire_path).as_str());
    let empire: EmpireData = serde_json::from_str(empire_data.as_str())
        .expect(format!("Could not parse file {}", empire_path).as_str());

    println!("{}", get_odds(mill_falc, empire));
}
