extern crate pancurses;

use pancurses::*;

const X: usize = 64;
const Y: usize = 32;

#[derive(Clone, Copy)]
struct room_cell{

}

static mut room: [[room_cell; X]; Y] = [[room_cell{}; X]; Y];

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
