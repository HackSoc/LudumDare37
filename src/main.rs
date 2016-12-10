extern crate pancurses;

mod controller;
mod model;
mod view;
mod enemies;

use model::*;

use pancurses::*;

fn initial_world() -> WorldData {
    let mut world_data = WorldData {
        statics: [[None; X]; Y],
        mobiles: [[None; X]; Y],
        player_info: PlayerInfo {
            location: (20, 20),
            health: 100,
            max_health: 100,
            damage_factor: 1,
            heal_factor: 1,
            armour_factor: 1,
        },
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
        world_data.statics[0][x + (X / 2) - 3] = Some(Gate);
        world_data.statics[Y - 1][x + (X / 2) - 3] = Some(Gate);
    }

    for y in 0..7 {
        world_data.statics[y + (Y / 2) - 3][0] = Some(Gate);
        world_data.statics[y + (Y / 2) - 3][X - 1] = Some(Gate);
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

    // Some example enemies.
    world_data.mobiles[Y - 1][1 + (X / 2) - 3] = Some(enemies::kobold());
    world_data.mobiles[Y - 3][3 + (X / 2) - 3] = Some(enemies::kobold());
    world_data.mobiles[Y - 1][5 + (X / 2) - 3] = Some(enemies::kobold());

    return world_data;
}

fn main() {
    let window = initscr();
    let _ = noecho();
    let _ = curs_set(0);
    let mut world_data = initial_world();
    world_data.setup_render();
    let _ = window.keypad(true);
    let mut gamestate = controller::Fight;
    while gamestate != controller::End {
        world_data.render(&window);
        if let Some(i) = window.getch() {
            gamestate.handle(&mut world_data, i)
        }
    }
    let _ = endwin();
}
