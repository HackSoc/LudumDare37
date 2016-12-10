use model::{X, Y, WorldData, Static, Mobile};

use pancurses::Window;

impl WorldData {
    pub fn render(&self, window: &Window) {
        for row_n in 0..Y {
            for col_n in 0..X {
                let mut ch = ' ';
                if let Some(mob) = self.mobiles[row_n][col_n] {
                    ch = match mob {
                        Mobile::Player => '@',
                        Mobile::Fiend => 'f',
                        Mobile::Arrow => '/'
                    };
                } else if let Some(stat) = self.statics[row_n][col_n] {
                    ch = match stat {
                        Static::Wall => '#',
                        Static::Gate => if row_n == 0 || row_n == Y-1 { '-' } else { '|' },
                        Static::Goal => 'Y',
                        Static::Turret => 'O',
                        Static::Obstacle => '=',
                    };
                }
                window.mvaddch(row_n as i32, col_n as i32, ch);
            }
        }
    }
}
