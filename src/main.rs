extern crate astar;
extern crate pancurses;
extern crate rand;

mod controller;
mod model;
mod view;
mod fiends;
mod util;

use std::collections::BTreeSet;

use model::*;

use pancurses::*;

use fiends::make_wave;

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
        menu: Menu::Root,
        menu_index: 0,
        placement: None
        log: ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
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

    // Some example turrets.
    let turret_info = TurretInfo {
        form: (),
        cooldown: 0,
        max_cooldown: 3,
        range: 50,
        health: 100,
        max_health: 100,
        arrow_speed: 2,
    };
    world_data.statics[Y / 2][X / 2 - 1] = Some(Turret { info: turret_info });
    world_data.statics[Y / 2][X / 2 + 1] = Some(Turret { info: turret_info });
    world_data.turrets.insert((X / 2 - 1, Y / 2));
    world_data.turrets.insert((X / 2 + 1, Y / 2));

    return world_data;
}

fn main() {
    let window = initscr();
    let _ = noecho();
    let _ = curs_set(0);
    let mut world_data = initial_world();
    let game_windows = view::setup_render(&window);
    let _ = window.keypad(true);
    let mut gamestate = model::Construct;
    while gamestate != model::End {
        world_data.render(&game_windows, &gamestate);
        if let Some(i) = window.getch() {
            gamestate.handle(&mut world_data, i)
        }
    }
    let _ = endwin();
}
