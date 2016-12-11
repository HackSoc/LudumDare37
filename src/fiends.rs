use model::*;

use rand::{Rng, thread_rng};
use std::cmp::max;

// The absolute minimum cost of a fiend.
const MIN_POINTS: usize = 8;

// Minimum point score, character, and name. Point score is to prevent
// pitifully weak demons, for instance.
const SPECIES: [(usize, char, &'static str); 5] = [(15, 'k', "kobold"),
                                                   (15, 'w', "waynhim"),
                                                   (75, 'd', "demondim"),
                                                   (100, 'v', "vile"),
                                                   (150, 'G', "giant")];

// Variants: scale the minimum point cost of a thing.
const VARIANTS: [(f64, &'static str); 3] = [(0.5, "lesser "), (2.0, "greater "), (5.0, "ur-")];

// Archetypes: AI roles.
const ARCHETYPES: [(usize, usize, usize, usize); 3] =
    [(75, 10, 5, 5), (10, 75, 5, 5), (5, 5, 25, 25)];

// Maximum number of types of enemies on each wave.
const MAX_TYPES: [(usize, usize); 4] = [(2, 2), (5, 3), (10, 5), (15, 10)];

pub fn make_wave(wave: usize) -> Vec<FiendInfo> {
    // TODO: boss monsters every so often.
    let points = 50 + wave * wave * (wave as f64).ln().trunc() as usize;

    // Work out how many types of fiend we'll have.
    let mut max_types = 1;
    for &(w, maxty) in MAX_TYPES.iter() {
        if w > wave {
            break;
        }
        max_types = maxty;
    }
    let fiend_types = 1 + gen_range_panic(&"fiend_types", 0, max_types);

    // Generate fiends.
    let mut fiends: Vec<FiendInfo> = Vec::new();
    let mut remaining = points;
    for i in 0..fiend_types {
        // Allocate points for this type.
        let max_points = remaining - (fiend_types - i - 1) * MIN_POINTS;
        let allocated = if i == fiend_types - 1 {
            remaining
        } else {
            gen_range_panic(&"allocated", MIN_POINTS, 1 + max_points)
        };
        remaining -= allocated;

        // Determine the cost of one.
        let cost = gen_range_panic(&"cost", MIN_POINTS, 1 + max(MIN_POINTS, allocated / 3));

        // Generate fiend.
        let fiend = make_fiend(cost).expect("Should have been able to afford a fiend");

        // Populate fiends vector.
        let mut my_remaining = allocated;
        while my_remaining >= cost {
            fiends.push(fiend);
            my_remaining -= cost;
        }
        remaining += my_remaining;
    }
    fiends
}

pub fn make_fiend(points: usize) -> Option<FiendInfo> {
    // Affordable fiends.
    let mut choices: Vec<(char, FiendName)> = Vec::new();
    for &(min_cost, ch, name) in SPECIES.iter() {
        if min_cost <= points {
            choices.push((ch,
                          FiendName {
                              prefix: None,
                              name: name,
                              suffix: None,
                          }));
        }
        for &(scale, title) in VARIANTS.iter() {
            if min_cost as f64 * scale > points as f64 {
                continue;
            }
            choices.push((ch,
                          FiendName {
                              prefix: Some(title),
                              name: name,
                              suffix: None,
                          }));
        }
    }

    if choices.len() == 0 {
        return None;
    }

    // Choose one
    let (ch, name) = choices[gen_range_panic(&"choose one", 0, choices.len())];

    // Assign points to stuff.
    let max_health = points;
    let damage_factor = gen_range_panic(&"damage_factor", points / 3, 1 + points);
    let armour_factor = points - damage_factor;

    let (player_target_distance,
         goal_target_distance,
         turret_target_distance,
         obstacle_target_distance) = ARCHETYPES[gen_range_panic(&"archetype", 0, ARCHETYPES.len())];

    Some(FiendInfo {
        ch: ch,
        name: name,
        form: (),
        health: max_health,
        max_health: max_health,
        damage_factor: damage_factor,
        armour_factor: armour_factor,
        player_target_distance: player_target_distance,
        goal_target_distance: goal_target_distance,
        turret_target_distance: turret_target_distance,
        obstacle_target_distance: obstacle_target_distance,
    })
}

fn gen_range_panic(msg: &str, lo: usize, hi: usize) -> usize {
    if lo >= hi {
        panic!("{} > {}: {}", lo, hi, msg);
    }
    thread_rng().gen_range(lo, hi)
}
