use model::*;
use util::*;

impl WorldData {
    pub fn step_fiend(&mut self, old_xy: (usize, usize), fiend_info: FiendInfo) {
        let (old_x, old_y) = old_xy;
        let player_xy = self.player_info.location;
        let goal_xy = (X / 2, Y / 2);
        let turret_xy = find_nearest(&self.turrets, old_xy);
        let obstacle_xy = find_nearest(&self.obstacles, old_xy);

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
            Some(Wall) => return,
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
            _ => {} // we can move into an empty space, and also broken turrets and gates.
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
}
