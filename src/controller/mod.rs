mod arrow;
mod fiend;
mod turret;

use model::*;
use fiends::make_wave;

use pancurses::Input;
use pancurses::Input::*;

use rand::{Rng, thread_rng};

use std::cmp::min;
use std::collections::BTreeSet;

enum Dir {
    N,
    E,
    S,
    W,
}

impl Static {
    fn player_interact(&mut self, player_info: &PlayerInfo) {
        match *self {
            Wall | Gate => {}
            Obstacle { mut health, max_health } |
            Goal { mut health, max_health } |
            Turret { info: TurretInfo { mut health, max_health, .. } } => {
                health = min(health + player_info.heal_factor, max_health)
            }
        };
    }
}

impl GameState {
    pub fn handle(&mut self, world_data: &mut WorldData, i: Input) {
        match *self {
            Startup => *self = Construct,
            Construct => self.construct_handler(world_data, i),
            Fight { .. } => self.fight_handler(world_data, i),
            GameOver { .. } => self.gameover_handler(i),
            End => panic!("Should have ended and didn't!"),
        };
    }

    fn construct_handler(&mut self, world_data: &mut WorldData, i: Input) {
        match world_data.menu {
            Menu::Place => {
                match i {
                    KeyDown | Character('s') => {
                        world_data.player_info.location.1 += 1;
                        if world_data.player_info.location.1 == Y - 1 {
                            world_data.player_info.location.1 = Y - 2;
                        };
                    }
                    KeyUp | Character('w') => {
                        world_data.player_info.location.1 -= 1;
                        if world_data.player_info.location.1 == 0 {
                            world_data.player_info.location.1 = 1;
                        };
                    }
                    KeyLeft | Character('a') => {
                        world_data.player_info.location.0 -= 1;
                        if world_data.player_info.location.0 == 0 {
                            world_data.player_info.location.0 = 1;
                        };
                    }
                    KeyRight | Character('d') => {
                        world_data.player_info.location.0 += 1;
                        if world_data.player_info.location.0 == X - 1 {
                            world_data.player_info.location.0 = X - 2;
                        };
                    }
                    Character(' ') | Character('\n') => {
                        if world_data.statics[world_data.player_info
                                .location
                                .1][world_data.player_info.location.0]
                            .is_some() {
                            return;
                        }
                        world_data.statics[world_data.player_info.location.1][world_data.player_info
                            .location
                            .0] = world_data.placement;
                        match world_data.placement {
                            Some(Turret { .. }) => {
                                world_data.turrets.insert(world_data.player_info.location)
                            }
                            Some(Obstacle { .. }) => {
                                world_data.obstacles.insert(world_data.player_info.location)
                            }
                            _ => panic!("Lost placement???"),
                        };
                        world_data.placement = None;
                        world_data.menu = Menu::Root;
                    }
                    _ => {}
                }
            }
            Menu::Move2 => {}
            _ => {
                match i {
                    KeyDown | Character('s') => {
                        world_data.menu_index += 1;
                        world_data.menu_index %= world_data.current_menu_length();
                    }
                    KeyUp | Character('w') => {
                        world_data.menu_index = world_data.menu_index
                            .checked_sub(1)
                            .unwrap_or(world_data.current_menu_length() - 1);
                    }

                    Character('q') => *self = End,
                    KeyBackspace => {
                        world_data.menu = Menu::Root;
                        world_data.menu_index = 0;
                    }
                    Character(' ') | Character('\n') => {
                        match (world_data.menu, world_data.menu_index) {
                            (Menu::Root, 0) => {
                                world_data.menu = Menu::Build;
                                world_data.menu_index = 0;
                            }
                            (Menu::Root, 1) => {
                                world_data.menu = Menu::Move;
                                world_data.menu_index = 0;
                            }
                            (Menu::Root, 2) => {
                                world_data.menu = Menu::Upgrade;
                                world_data.menu_index = 0;
                            }
                            (Menu::Root, 3) => {
                                world_data.menu_index = 0;
                                world_data.wave += 1;
                                *self = Fight { to_spawn: make_wave(world_data.wave) };
                                world_data.player_info.location = (20, 20);
                            }
                            (Menu::Build, 0) => {
                                world_data.menu = Menu::Place;
                                world_data.menu_index = 0;
                                world_data.placement = Some(Turret {
                                    info: TurretInfo {
                                        form: (),
                                        cooldown: 0,
                                        max_cooldown: 3,
                                        range: 50,
                                        health: 100,
                                        max_health: 100,
                                        arrow_speed: 2,
                                        damage_factor: 300,
                                    },
                                });
                            }
                            (Menu::Build, 1) => {
                                world_data.menu = Menu::Place;
                                world_data.menu_index = 0;
                                world_data.placement = Some(Obstacle {
                                    health: 300,
                                    max_health: 300,
                                });
                            }
                            (Menu::Build, 2) => {
                                world_data.menu = Menu::Root;
                                world_data.menu_index = 0
                            }

                            _ => unimplemented!(),
                        }
                    }
                    _ => (),
                }
            }
        };
    }

