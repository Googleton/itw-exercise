use crate::sim::{simulate};
use crate::travel::{Graph, PathStep};
use serde::Deserialize;

mod travel;
mod sim;

#[derive(Clone, Deserialize, Debug)]
pub struct MilleniumFalconData {
    pub autonomy: u32,
    pub departure: String,
    pub arrival: String,
    pub routes_db: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct EmpireData {
    pub countdown: u32,
    pub bounty_hunters: Vec<BountyHunterData>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct BountyHunterData {
    pub planet: String,
    pub day: u32,
}

#[derive(Debug)]
pub struct Route {
    pub origin: String,
    pub destination: String,
    pub travel_time: u32
}

// Computes the odds of our heroes making it before the Death Star fires
// Returns the probability of making it, as a 0-100 integer.
pub fn get_odds(millenium_falcon: MilleniumFalconData, empire: EmpireData) -> u8 {
    let connection = sqlite::open(millenium_falcon.routes_db.clone()).unwrap();
    let mut routes: Vec<Route> = vec![];

    connection.iterate("SELECT * FROM routes", |pairs| {
        let route = Route {
            origin: pairs[0].1.unwrap().to_string(),
            destination: pairs[1].1.unwrap().to_string(),
            travel_time: pairs[2].1.unwrap().parse().unwrap()
        };
        routes.push(route);
        true
    })
        .unwrap();

    // Get all paths between two points
    let graph = Graph::new(&routes);
    let paths = graph.get_all_paths(millenium_falcon.departure.clone(), millenium_falcon.arrival.clone());

    simulate(&paths, &millenium_falcon, &empire)
}

#[cfg(test)]
mod tests {
    use crate::{BountyHunterData, EmpireData, get_odds, MilleniumFalconData};

    #[test]
    fn seven_days_is_not_enough() {
        let odds = get_odds(MilleniumFalconData {
            routes_db: String::from("universe.db"),
            autonomy: 6,
            departure: String::from("Tatooine"),
            arrival: String::from("Endor")
        }, EmpireData {
            countdown: 7,
            bounty_hunters: vec![
                    BountyHunterData {
                        planet: "Hoth".to_string(), day: 6
                    },
                    BountyHunterData {
                        planet: "Hoth".to_string(), day: 7
                    },
                    BountyHunterData {
                        planet: "Hoth".to_string(), day: 8
                    }
            ]
        });

        assert_eq!(odds, 0);
    }

    #[test]
    fn eight_days_is_a_bit_better() {
        let odds = get_odds(MilleniumFalconData {
            routes_db: String::from("universe.db"),
            autonomy: 6,
            departure: String::from("Tatooine"),
            arrival: String::from("Endor")
        }, EmpireData {
            countdown: 8,
            bounty_hunters: vec![
                BountyHunterData {
                    planet: "Hoth".to_string(), day: 6
                },
                BountyHunterData {
                    planet: "Hoth".to_string(), day: 7
                },
                BountyHunterData {
                    planet: "Hoth".to_string(), day: 8
                }
            ]
        });

        assert_eq!(odds, 81);
    }

    #[test]
    fn nine_days_is_even_better() {
        let odds = get_odds(MilleniumFalconData {
            routes_db: String::from("universe.db"),
            autonomy: 6,
            departure: String::from("Tatooine"),
            arrival: String::from("Endor")
        }, EmpireData {
            countdown: 9,
            bounty_hunters: vec![
                BountyHunterData {
                    planet: "Hoth".to_string(), day: 6
                },
                BountyHunterData {
                    planet: "Hoth".to_string(), day: 7
                },
                BountyHunterData {
                    planet: "Hoth".to_string(), day: 8
                }
            ]
        });

        assert_eq!(odds, 90);
    }

    #[test]
    fn ten_days_is_perfect() {
        let odds = get_odds(MilleniumFalconData {
            routes_db: String::from("universe.db"),
            autonomy: 6,
            departure: String::from("Tatooine"),
            arrival: String::from("Endor")
        }, EmpireData {
            countdown: 10,
            bounty_hunters: vec![
                BountyHunterData {
                    planet: "Hoth".to_string(), day: 6
                },
                BountyHunterData {
                    planet: "Hoth".to_string(), day: 7
                },
                BountyHunterData {
                    planet: "Hoth".to_string(), day: 8
                }
            ]
        });

        assert_eq!(odds, 100);
    }
}
