use std::cmp::max;
use crate::{EmpireData, MilleniumFalconData, PathStep};
use log::trace;

// Runs a complete path simulation and returns the highest % chance
pub fn simulate(paths: &Vec<Vec<PathStep>>, millenium_falcon: &MilleniumFalconData, empire: &EmpireData) -> u8 {
    let mut final_chance = 0;

    for path in paths {
        // For every path, simulate with no sleep and sleep once per planet.
        for i in -1..(path.len() as i32) {
            let chance = simulate_path(&path, &millenium_falcon, &empire, i);
            trace!("Path {:?} (sleeping at step {}) has {}% chance of succeeding", path, i, chance);
            final_chance = max(final_chance, chance);
        }
    }

    final_chance
}

// Runs a path simulation and returns the % chance it succeeds
pub fn simulate_path(path: &Vec<PathStep>, mill_falc_data: &MilleniumFalconData, empire_data: &EmpireData, sleep_on_path: i32) -> u8 {
    let mut energy = mill_falc_data.autonomy;
    let mut current_day = 0;
    let mut days_with_bhunters = 0;

    for step in 0..path.len() {
        // We consider the start of a step to be when we arrive on a planet
        // Therefore, we first check to see if there are bhunters:
        days_with_bhunters += has_bounty_hunters(path, empire_data, current_day, step);

        let cost = if step+1 < path.len() { path[step+1].cost } else { 0 };

        // If we do not have the energy needed, we sadly have to rest for a day.
        if cost > energy || step == sleep_on_path as usize {
            current_day += 1;
            energy = mill_falc_data.autonomy;

            days_with_bhunters += has_bounty_hunters(path, empire_data, current_day, step);
        }

        // Consume the energy
        energy -= cost;
        // Travel however many days
        current_day += cost;

        if current_day > empire_data.countdown {
            return 0; // We spent too much time on this path, we are doomed!
        }
    }

    let mut chance = 0.0;
    for day in 0..days_with_bhunters {
        if day == 0 {
            chance += 1.0/10.0;
        } else {
            chance += 9.0_f32.powi(day) / 10.0_f32.powi(day+1)
        }
    }

    100 - ((chance * 100.0) as u8)
}

fn has_bounty_hunters(path: &Vec<PathStep>, empire_data: &EmpireData, current_day: u32, step: usize) -> i32 {
    empire_data.bounty_hunters
        .iter()
        .find(|bh| bh.day == current_day && bh.planet == path[step].to)
        .map_or_else(|| 0, |_| 1)
}