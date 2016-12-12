extern crate astar;
extern crate pancurses;
extern crate rand;

mod controller;
mod model;
mod view;
mod fiends;
mod util;

use std::collections::BTreeSet;
use std::env;

use model::*;

use pancurses::*;

fn initial_world() -> WorldData {
    let mut world_data = WorldData {
        statics: [[None; X]; Y],
        mobiles: [[None; X]; Y],
        fiends: BTreeSet::new(),
        arrows: BTreeSet::new(),
        turrets: BTreeSet::new(),
        obstacles: BTreeSet::new(),
        gates: BTreeSet::new(),
        player_info: PlayerInfo {
            location: (20, 20),
            health: 100,
            max_health: 100,
            damage_factor: 1,
            heal_factor: 1,
            armour_factor: 1,
        },
        goal_location: (X / 2, Y / 2),
        log: ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        cash: 0,
        wave: 0,
    };
    // add walls!
    for x in 0..X {
        world_data.statics[0][x] = Some(Wall);
        world_data.statics[Y - 1][x] = Some(Wall);
    }
    for y in 0..Y {
        world_data.statics[y][0] = Some(Wall);
        world_data.statics[y][X - 1] = Some(Wall);
    }
    // add goal!
    world_data.statics[Y / 2][X / 2] = Some(Goal {
        health: 10,
        max_health: 10,
    });

    // add gates!
    for x in 0..7 {
        let gx = x + (X / 2) - 3;
        world_data.statics[0][gx] = Some(Gate);
        world_data.statics[Y - 1][gx] = Some(Gate);
        world_data.gates.insert((gx, 0));
        world_data.gates.insert((gx, Y - 1));
    }

    for y in 0..7 {
        let gy = y + (Y / 2) - 3;
        world_data.statics[gy][0] = Some(Gate);
        world_data.statics[gy][X - 1] = Some(Gate);
        world_data.gates.insert((0, gy));
        world_data.gates.insert((X - 1, gy));
    }

    world_data.mobiles[20][20] = Some(Player);

    return world_data;
}

fn main() {
    if env::args().nth(1) == Some("name_gen".to_string()) {
        name_gen()
    } else {
        play()
    }
}

fn name_gen() {
    for wave in 1..101 {
        let the_fiends = fiends::make_wave(wave);
        let mut the_names = BTreeSet::new();
        for fiend in the_fiends {
            the_names.insert(fiend.name);
        }
        let mut names = "".to_string();
        let mut i = 0;
        for name in &the_names {
            i += 1;
            names = format!("{} {}{}",
                            names,
                            name,
                            if i == the_names.len() { "" } else { "," });
        }
        if wave % 10 == 0 {
            println!("Wave {}:{} [BIG BOSS]", wave, names);
        } else if wave % 5 == 0 {
            println!("Wave {}:{} [BOSS]", wave, names);
        } else {
            println!("Wave {}:{}", wave, names);
        }
    }
}

fn play() {
    let window = initscr();
    let _ = noecho();
    let _ = curs_set(0);
    let mut world_data = initial_world();
    let game_windows = view::setup_render(&window);
    let _ = window.keypad(true);
    let mut gamestate = model::Construct {
        menu: Menu::Root,
        menu_index: 0,
    };
    while gamestate != model::End {
        world_data.render(&game_windows, &gamestate);
        if let Some(i) = window.getch() {
            gamestate.handle(&mut world_data, i)
        }
    }
    let _ = endwin();
}
