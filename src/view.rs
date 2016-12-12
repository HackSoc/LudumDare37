use model::*;

use pancurses::*;

type color_pair = u32;
const DEFAULT_COLORS: color_pair = 0;
const GOAL_COLORS: color_pair = 1;
const BROKEN_TURRET_COLORS: color_pair = 2;
const DAMAGED_TURRET_COLORS: color_pair = 3;
const PLACEMENT_COLORS: color_pair = 4;
const GAMEOVER_COLORS: color_pair = 5;

const EMPTY_CELL: chtype = ' ' as u32;

pub struct GameWindows {
    stats: Window,
    view: Window,
    help: Window,
    log: Window,
}

impl GameWindows {
    fn refresh(&self) {
        self.stats.refresh();
        self.view.refresh();
        self.help.refresh();
        self.log.refresh();
    }
}

pub fn setup_render(window: &Window) -> GameWindows {
    start_color();
    use_default_colors();
    init_pair(DEFAULT_COLORS as i16, COLOR_WHITE, -1);
    init_pair(GOAL_COLORS as i16, COLOR_YELLOW, -1);
    init_pair(BROKEN_TURRET_COLORS as i16, COLOR_RED, -1);
    init_pair(DAMAGED_TURRET_COLORS as i16, COLOR_MAGENTA, -1);
    init_pair(PLACEMENT_COLORS as i16, COLOR_BLUE, -1);
    init_pair(GAMEOVER_COLORS as i16, COLOR_RED, -1);

    let stats = window.subwin(5, X as i32, 0, 0).unwrap();
    stats.keypad(true);
    stats.draw_box(0, 0);
    let view = window.subwin(Y as i32, X as i32, 5, 0).unwrap();
    view.keypad(true);
    let help = window.subwin(5 + Y as i32 + 7, 80 - X as i32, 0, X as i32).unwrap();
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
    pub fn render(&self, windows: &GameWindows, game_state: &GameState) {
        match *game_state {
            Construct { menu: menu, menu_index: menu_index } => {
                self.render_construct(windows, menu, menu_index)
            }
            Fight { .. } => self.render_fight(windows),
            GameOver { ref msg } => self.render_gameover(windows, msg),
            _ => unimplemented!(),
        };
    }

    pub fn render_construct(&self, windows: &GameWindows, menu: Menu, menu_index: usize) {
        windows.help.erase();
        windows.help.draw_box(0, 0);
        windows.help.mvaddstr(1, 1, "THING PROTECTOR");
        for row_n in 0..Y {
            for col_n in 0..X {
                let ch = self.statics[row_n][col_n]
                    .map_or(EMPTY_CELL, |s| self.render_static(row_n, s));
                windows.view.mvaddch(row_n as i32, col_n as i32, ch);
            }
        }

        match menu {
            Menu::Root => {
                windows.help.mvaddstr(3, 3, "Build");
                windows.help.mvaddstr(4, 3, "Move");
                windows.help.mvaddstr(5, 3, "Upgrade");
                windows.help.mvaddstr(6, 3, "Continue");
                windows.help.mvaddch(menu_index as i32 + 3, 2, '>');
                windows.help.mvaddch(menu_index as i32 + 3, 13, '<');
            }
            Menu::Build => {
                windows.help.mvaddstr(3, 3, "Turret");
                windows.help.mvaddstr(4, 3, "Obstacle");
                windows.help.mvaddstr(5, 3, "Back");
                windows.help.mvaddch(menu_index as i32 + 3, 2, '>');
                windows.help.mvaddch(menu_index as i32 + 3, 13, '<');
            }

            Menu::Place(placement) => {
                windows.help.mvaddstr(3, 3, "Placing a");
                windows.help.mvaddstr(4,
                                      3,
                                      match placement {
                                          Turret { .. } => "Turret",
                                          Obstacle { .. } => "Obstacle",
                                          _ => "Error",
                                      });
                windows.view.mvaddch(self.player_info.location.1 as i32,
                                     self.player_info.location.0 as i32,
                                     self.render_static(1, placement));

            }
            _ => unimplemented!(),
        }
        windows.refresh();
    }

    pub fn render_fight(&self, windows: &GameWindows) {
        for row_n in 0..Y {
            for col_n in 0..X {
                let ch = self.mobiles[row_n][col_n].map_or(self.statics[row_n][col_n]
                                                               .map_or(EMPTY_CELL, |s| {
                                                                   self.render_static(row_n, s)
                                                               }),
                                                           |m| self.render_mobile(m));
                windows.view.mvaddch(row_n as i32, col_n as i32, ch);
            }
        }
        let stat_string1 = format!("Health: {:3} | Thing Integrity: {:3} | Wave: {:3}",
                                   self.player_info.health,
                                   match self.statics[Y / 2][X / 2] {
                                       Some(Goal { health: h, .. }) => h,
                                       _ => 0,
                                   },
                                   1);
        let stat_string2 = format!("Cash: {:5}", self.cash);

        let offset = (X - stat_string1.len()) as i32 / 2;
        windows.stats.mvaddstr(2, offset, stat_string1.as_str());
        windows.stats.mvaddstr(3, offset, stat_string2.as_str());
        windows.stats.refresh();

        windows.view.refresh();

        windows.help.mvaddstr(1, 1, "THING PROTECTOR");
        windows.help.refresh();

        windows.log.clear();
        windows.log.draw_box(0, 0);
        for i in 0..self.log.len() {
            windows.log.mvaddstr(i as i32 + 1, 1, self.log[i].as_str());
        }

        windows.stats.refresh();
        windows.view.refresh();
        windows.help.refresh();
        windows.log.refresh();
    }

    pub fn render_gameover(&self, windows: &GameWindows, msg: &String) {
        let x = (X - msg.len() - 2) as i32 / 2;
        let y = (Y - 3) as i32 / 2 + 5;
        let gameover = windows.view.subwin(3, msg.len() as i32 + 2, y, x).unwrap();

        gameover.attron(COLOR_PAIR(GAMEOVER_COLORS));
        gameover.draw_box(0, 0);
        gameover.attron(A_BOLD);
        gameover.mvaddstr(1, 1, msg.as_str());
        gameover.attroff(A_BOLD | COLOR_PAIR(GAMEOVER_COLORS));

        gameover.refresh();
        windows.view.refresh();
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
