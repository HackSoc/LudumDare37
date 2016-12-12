use model::*;
use util::*;

impl WorldData {
    pub fn step_turret(&mut self, xy: (usize, usize), turret_info: TurretInfo) {
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

            match find_nearest(&self.fiends, xy) {
                Some(fiend_xy) if distance(xy, fiend_xy) <= turret_info.range as usize => {
                    let (fiend_x, fiend_y) = fiend_xy;
                    let (dx, incx) = make_delta(x, fiend_x);
                    let (dy, incy) = make_delta(y, fiend_y);
                    let arrow = Arrow {
                        info: ArrowInfo {
                            // Vector
                            dx: dx,
                            dy: dy,
                            incx: if dx > dy {
                                if incx { 1 } else { -1 }
                            } else {
                                0
                            },
                            incy: if dx > dy {
                                0
                            } else {
                                if incy { 1 } else { -1 }
                            },
                            speed: turret_info.arrow_speed,
                            err: if dx > dy {
                                dy as i32 * 2 - dx as i32
                            } else {
                                dx as i32 * 2 - dy as i32
                            },
                            err_inc: if dx > dy {
                                dy as i32 * 2
                            } else {
                                dx as i32 * 2
                            },
                            err_dec: if dx > dy {
                                dx as i32 * 2
                            } else {
                                dy as i32 * 2
                            },
                            corrx: if dx > dy {
                                0
                            } else {
                                if incx { 1 } else { -1 }
                            },
                            corry: if dx > dy {
                                if incy { 1 } else { -1 }
                            } else {
                                0
                            },
                            damage_factor: turret_info.damage_factor,
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
}

fn make_delta(start: usize, end: usize) -> (usize, bool) {
    if start < end {
        (end - start, true)
    } else {
        (start - end, false)
    }
}
