use model::*;

use pancurses::Input;
use pancurses::Input::*;

use std::cmp::{min, max};
use std::collections::LinkedList;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Startup,
    Build,
    Fight,
    GameOver,
    End,
}

pub use self::GameState::*;

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
            Turret { mut health, max_health, .. } => {
                health = min(health + player_info.heal_factor, max_health)
            }
        };
    }

    fn fiend_interact(&mut self, fiend_info: &FiendInfo) {
        match *self {
            Wall | Gate => {}
            Obstacle { mut health, .. } |
            Goal { mut health, .. } |
            Turret { mut health, .. } => health = health.saturating_sub(fiend_info.damage_factor),
        }
    }
}

impl Mobile {
    fn player_interact(&mut self, player_info: &PlayerInfo) {
        match *self {
            Arrow { .. } => {}
            Fiend { mut info } => {
                info.health = info.health.saturating_sub(player_info.damage_factor)
            }
            Player => panic!("Player walked into themself"),
        };
    }

    fn fiend_interact(&mut self, fiend_info: &FiendInfo, player_info: &PlayerInfo) {
        match *self {
            Arrow { .. } => {}
            Fiend { .. } => {}
            Player => Fiend { info: *fiend_info }.player_interact(player_info),
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
        let do_fiends = match i {
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

        if do_fiends {
            let mut fiends = LinkedList::new();
            for x in 0..X {
                for y in 0..Y {
                    if let Some(Fiend { info }) = world_data.mobiles[y][x] {
                        fiends.push_back(((x, y), info))
                    }
                }
            }
            for fiend in fiends.iter_mut() {
                world_data.step_fiend(fiend.0, fiend.1)
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
            Some(mut sta) => {
                sta.player_interact(&self.player_info);
                return;
            }
            None => {} // we can move into an empty space
        };
        match self.mobiles[new_y][new_x] {
            Some(mut mob) => {
                mob.player_interact(&self.player_info);
                return;
            }
            None => {} // we can move into an empty space
        }
        self.player_info.location = (new_x, new_y);
        self.mobiles[old_y][old_x] = None;
        self.mobiles[new_y][new_x] = Some(Player);
        assert!(self.mobiles[new_y][new_x].map_or(false, |p| p.is_player()));
    }

    fn step_fiend(&mut self, old_xy: (usize, usize), fiend_info: FiendInfo) {
        let (old_x, old_y) = old_xy;
        let player_xy = self.player_info.location;
        let goal_xy = (X / 2, Y / 2);
        let turret_xy = self.find_turret(old_xy);
        let obstacle_xy = self.find_obstacle(old_xy);

        let (target_x, target_y) = match (turret_xy, obstacle_xy) {
            _ if distance(old_xy, goal_xy) <= fiend_info.goal_target_distance as f64 => {
                goal_xy // move towards goal
            }
            _ if distance(old_xy, player_xy) <= fiend_info.player_target_distance as f64 => {
                player_xy // move towards player
            }
            (Some(xy), _) if distance(old_xy, xy) <= fiend_info.turret_target_distance as f64 => {
                xy // move towards turret
            }
            (_, Some(xy)) if distance(old_xy, xy) <= fiend_info.obstacle_target_distance as f64 => {
                xy // move towards obstacle
            }
            _ => {
                goal_xy // move towards goal if no better options
            }
        };

        // move directly towards the target
        let new_x = if target_x < old_x {
            old_x - 1
        } else if target_x > old_x {
            old_x + 1
        } else {
            old_x
        };
        let new_y = if target_y < old_y {
            old_y - 1
        } else if target_y > old_y {
            old_y + 1
        } else {
            old_y
        };

        match self.statics[new_y][new_x] {
            Some(mut sta) => {
                sta.fiend_interact(&fiend_info);
                return;
            }
            None => {} // we can move into an empty space
        };
        match self.mobiles[new_y][new_x] {
            Some(mut mob) => {
                mob.fiend_interact(&fiend_info, &self.player_info);
                return;
            }
            None => {} // we can move into an empty space
        }
        self.mobiles[old_y][old_x] = None;
        self.mobiles[new_y][new_x] = Some(Fiend { info: fiend_info });
    }

    fn find_obstacle(&self, my_xy: (usize, usize)) -> Option<(usize, usize)> {
        fn obstacle_predicate(world: &WorldData, (x, y): (usize, usize)) -> bool {
            match world.statics[y][x] {
                Some(Obstacle { .. }) => true,
                _ => false,
            }
        };
        self.find_nearest(obstacle_predicate, my_xy)
    }

    fn find_turret(&self, my_xy: (usize, usize)) -> Option<(usize, usize)> {
        fn turret_predicate(world: &WorldData, (x, y): (usize, usize)) -> bool {
            match world.statics[y][x] {
                Some(Turret { .. }) => true,
                _ => false,
            }
        };
        self.find_nearest(turret_predicate, my_xy)
    }

    fn find_nearest(&self,
                    predicate: fn(&WorldData, (usize, usize)) -> bool,
                    my_xy: (usize, usize))
                    -> Option<(usize, usize)> {
        // Much better would be to do some sort of moving out from the
        // starting coordinates and stopping at the first found.
        let mut found_xy = None;
        let mut dist = 0.0;
        for x in 0..X {
            for y in 0..Y {
                if !predicate(self, (x, y)) {
                    continue;
                }

                match found_xy {
                    Some(xy) => {
                        let newdist = distance(xy, (x, y));
                        if newdist < dist {
                            found_xy = Some((x, y));
                            dist = newdist;
                        }
                    }
                    None => {
                        found_xy = Some((x, y));
                        dist = distance(my_xy, (x, y));
                    }
                }
            }
        }

        return found_xy;
    }
}

// currently this is euclidean distance, but really it should be walking distance
fn distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> f64 {
    let dx = x1 as f64 - x2 as f64;
    let dy = y1 as f64 - y2 as f64;
    return (dx * dx + dy * dy).sqrt();
}
