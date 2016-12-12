use model::*;

use rand::{Rng, thread_rng};

use std::cmp::max;

macro_rules! fiend {
    ($prefix:expr, $name:expr) => (FiendName {
        prefix: $prefix,
        name: $name,
        suffix: None,
    });
    ($prefix:expr, $name:expr, $suffix:expr) => (FiendName {
        prefix: $prefix,
        name: $name,
        suffix: Some($suffix),
    });
}

// The absolute minimum cost of a fiend.
const MIN_POINTS: usize = 8;

// Minimum point score, character, and name. Point score is to prevent
// pitifully weak demons, for instance.
const SPECIES: [(usize, char, &'static str); 5] = [(15, 'k', "kobold"),
                                                   (15, 'w', "waynhim"),
                                                   (75, 'd', "demondim"),
                                                   (100, 'v', "vile"),
                                                   (150, 'G', "giant")];
const BIGBOSS_SPECIES: [(usize, char, &'static str); 8] = [(30, 'I', "Immolator"),
                                                           (30, 'B', "Behemoth"),
                                                           (80, 'M', "Morgoth"),
                                                           (80, 'K', "Kenaustin Ardenol"),
                                                           (120, 'F', "Findail"),
                                                           (120, 'V', "Vain"),
                                                           (170, 'C', "Covenant"),
                                                           (170, 'F', "Foul")];

// Variants: scale the minimum point cost of a thing.
const VARIANTS: [(f64, &'static str); 4] =
    [(0.5, "lesser "), (1.0, ""), (2.0, "greater "), (5.0, "ur-")];
const BOSS_VARIANTS: [(f64, &'static str); 5] =
    [(1.0, "great "), (1.0, "potent "), (1.0, "dark "), (1.0, "mighty "), (1.0, "grim ")];
const BIGBOSS_VARIANTS: [(f64, &'static str); 5] =
    [(1.0, "ak-Haru "), (1.0, "Lord "), (1.0, "Ur-Lord "), (1.0, "Darth "), (1.0, "na-Mhoram ")];

// Suffixes: make big bosses sound more badass.
const BIGBOSS_SUFFIXES: [&'static str; 6] =
    [" the Despirer", " the Appointed", " the Guardian", " the Grim", "-cro", "-in"];

// Archetypes: AI roles.
const ARCHETYPES: [(usize, usize, usize, usize); 3] =
    [(75, 10, 5, 5), (10, 75, 5, 5), (5, 5, 25, 25)];

// Maximum number of types of enemies on each wave.
const MAX_TYPES: [(usize, usize); 4] = [(2, 2), (5, 3), (15, 5), (30, 10)];

pub fn make_wave(wave: usize) -> Vec<FiendInfo> {
    let points = 25 + wave * (wave as f64).ln().round() as usize;

    if wave % 5 == 0 {
        // It's a boss!
        vec![make_boss(points, wave).expect("Should be able to make a boss")]
    } else {
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
}

fn make_boss(points: usize, wave: usize) -> Option<FiendInfo> {
    // Small bosses (every 5th level) are just tough versions of
    // normal fiends.
    //
    // Large bosses (every 10th level) are separate things.

    let boss_archetypes: &[(usize, usize, usize, usize)] =
        &[(150, 5, 5, 5), (5, 150, 5, 5), (5, 5, 150, 150)];

    if wave % 10 == 0 {
        make_fiend_from(points,
                        &BIGBOSS_SPECIES,
                        &BIGBOSS_VARIANTS,
                        &BIGBOSS_SUFFIXES,
                        boss_archetypes)
    } else {
        make_fiend_from(points, &SPECIES, &BOSS_VARIANTS, &[], boss_archetypes)
    }
}

fn make_fiend(points: usize) -> Option<FiendInfo> {
    make_fiend_from(points, &SPECIES, &VARIANTS, &[], &ARCHETYPES)
}

fn make_fiend_from(points: usize,
                   species: &[(usize, char, &'static str)],
                   variants: &[(f64, &'static str)],
                   suffixes: &[&'static str],
                   archetypes: &[(usize, usize, usize, usize)])
                   -> Option<FiendInfo> {
    // Affordable fiends.
    let mut choices: Vec<(char, FiendName)> = Vec::new();
    for &(min_cost, ch, name) in species.iter() {
        for &(scale, title) in variants.iter() {
            if min_cost as f64 * scale > points as f64 {
                continue;
            }
            choices.push((ch, fiend!(title, name)));
            for &descriptor in suffixes.iter() {
                choices.push((ch, fiend!(title, name, descriptor)));
            }
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
         obstacle_target_distance) = archetypes[gen_range_panic(&"archetype", 0, archetypes.len())];

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
        value: points,
    })
}

fn gen_range_panic(msg: &str, lo: usize, hi: usize) -> usize {
    if lo >= hi {
        panic!("{} > {}: {}", lo, hi, msg);
    }
    thread_rng().gen_range(lo, hi)
}
