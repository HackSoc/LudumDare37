extern crate pancurses;

mod model;
mod view;

use view::render;

use pancurses::*;


fn main() {
    let window = initscr();
    render(&window);
    let _ = window.getch();
    let _ = endwin();
}
