use model::*;
use util::*;

use astar::*;
use std::usize;
use std::vec::IntoIter;

impl WorldData {
    pub fn step_fiend(&mut self, old_xy: (usize, usize), fiend_info: FiendInfo) {
        let (old_x, old_y) = old_xy;
        let player_xy = self.player_info.location;
        let goal_xy = (X / 2, Y / 2);
        let turret_xy = find_nearest(&self.turrets, old_xy);
        let obstacle_xy = find_nearest(&self.obstacles, old_xy);

        let target_xy = match (turret_xy, obstacle_xy) {
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

        // Find the next step.
        let (new_x, new_y) = self.pathfind(old_xy, target_xy, fiend_info.damage_factor);

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

    // A* search, with the following special costs:
    //
    // - The cost of walking through an obstacle or turret is 1 + the
    //   number of turns taken to destroy it
    //
    //  - The cost of walking through another fiend is 2 (1 turn for
    //  it to move away, then 1 turn to move to the space).
    //
    // Returns the first step along the path.
    fn pathfind(&self,
                my_xy: (usize, usize),
                target_xy: (usize, usize),
                damage_factor: usize)
                -> (usize, usize) {
        let mut searcher = WorldSearch {
            world_data: self,
            start: my_xy,
            end: target_xy,
            damage_factor: damage_factor,
        };
        let path = astar(&mut searcher);
        *path.expect("No path found!").get(1).expect("No path found!")
    }
}

// A* implementation
struct WorldSearch<'a> {
    world_data: &'a WorldData,
    start: (usize, usize),
    end: (usize, usize),
    damage_factor: usize,
}

impl<'a> SearchProblem for WorldSearch<'a> {
    type Node = (usize, usize);
    type Cost = usize;
    type Iter = IntoIter<((usize, usize), usize)>;

    fn start(&self) -> (usize, usize) {
        self.start
    }
    fn is_end(&self, p: &(usize, usize)) -> bool {
        *p == self.end
    }
    fn heuristic(&self, &(p_x, p_y): &(usize, usize)) -> usize {
        let (s_x, s_y) = self.end;
        (s_x.saturating_sub(p_x)).saturating_add(s_y.saturating_sub(p_y))
    }
    fn neighbors(&mut self, position: &(usize, usize)) -> IntoIter<((usize, usize), usize)> {
        let mut vec = vec![];
        for (x, y) in adjacency(*position) {
            let mcost = match (self.world_data.statics[y][x], self.world_data.mobiles[y][x]) {
                (Some(Wall), _) => None,
                (Some(Turret { info: TurretInfo { health, .. } }), _) |
                (Some(Obstacle { health, .. }), _) |
                (Some(Goal { health, .. }), _) => Some(health / self.damage_factor),
                (_, Some(Player)) => Some(self.world_data.player_info.health / self.damage_factor),
                (_, Some(Fiend { .. })) => Some(1),
                _ => Some(0),
            };
            match mcost {
                Some(cost) => vec.push(((x, y), cost.saturating_add(1))),
                None => {}
            }
        }
        vec.into_iter()
    }
}