    fn gameover_handler(&mut self, i: Input) {
        match i {
            Character('q') => {
                *self = End;
                return;
            }
            _ => {}
        }
    }

    fn fight_handler(&mut self, world_data: &mut WorldData, i: Input) {
        match i {
            KeyDown | Character('s') => {
                world_data.move_player(Dir::S);
            }
            KeyUp | Character('w') => {
                world_data.move_player(Dir::N);
            }
            KeyLeft | Character('a') => {
                world_data.move_player(Dir::W);
            }
            KeyRight | Character('d') => {
                world_data.move_player(Dir::E);
            }
            Character('q') => {
                *self = End;
                return;
            }
            _ => {}
        };

        // step fiends, turrets, and arrows.
        for fiend_xy in &world_data.fiends.clone() {
            match world_data.mobiles[fiend_xy.1][fiend_xy.0] {
                Some(Fiend { info }) => world_data.step_fiend(*fiend_xy, info),
                mob => {
                    panic!("({}, {}) is not a fiend (got {:?})!",
                           fiend_xy.0,
                           fiend_xy.1,
                           mob)
                }
            }
        }

        for turret_xy in &world_data.turrets.clone() {
            match world_data.statics[turret_xy.1][turret_xy.0] {
                Some(Turret { info }) => world_data.step_turret(*turret_xy, info),
                stat => {
                    panic!("({}, {}) is not a turret (got {:?})!",
                           turret_xy.0,
                           turret_xy.1,
                           stat)
                }
            }
        }

        for arrow_xy in &world_data.arrows.clone() {
            match world_data.mobiles[arrow_xy.1][arrow_xy.0] {
                Some(Arrow { info }) => world_data.step_arrow(*arrow_xy, info),
                mob => {
                    panic!("({}, {}) is not an arrow (got {:?})!",
                           arrow_xy.0,
                           arrow_xy.1,
                           mob)
                }
            }
        }

        // clean up dead mobs and obstacles.
        for fiend_xy in &world_data.fiends.clone() {
            match world_data.mobiles[fiend_xy.1][fiend_xy.0] {
                Some(Fiend { info }) if info.health == 0 => {
                    world_data.mobiles[fiend_xy.1][fiend_xy.0] = None;
                    world_data.fiends.remove(fiend_xy);
                }
                Some(Fiend { .. }) => {}
                mob => {
                    panic!("({}, {}) is not a fiend (got {:?})!",
                           fiend_xy.0,
                           fiend_xy.1,
                           mob)
                }
            }
        }

        for obstacle_xy in &world_data.obstacles.clone() {
            match world_data.statics[obstacle_xy.1][obstacle_xy.0] {
                Some(Obstacle { health, .. }) if health == 0 => {
                    world_data.statics[obstacle_xy.1][obstacle_xy.0] = None;
                    world_data.obstacles.remove(obstacle_xy);
                }
                Some(Obstacle { .. }) => {}
                stat => {
                    panic!("({}, {}) is not an obstacle (got {:?})!",
                           obstacle_xy.0,
                           obstacle_xy.1,
                           stat)
                }
            }
        }

        // spawn new fiends
        match *self {
            Fight { ref mut to_spawn } => spawn_fiends(world_data, to_spawn),
            _ => {}
        }

        // Check for game over
        if world_data.player_info.health == 0 {
            *self = GameOver { msg: "You have died!".to_string() }
        } else {
            match world_data.statics[world_data.goal_location.1][world_data.goal_location.0] {
                Some(Goal { health, .. }) if health == 0 => {
                    *self = GameOver { msg: "The Thing is destroyed!".to_string() }
                }
                _ => {}
            }
        }

        // Check for phase end
        if world_data.fiends.is_empty() {
            world_data.start_construct();
            *self = Construct;
        }
    }
}

impl WorldData {
    fn current_menu_length(&self) -> usize {
        match self.menu {
            Menu::Root => 4,
            Menu::Build => 3,
            Menu::Move => 1 + self.turrets.len() + self.obstacles.len(),
            Menu::Upgrade => 1 + self.turrets.len(),
            Menu::Continue => 0,
            Menu::Place => 0,
            Menu::Move2 => 0,
        }
    }

