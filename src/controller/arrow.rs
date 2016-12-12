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

            // Bresenham's line algorithm
            for _ in 0..info.speed {
                if info.err >= 0 {
                    info.err -= info.err_dec;
                    x = signed_add(x, info.corrx);
                    y = signed_add(y, info.corry);
                }
                info.err += info.err_inc;
                x = signed_add(x, info.incx);
                y = signed_add(y, info.incy);
                if !go((x, y)) {
                    return;
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
