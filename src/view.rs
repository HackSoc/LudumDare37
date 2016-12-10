use model::*;

use pancurses::Window;

impl WorldData {
    pub fn render(&self, window: &Window) {
        for row_n in 0..Y {
            for col_n in 0..X {
                let ch = self.mobiles[row_n][col_n]
                    .map_or(self.statics[row_n][col_n]
                            .map_or(' ', |s| self.render_static(row_n, s)), |m| self.render_mobile(m));
                window.mvaddch(row_n as i32, col_n as i32, ch);
            }
        }
    }

    pub fn render_mobile(&self, mob: Mobile) -> char {
        match mob {
            Player => '@',
            Fiend => 'f',
            Arrow => '/'
        }
    }

    pub fn render_static(&self, row_n: usize, stat: Static) -> char {
        match stat {
            Wall => '#',
            Gate => if row_n == 0 || row_n == Y-1 { '-' } else { '|' },
            Goal => 'Y',
            Turret => 'O',
            Obstacle => '=',
        }
    }
}
