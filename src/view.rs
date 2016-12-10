use model::*;

use pancurses::{A_BOLD, ToChtype, Window, chtype};

impl WorldData {
    pub fn render(&self, window: &Window) {
        for row_n in 0..Y {
            for col_n in 0..X {
                let ch = self.mobiles[row_n][col_n].map_or(self.statics[row_n][col_n]
                                                               .map_or(' '.to_chtype(), |s| {
                                                                   self.render_static(row_n, s)
                                                               }),
                                                           |m| self.render_mobile(m));
                window.mvaddch(row_n as i32, col_n as i32, ch);
            }
        }
    }

    pub fn render_mobile(&self, mob: Mobile) -> chtype {
        match mob {
                Player => '@',
                Fiend => 'f',
                Arrow { dx, dy } => {
                    if dx == 0 {
                        '|'
                    } else if dy == 0 {
                        '-'
                    } else if (dx < 0 && dy < 0) || (dy > 0 && dx > 0) {
                        '\\'
                    } else {
                        '/'
                    }
                }
            }
            .to_chtype()
    }

    pub fn render_static(&self, row_n: usize, stat: Static) -> chtype {
        let chty = match stat {
                Wall => '#',
                Gate => {
                    if row_n == 0 || row_n == Y - 1 {
                        '-'
                    } else {
                        '|'
                    }
                }
                Goal => 'Y',
                Turret => 'O',
                Obstacle => '=',
            }
            .to_chtype();

        if stat == Turret { chty | A_BOLD } else { chty }
    }
}
