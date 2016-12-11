use model::*;

use pancurses::*;

type color_pair = u32;
const DEFAULT_COLORS: color_pair = 0;
const GOAL_COLORS: color_pair = 1;
const BROKEN_TURRET_COLORS: color_pair = 2;
const DAMAGED_TURRET_COLORS: color_pair = 3;

const EMPTY_CELL: chtype = ' ' as u32;

pub struct GameWindows {
    stats: Window,
    view: Window,
    help: Window,
    log: Window,
}

pub fn setup_render(window: &Window) -> GameWindows {
    start_color();
    use_default_colors();
    init_pair(DEFAULT_COLORS as i16, COLOR_WHITE, -1);
    init_pair(GOAL_COLORS as i16, COLOR_YELLOW, -1);
    init_pair(BROKEN_TURRET_COLORS as i16, COLOR_RED, -1);
    init_pair(DAMAGED_TURRET_COLORS as i16, COLOR_MAGENTA, -1);

    let stats = window.subwin(5, X as i32, 0, 0).unwrap();
    stats.keypad(true);
    stats.draw_box(0, 0);
    stats.mvaddstr(3, 2, "THIS IS THE STATUS BOX");
    let view = window.subwin(Y as i32, X as i32, 5, 0).unwrap();
    view.keypad(true);
    let help = window.subwin(5 + Y as i32 + 7, 10, 0, X as i32).unwrap();
    help.draw_box(0, 0);
    help.keypad(true);
    let log = window.subwin(7, X as i32, 5 + Y as i32, 0).unwrap();
    log.draw_box(0, 0);
    log.keypad(true);
    return GameWindows {
        stats: stats,
        view: view,
        help: help,
        log: log,
    };
}

impl WorldData {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn render(&self, windows: &GameWindows) {
        for row_n in 0..Y {
            for col_n in 0..X {
                let ch = self.mobiles[row_n][col_n].map_or(
                    self.statics[row_n][col_n].map_or(
                        EMPTY_CELL, |s| {
                            self.render_static(row_n, s)
                        }),
                    |m| self.render_mobile(m));
                windows.view.mvaddch(row_n as i32, col_n as i32, ch);
            }
        }
        windows.stats.refresh();
        windows.view.refresh();
        windows.help.refresh();
        windows.log.refresh();
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