    fn move_player(&mut self, dir: Dir) {
        let old_x = self.player_info.location.0;
        let old_y = self.player_info.location.1;
        assert!(self.mobiles[old_y][old_x].map_or(false, |p| p.is_player()));
        let mut new_x = old_x;
        let mut new_y = old_y;
        match dir {
            Dir::N => new_y = old_y - 1,
            Dir::E => new_x = old_x + 1,
            Dir::S => new_y = old_y + 1,
            Dir::W => new_x = old_x - 1,
        };
        match self.statics[new_y][new_x] {
            // Broken turrets can be moved through.
            Some(Turret { info: TurretInfo { health, .. } }) if health == 0 => {}
            Some(mut sta) => {
                sta.player_interact(&self.player_info);
                self.statics[new_y][new_x] = Some(sta);
                return;
            }
            None => {} // we can move into an empty space
        };
        match self.mobiles[new_y][new_x] {
            Some(Arrow { .. }) => return,
            Some(Fiend { mut info }) => {
                let damage_factor = self.player_info.damage_factor;
                info.health = info.health.saturating_sub(damage_factor);
                self.attack(info, damage_factor);
                self.mobiles[new_y][new_x] = Some(Fiend { info: info });
                return;
            }
            Some(Player) => panic!("Player walked into themself"),
            None => {} // we can move into an empty space
        }
        self.player_info.location = (new_x, new_y);
        self.mobiles[old_y][old_x] = None;
        self.mobiles[new_y][new_x] = Some(Player);
        assert!(self.mobiles[new_y][new_x].map_or(false, |p| p.is_player()));
    }

    fn attack(&mut self, info: FiendInfo, damage_factor: usize) {
        if info.health == 0 {
            self.log_msg(format!("{} is hit for {} damage! (dead!)", info.name, damage_factor));
            self.cash += info.value;
        } else {
            self.log_msg(format!("{} is hit for {} damage! ({} / {})",
                                 info.name,
                                 damage_factor,
                                 info.health,
                                 info.max_health));
        }
    }

    fn start_construct(&mut self) {
        // Delete all arrows
        for &(x, y) in &self.arrows {
            self.mobiles[y][x] = None;
        }
        self.arrows = BTreeSet::new();

        // Heal turrets, obstacles, and player.
        for &(x, y) in &self.turrets {
            match self.statics[y][x] {
                Some(Turret { mut info }) => {
                    info.health = info.max_health;
                    self.statics[y][x] = Some(Turret { info: info });
                }
                _ => panic!("Not a turret!"),
            }
        }
        for &(x, y) in &self.obstacles {
            match self.statics[y][x] {
                Some(Obstacle { max_health, .. }) => {
                    self.statics[y][x] = Some(Obstacle {
                        health: max_health,
                        max_health: max_health,
                    });
                }
                _ => panic!("Not a turret!"),
            }
        }
        self.player_info.health = self.player_info.max_health;
    }
}

// Spawn as many fiends as possible.
fn spawn_fiends(world_data: &mut WorldData, to_spawn: &mut Vec<FiendInfo>) {
    if !to_spawn.is_empty() {
        let mut free_gates = BTreeSet::new();
        for gate_xy in &world_data.gates {
            if world_data.mobiles[gate_xy.1][gate_xy.0] == None {
                free_gates.insert(gate_xy);
            }
        }

        while !free_gates.is_empty() && !to_spawn.is_empty() {
            let mut gate_i = thread_rng().gen_range(0, free_gates.len());
            let mut gate = &(0, 0); // hack to remove "possibly uninitialised" errors.
            for g in &free_gates {
                if gate_i > 0 {
                    gate_i -= 1;
                    continue;
                }
                gate = *g;
                break;
            }
            let spawn_i = thread_rng().gen_range(0, to_spawn.len());
            let fiend = to_spawn[spawn_i];
            world_data.fiends.insert(*gate);
            world_data.mobiles[gate.1][gate.0] = Some(Fiend { info: fiend });
            to_spawn.swap_remove(spawn_i);
            free_gates.remove(gate);

            // Have to inline this, rather than use
            // 'world_data.log_msg' because it needs a mutable borrow
            // and 'world_data.gates' is borrowed immutably.
            let len = world_data.log.len();
            for i in 1..len {
                world_data.log[len - i] = world_data.log[len - i - 1].clone();
            }
            world_data.log[0] = format!("A {} appears!", fiend.name);
        }
    }
}
