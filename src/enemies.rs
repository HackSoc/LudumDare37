use model::*;

pub fn kobold() -> Mobile {
    Fiend {
        info: FiendInfo {
            ch: 'k',
            form: (),
            health: 15,
            damage_factor: 3,
            armour_factor: 0,
            player_target_distance: 25,
            goal_target_distance: 10,
            turret_target_distance: 0,
            obstacle_target_distance: 0,
        },
    }
}
