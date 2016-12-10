pub const X: usize = 63;
pub const Y: usize = 31;

#[derive(Clone, Copy)]
pub enum Static {
    Wall,
    Gate,
    Goal,
    Turret,
    Obstacle,
}

#[derive(Clone, Copy)]
pub enum Mobile {
    Player,
    Fiend,
    Arrow,
}

pub static mut statics: [[Option<Static>; X]; Y] = [[None; X]; Y];
pub static mut mobiles: [[Option<Mobile>; X]; Y] = [[None; X]; Y];
