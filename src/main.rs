extern crate pancurses;

mod model;
mod view;

use model::*;

use pancurses::*;

fn initial_world() -> WorldData {
    let mut world_data = WorldData {
        statics: [[None; X]; Y],
        mobiles: [[None; X]; Y],
    };
    // add walls!
    for x in 0..X {
        world_data.statics[0][x] = Some(Static::Wall);
        world_data.statics[Y-1][x] = Some(Static::Wall);
    }
    for y in 0..Y {
        world_data.statics[y][0] = Some(Static::Wall);
        world_data.statics[y][X-1] = Some(Static::Wall);
    }
    // add goal!
    world_data.statics[Y/2][X/2]= Some(Static::Goal);

    // add gates!
    for x in 0..7 {
        world_data.statics[0][x + (X/2) - 3] = Some(Static::Gate);
        world_data.statics[Y-1][x + (X/2) - 3] = Some(Static::Gate);
    }

    for y in 0..7 {
        world_data.statics[y + (Y/2) - 3][0] = Some(Static::Gate);
        world_data.statics[y + (Y/2) - 3][X-1] = Some(Static::Gate);
    }
    
    return world_data;
}

fn main() {
    let window = initscr();
    let world_data = initial_world();
    world_data.render(&window);
    let _ = window.getch();
    let _ = endwin();
}
