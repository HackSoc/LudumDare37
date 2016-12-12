use model::*;

impl WorldData {
    pub fn step_arrow(&mut self, (old_x, old_y): (usize, usize), arrow_info: ArrowInfo) {
        let mut x = old_x;
        let mut y = old_y;
        let mut info = arrow_info;

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
                        self.shoot(info, arrow_info.damage_factor);
                        self.mobiles[y][x] = Some(Fiend { info: info });
                        false
                    }
                    (_, Some(_)) => false,
                    _ => true,
                }
            };

            let dx = info.dx;
            let dy = info.dy;
            let incx = info.incx;
            let incy = info.incy;
            let speed = info.speed;

            if dx == 0 {
                for _ in 0..speed {
                    y = if incy { y + 1 } else { y - 1 };
                    if !go((x, y)) {
                        return;
                    }
                }
            } else if dy == 0 {
                for _ in 0..speed {
                    x = if incx { x + 1 } else { x - 1 };
                    if !go((x, y)) {
                        return;
                    }
                }
            } else {
                // Bresenham's line algorithm
                let gdx = dx > dy;
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
                for _ in 0..speed {
                    if info.err >= 0 {
                        info.err -= err_dec;
                        x = signed_add(x, correction.0);
                        y = signed_add(y, correction.1);
                    }
                    info.err += err_inc;
                    x = signed_add(x, inc.0);
                    y = signed_add(y, inc.1);
                    if !go((x, y)) {
                        return;
                    }
                }
            }
        }

        self.arrows.insert((x, y));
        self.mobiles[y][x] = Some(Arrow { info: info });
    }

    fn shoot(&mut self, info: FiendInfo, damage_factor: usize) {
        if info.health == 0 {
            self.log_msg(format!("{} is shot for {} damage! (dead!)",
                                 info.name,
                                 damage_factor));
            self.cash += info.value;
        } else {
            self.log_msg(format!("{} is shot for {} damage! ({} / {})",
                                 info.name,
                                 damage_factor,
                                 info.health,
                                 info.max_health));
        }
    }
}

// I felt like making this a macro
fn signed_add(u: usize, s: i8) -> usize {
    if s < 0 {
        u.saturating_sub(s.abs() as usize)
    } else {
        u.saturating_add(s.abs() as usize)
    }
}
