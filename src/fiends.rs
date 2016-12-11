use model::*;

use rand::{Rng, thread_rng};

// Minimum point score, character, and name. Point score is to prevent
// pitifully weak demons, for instance.
const SPECIES: [(usize, char, &'static str); 1] = [(1, 'k', "kobold")];

// Variants: scale the minimum point cost of a thing.
const VARIANTS: [(f64, &'static str); 3] = [(0.5, "lesser "), (2.0, "greater "), (5.0, "ur-")];

// Archetypes: AI roles.
const ARCHETYPES: [(usize, usize, usize, usize); 3] =
    [(75, 10, 5, 5), (10, 75, 5, 5), (5, 5, 25, 25)];

pub fn make_fiend(points: usize) -> Option<Mobile> {
    match make_fiend_info(points) {
        Some(fiend_info) => Some(Fiend { info: fiend_info }),
        None => None,
    }
}

pub fn make_fiend_info(points: usize) -> Option<FiendInfo> {
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
    let (ch, name) = choices[thread_rng().gen_range(0, choices.len())];

    // Assign points to stuff.
    let max_health = points;
    let damage_factor = thread_rng().gen_range(points / 3, points);
    let armour_factor = points - damage_factor;

    let (player_target_distance,
         goal_target_distance,
         turret_target_distance,
         obstacle_target_distance) = ARCHETYPES[thread_rng().gen_range(0, ARCHETYPES.len())];

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
