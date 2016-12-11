use model::*;

use pancurses::Input;
use pancurses::Input::*;

use std::cmp::{min, max};
use std::collections::BTreeSet;
use std::ops::Sub;

// I felt like making this a macro
macro_rules! signed_add {
    ($u:expr, $s:expr, $t:ty) => {{
        let u = $u;
        let s = $s;
        if s < 0 {
            u.saturating_sub(s.abs() as $t)
        } else {
            u.saturating_add(s.abs() as $t)
        }
    }}
}

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

    fn step_fiend(&mut self, old_xy: (usize, usize), fiend_info: FiendInfo) {
        let (old_x, old_y) = old_xy;
        let player_xy = self.player_info.location;
        let goal_xy = (X / 2, Y / 2);
        let turret_xy = self.find_turret(old_xy);
        let obstacle_xy = self.find_obstacle(old_xy);

        let (target_x, target_y) = match (turret_xy, obstacle_xy) {
            _ if distance(old_xy, goal_xy) <= fiend_info.goal_target_distance => {
                goal_xy // move towards goal
            }
            _ if distance(old_xy, player_xy) <= fiend_info.player_target_distance => {
                player_xy // move towards player
            }
            (Some(xy), _) if distance(old_xy, xy) <= fiend_info.turret_target_distance => {
                xy // move towards turret
            }
            (_, Some(xy)) if distance(old_xy, xy) <= fiend_info.obstacle_target_distance => {
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
            Some(Wall) | Some(Gate) => return,
            Some(Obstacle { health, max_health }) => {
                self.statics[new_y][new_x] = Some(Obstacle {
                    health: health.saturating_sub(fiend_info.damage_factor),
                    max_health: max_health,
                });
                return;
            }
            Some(Goal { health, max_health }) => {
                self.statics[new_y][new_x] = Some(Goal {
                    health: health.saturating_sub(fiend_info.damage_factor),
                    max_health: max_health,
                });
                return;
            }
            Some(Turret { mut info }) if info.health > 0 => {
                info.health = info.health.saturating_sub(fiend_info.damage_factor);
                self.statics[new_y][new_x] = Some(Turret { info: info });
                return;
            }
            Some(Turret { .. }) => {} // a broken turret can be bypassed
            None => {} // we can move into an empty space
        };
        match self.mobiles[new_y][new_x] {
            Some(Arrow { .. }) => return, // TODO: be damaged
            Some(Fiend { .. }) => return, // TODO: try moving elsewhere
            Some(Player) => {
                self.player_info.health =
                    self.player_info.health.saturating_sub(fiend_info.damage_factor);
                return;
            }
            None => {} // we can move into an empty space
        }
        self.fiends.remove(&(old_x, old_y));
        self.fiends.insert((new_x, new_y));
        self.mobiles[old_y][old_x] = None;
        self.mobiles[new_y][new_x] = Some(Fiend { info: fiend_info });
    }

    fn step_turret(&mut self, xy: (usize, usize), turret_info: TurretInfo) {
        let (x, y) = xy;
        let mut new_turret_info = turret_info;

        if turret_info.health == 0 {
            return;
        }

        if turret_info.cooldown != 0 {
            new_turret_info.cooldown -= 1
        } else {
            if self.mobiles[y][x].is_some() {
                return;
            }

            match self.find_fiend(xy) {
                Some(fiend_xy) if distance(xy, fiend_xy) <= turret_info.range as usize => {
                    let (fiend_x, fiend_y) = fiend_xy;
                    let (dx, incx) = make_delta(x, fiend_x);
                    let (dy, incy) = make_delta(y, fiend_y);
                    let magnitude = (dx as f64 * dx as f64 + dy as f64 * dy as f64).sqrt();
                    let arrow = Arrow {
                        info: ArrowInfo {
                            dx: (dx as f64 / magnitude * turret_info.arrow_speed as f64)
                                .trunc() as usize,
                            dy: (dy as f64 / magnitude * turret_info.arrow_speed as f64)
                                .trunc() as usize,
                            incx: incx,
                            incy: incy,
                            damage_factor: 300,
                        },
                    };
                    self.arrows.insert((x, y));
                    self.mobiles[y][x] = Some(arrow);
                    new_turret_info.cooldown = turret_info.max_cooldown;
                }
                _ => {}
            };
        }

        self.statics[y][x] = Some(Turret { info: new_turret_info });
    }

    fn step_arrow(&mut self, (old_x, old_y): (usize, usize), arrow_info: ArrowInfo) {
        let arrow = self.mobiles[old_y][old_x];

        let mut x = old_x;
        let mut y = old_y;

        self.mobiles[y][x] = None;
        self.arrows.remove(&(x, y));

        // Would be nice to avoid this extra scope...
        {
            // returns 'true' if movement should continue ('false' if
            // the arrow hits something and gets destroyed)
            //
            // I am not sure why this needs to be mut.
            let mut go = |(x, y): (usize, usize)| {
                match (self.statics[y][x], self.mobiles[y][x]) {
                    (Some(Wall), _) => false,
                    (Some(Gate), _) => false,
                    (_, Some(Fiend { mut info })) => {
                        info.health = info.health.saturating_sub(arrow_info.damage_factor);
                        self.mobiles[y][x] = Some(Fiend { info: info });
                        false
                    }
                    (_, Some(_)) => false,
                    _ => true,
                }
            };

            let dx = arrow_info.dx;
            let dy = arrow_info.dy;
            let incx = arrow_info.incx;
            let incy = arrow_info.incy;

            if dx == 0 {
                for _ in 0..dy {
                    y = if incy { y + 1 } else { y - 1 };
                    if !go((x, y)) {
                        return;
                    }
                }
            } else if dy == 0 {
                for _ in 0..dx {
                    x = if incx { x + 1 } else { x - 1 };
                    if !go((x, y)) {
                        return;
                    }
                }
            } else {
                // Bresenham's line algorithm
                let gdx = dx > dy;
                let counter = if gdx { dx } else { dy };
                let mut err: i32 = if gdx {
                    dy as i32 * 2 - dx as i32
                } else {
                    dx as i32 * 2 - dy as i32
                };
                let err_inc: i32 = if gdx { dy as i32 * 2 } else { dx as i32 * 2 };
                let err_dec: i32 = if gdx { dx as i32 * 2 } else { dy as i32 * 2 };
                let inc: (i8, i8) = if gdx {
                    if incx { (1, 0) } else { (-1, 0) }
                } else {
                    if incy { (0, 1) } else { (0, -1) }
                };
                let correction: (i8, i8) = if gdx {
                    if incy { (0, 1) } else { (0, -1) }
                } else {
                    if incx { (1, 0) } else { (-1, 0) }
                };
                for _ in 0..counter {
                    if err >= 0 {
                        err -= err_dec;
                        x = signed_add!(x, correction.0, usize);
                        y = signed_add!(y, correction.1, usize);
                    }
                    err += err_inc;
                    x = signed_add!(x, inc.0, usize);
                    y = signed_add!(y, inc.1, usize);
                    if !go((x, y)) {
                        return;
                    }
                }
            }
        }

        self.arrows.insert((x, y));
        self.mobiles[y][x] = arrow;
    }

    fn find_fiend(&self, my_xy: (usize, usize)) -> Option<(usize, usize)> {
        find_nearest(&self.fiends, my_xy)
    }

    fn find_obstacle(&self, my_xy: (usize, usize)) -> Option<(usize, usize)> {
        find_nearest(&self.obstacles, my_xy)
    }

    fn find_turret(&self, my_xy: (usize, usize)) -> Option<(usize, usize)> {
        find_nearest(&self.turrets, my_xy)
    }
}

fn find_nearest(points: &BTreeSet<(usize, usize)>,
                my_xy: (usize, usize))
                -> Option<(usize, usize)> {
    let mut found_xy = None;
    let mut dist = 0;
    for xy in points {
        match found_xy {
            Some(xy) => {
                let newdist = distance(xy, my_xy);
                if newdist < dist {
                    found_xy = Some(xy);
                    dist = newdist;
                }
            }
            None => {
                found_xy = Some(*xy);
                dist = distance(*xy, my_xy);
            }
        }
    }

    return found_xy;
}

// implements Chebyshev distance https://en.wikipedia.org/wiki/Chebyshev_distance
fn distance<T>((x1, y1): (T, T), (x2, y2): (T, T)) -> T::Output
    where T: Sub + Ord + Copy,
          <T as Sub>::Output: Ord
{
    let dx = max(x1, x2) - min(x1, x2);
    let dy = max(y1, y2) - min(y1, y2);
    return max(dx, dy);
}

fn make_delta(start: usize, end: usize) -> (usize, bool) {
    if start < end {
        (end - start, true)
    } else {
        (start - end, false)
    }
}
