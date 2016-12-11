mod arrow;
mod fiend;
mod turret;

use model::*;
use controller::arrow::*;
use controller::fiend::*;
use controller::turret::*;

use pancurses::Input;
use pancurses::Input::*;

use std::cmp::min;

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
            Startup => *self = Build,
            Build => unimplemented!(),
            Fight => self.fight_handler(world_data, i),
            GameOver => unimplemented!(),
            End => panic!("Should have ended and didn't!"),
        };
    }

    fn fight_handler(&mut self, world_data: &mut WorldData, i: Input) {
        let do_next = match i {
            KeyDown | Character('s') => {
                world_data.move_player(Dir::S);
                true
            }
            KeyUp | Character('w') => {
                world_data.move_player(Dir::N);
                true
            }
            KeyLeft | Character('a') => {
                world_data.move_player(Dir::W);
                true
            }
            KeyRight | Character('d') => {
                world_data.move_player(Dir::E);
                true
            }
            Character('q') => {
                *self = End;
                false
            }
            _ => false,
        };

        if do_next {
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
        }
    }
}

impl WorldData {
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
                info.health = info.health.saturating_sub(self.player_info.damage_factor);
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
}
