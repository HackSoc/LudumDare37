extern crate pancurses;

use pancurses::*;

const X: usize = 63;
const Y: usize = 31;

#[derive(Clone, Copy)]
enum Static {
    Wall,
    Gate,
    Goal,
    Turret,
    Obstacle
}

#[derive(Clone, Copy)]
enum Mobile {
    Player,
    Fiend,
    Arrow
}

static mut statics: [[Option<Static>; X]; Y] = [[None; X]; Y];
static mut mobiles: [[Option<Mobile>; X]; Y] = [[None; X]; Y];

fn main() {
    let window = initscr();
    for row_n in 0..Y {
        for col_n in 0..X {
            window.mvaddch(row_n as i32, col_n as i32, '+');
        }
    }
    let _ = window.getch();
    let _ = endwin();
}
