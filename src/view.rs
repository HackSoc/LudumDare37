use model::*;

use pancurses::*;

type color_pair = u32;
const DEFAULT_COLORS: color_pair = 0;
const GOAL_COLORS: color_pair = 1;
const BROKEN_TURRET_COLORS: color_pair = 2;
const DAMAGED_TURRET_COLORS: color_pair = 3;

const EMPTY_CELL: chtype = ' ' as u32;

pub fn setup_render() {
    start_color();
    use_default_colors();
    init_pair(DEFAULT_COLORS as i16, COLOR_WHITE, -1);
    init_pair(GOAL_COLORS as i16, COLOR_YELLOW, -1);
    init_pair(BROKEN_TURRET_COLORS as i16, COLOR_RED, -1);
    init_pair(DAMAGED_TURRET_COLORS as i16, COLOR_MAGENTA, -1);
}

impl WorldData {
    pub fn render(&self, window: &Window) {
        for row_n in 0..Y {
            for col_n in 0..X {
                let ch = self.mobiles[row_n][col_n].map_or(self.statics[row_n][col_n]
                                                               .map_or(EMPTY_CELL, |s| {
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
                Fiend { info } => info.ch,
                Arrow { info: ArrowInfo { dx, dy, incx, incy, .. } } => {
                    if dx == 0 {
                        '|'
                    } else if dy == 0 {
                        '-'
                    } else if incx == incy {
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
                Goal { .. } => 'Y',
                Turret { .. } => 'O',
                Obstacle { .. } => '=',
            }
            .to_chtype();

        // Apply formatting
        match stat {
            Goal { .. } => chty | COLOR_PAIR(GOAL_COLORS),
            Turret { info } => {
                let colour = if info.health == 0 {
                    BROKEN_TURRET_COLORS
                } else if info.health <= info.max_health / 2 {
                    DAMAGED_TURRET_COLORS
                } else {
                    DEFAULT_COLORS
                };
                chty | A_BOLD | COLOR_PAIR(colour)
            }
            _ => chty,
        }
    }
}
