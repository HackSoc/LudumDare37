extern crate pancurses;

mod model;
mod view;

use model::*;

use pancurses::*;

fn main() {
    let window = initscr();
    let world_data = WorldData {
        statics: [[None; X]; Y],
        mobiles: [[None; X]; Y],
    };
    world_data.render(&window);
    let _ = window.getch();
    let _ = endwin();
}
