use model::*;

use rand::{Rng, thread_rng};

// TODO: Figure out names

// Minimum point score, character, and name. Point score is to prevent
// pitifully weak demons, for instance.
const fiend_species: [(usize, char /* , str */); 1] = [(1, 'k' /* , "kobold" */)];

// Variants: scale the minimum point cost of a thing.
const fiend_variants: [f64; 2] = [0.5 /* , "lesser") */, 2.0 /* "greater") */];

// Archetypes: AI roles.
const fiend_archetypes: [(usize, usize, usize, usize); 3] =
    [(75, 10, 5, 5), (10, 75, 5, 5), (5, 5, 25, 25)];

pub fn make_fiend(points: usize) -> Option<Mobile> {
    match make_fiend_info(points) {
        Some(fiend_info) => Some(Fiend { info: fiend_info }),
        None => None,
    }
}

pub fn make_fiend_info(points: usize) -> Option<FiendInfo> {
    // Affordable fiends.
    let mut choices: Vec<char /* , str) */> = Vec::new();
    for &(min_cost, ch /* , name */) in fiend_species.iter() {
        for scale/*, title)*/ in fiend_variants.iter() {
            if min_cost as f64 * scale > points as f64 {
                continue;
            }
            choices.push(ch /* , title + " " + name) */);
        }
    }

    if choices.len() == 0 {
        return None;
    }

    // Choose one
    let ch/*, name)*/ = choices[thread_rng().gen_range(0, choices.len())];

    // Assign points to stuff.
    let max_health = points;
    let damage_factor = thread_rng().gen_range(points / 3, points);
    let armour_factor = points - damage_factor;

    let (player_target_distance,
         goal_target_distance,
         turret_target_distance,
         obstacle_target_distance) = fiend_archetypes[thread_rng()
        .gen_range(0, fiend_archetypes.len())];

    Some(FiendInfo {
        ch: ch,
        // name: name,
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
