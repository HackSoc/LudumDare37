use model::{X, Y, WorldData};

use pancurses::Window;

impl WorldData {
    pub fn render(&self, window: &Window) {
        for row_n in 0..Y {
            for col_n in 0..X {
                window.mvaddch(row_n as i32, col_n as i32, '+');
            }
        }
    }
}
