use model::*;

use pancurses::Input;
use pancurses::Input::*;

use std::cmp::{min, max};
use std::collections::LinkedList;

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
            Turret { info: TurretInfo { mut health, max_health, .. } } => {
                health = min(health + player_info.heal_factor, max_health)
            }
        };
    }

    fn fiend_interact(&mut self, fiend_info: &FiendInfo) {
        match *self {
            Wall | Gate => {}
            Obstacle { mut health, .. } |
            Goal { mut health, .. } |
            Turret { info: TurretInfo { mut health, .. } } => {
                health = health.saturating_sub(fiend_info.damage_factor)
            }
        }
    }
}

impl Mobile {
    fn fiend_interact(&mut self, fiend_info: &FiendInfo, player_info: &mut PlayerInfo) {
        match *self {
            Arrow { .. } => {}
            Fiend { .. } => {}
            Player => {
                player_info.health = player_info.health.saturating_sub(fiend_info.damage_factor)
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
            // find all fiends, turrets, and arrows (todo: just have
            // these in a list directly, rather than the current
            // matrix form?)
            let mut fiends = LinkedList::new();
            let mut turrets = LinkedList::new();
            let mut arrows = LinkedList::new();
            for x in 0..X {
                for y in 0..Y {
                    if let Some(Fiend { info }) = world_data.mobiles[y][x] {
                        fiends.push_back(((x, y), info))
                    }
                    if let Some(Turret { info }) = world_data.statics[y][x] {
                        turrets.push_back(((x, y), info))
                    }
                    if let Some(Arrow { info }) = world_data.mobiles[y][x] {
                        arrows.push_back(((x, y), info))
                    }
                }
            }

            // step everything.
            for fiend in fiends.iter_mut() {
                world_data.step_fiend(fiend.0, fiend.1)
            }

            for turret in turrets.iter_mut() {
                world_data.step_turret(turret.0, turret.1)
            }

            for arrow in arrows.iter_mut() {
                world_data.step_arrow(arrow.0, arrow.1)
            }

            // clean up dead mobs.
            for x in 0..X {
                for y in 0..Y {
                    match world_data.mobiles[y][x] {
                        Some(Fiend { info }) if info.health == 0 => world_data.mobiles[y][x] = None,
                        _ => {}
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
                self.mobiles[new_y][new_x] = Some(Fiend{info:info});
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
                mob.fiend_interact(&fiend_info, &mut self.player_info);
                return;
            }
            None => {} // we can move into an empty space
        }
        self.mobiles[old_y][old_x] = None;
        self.mobiles[new_y][new_x] = Some(Fiend { info: fiend_info });
    }

    fn step_turret(&mut self, xy: (usize, usize), turret_info: TurretInfo) {
        let (x, y) = xy;
        let mut new_turret_info = turret_info;

        if turret_info.cooldown != 0 {
            new_turret_info.cooldown -= 1
        } else {
            if self.mobiles[y][x].is_some() {
                return;
            }

            match self.find_fiend(xy) {
                Some(fiend_xy) if distance(xy, fiend_xy) <= turret_info.range as f64 => {
                    let (fiend_x, fiend_y) = fiend_xy;
                    let (dx, incx) = make_delta(x, fiend_x);
                    let (dy, incy) = make_delta(y, fiend_y);
                    let magnitude = (dx as f64 * dx as f64 + dy as f64 * dy as f64).sqrt();
                    let arrow = Arrow {
                        info: ArrowInfo {
                            dx: (dx as f64 / magnitude * turret_info.arrow_speed as f64)
                                .trunc() as u8,
                            dy: (dy as f64 / magnitude * turret_info.arrow_speed as f64)
                                .trunc() as u8,
                            incx: incx,
                            incy: incy,
                            damage_factor: 300,
                        },
                    };
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
                        self.mobiles[y][x] = Some(Fiend{info:info});
                        false
                    }
                    (_, Some(_)) => false,
                    _ => true,
                }
            };

            if arrow_info.dx == 0 {
                for _ in 0..arrow_info.dy {
                    y = if arrow_info.incy { y + 1 } else { y - 1 };
                    if !go((x, y)) {
                        return;
                    }
                }
            } else if arrow_info.dy == 0 {
                for _ in 0..arrow_info.dx {
                    x = if arrow_info.incx { x + 1 } else { x - 1 };
                    if !go((x, y)) {
                        return;
                    }
                }
            } else {
                // Bresenham's line algorithm
                let (counter, mut err, err_inc, err_dec, inc, correction): (u8, i32, i32, i32, (i8, i8), (i8, i8)) = if arrow_info.dx > arrow_info.dy {
                    (arrow_info.dx,
                     arrow_info.dy as i32 * 2 - arrow_info.dx as i32,
                     arrow_info.dy as i32 * 2,
                     arrow_info.dx as i32 * 2,
                     if arrow_info.incx { (1, 0) } else { (-1, 0) },
                     if arrow_info.incy { (0, 1) } else { (0, -1) })
                } else {
                    (arrow_info.dy,
                     arrow_info.dx as i32 * 2 - arrow_info.dy as i32,
                     arrow_info.dx as i32 * 2,
                     arrow_info.dy as i32 * 2,
                     if arrow_info.incy { (0, 1) } else { (0, -1) },
                     if arrow_info.incx { (1, 0) } else { (-1, 0) })
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

        self.mobiles[y][x] = arrow;
    }

    fn find_fiend(&self, my_xy: (usize, usize)) -> Option<(usize, usize)> {
        let fiend_predicate = |(x, y): (usize, usize)| {
            match self.mobiles[y][x] {
                Some(Fiend { .. }) => true,
                _ => false,
            }
        };
        find_nearest(fiend_predicate, my_xy)
    }

    fn find_obstacle(&self, my_xy: (usize, usize)) -> Option<(usize, usize)> {
        let obstacle_predicate = |(x, y): (usize, usize)| {
            match self.statics[y][x] {
                Some(Obstacle { .. }) => true,
                _ => false,
            }
        };
        find_nearest(obstacle_predicate, my_xy)
    }

    fn find_turret(&self, my_xy: (usize, usize)) -> Option<(usize, usize)> {
        let turret_predicate = |(x, y): (usize, usize)| {
            match self.statics[y][x] {
                Some(Turret { .. }) => true,
                _ => false,
            }
        };
        find_nearest(turret_predicate, my_xy)
    }
}

fn find_nearest<F>(predicate: F,
                my_xy: (usize, usize))
                -> Option<(usize, usize)>
    where F: Fn((usize, usize)) -> bool
{
    // Much better would be to do some sort of moving out from the
    // starting coordinates and stopping at the first found.
    let mut found_xy = None;
    let mut dist = 0.0;
    for x in 0..X {
        for y in 0..Y {
            if !predicate((x, y)) {
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

// currently this is euclidean distance, but really it should be walking distance
fn distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> f64 {
    let dx = x1 as f64 - x2 as f64;
    let dy = y1 as f64 - y2 as f64;
    return (dx * dx + dy * dy).sqrt();
}

fn make_delta(start: usize, end: usize) -> (usize, bool) {
    if start < end {
        (end - start, true)
    } else {
        (start - end, false)
    }
}
