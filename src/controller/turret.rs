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
                    let magnitude = (dx as f64 * dx as f64 + dy as f64 * dy as f64).sqrt();
                    let arrow = Arrow {
                        info: ArrowInfo {
                            dx: (dx as f64 / magnitude * turret_info.arrow_speed as f64)
                                .trunc() as usize,
                            dy: (dy as f64 / magnitude * turret_info.arrow_speed as f64)
                                .trunc() as usize,
                            incx: incx,
                            incy: incy,
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
