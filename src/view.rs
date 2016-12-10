use model::{X, Y};

use pancurses::Window;

pub fn render(window: &Window) {
    for row_n in 0..Y {
        for col_n in 0..X {
            window.mvaddch(row_n as i32, col_n as i32, '+');
        }
    }
}
