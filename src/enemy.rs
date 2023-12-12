use crate::structures;
use crate::character;

#[allow(dead_code)]
pub struct Enemy {
    pub sprite: structures::Sprite,
    pub movement: Box<dyn Fn(&mut Enemy) -> ()>,
    pub attack: Option<structures::Attack>,
    pub enemy_pos: structures::Position,
    pub player_pos: structures::Position
}

