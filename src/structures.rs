#[derive(Copy, Clone)]
pub struct Position(pub u16, pub u16);
impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Position(x,y)
    }
    pub fn set(&mut self, term_width: u16, term_height: u16) -> Self {
        self.0 = if self.0 <1 || self.0 > term_width-3 {0x1} else {self.0};
        self.1 = if self.1 <1 || self.1 > term_height-2 {0x1} else {self.1};
        *self
    }
}

#[allow(dead_code)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT
}

#[allow(dead_code)]
pub fn basic_attack(_direction: Direction) {
    //does nothing
}

#[allow(dead_code)]
pub struct Attack {
    pub sprite: String,
    pub attack_pos: Position,
    pub life_time: u32,
    pub attack: Box<dyn Fn(&mut Attack, Direction) -> ()>
}

#[allow(dead_code)]
impl Attack {
    pub fn new( sprite: String,
                attack_pos: Position,
                life_time: u32,
                attack: Box<dyn Fn(&mut Attack, Direction) -> ()>
    ) -> Self {
        Self {
            sprite: sprite,
            attack_pos: attack_pos,
            life_time: life_time,
            attack: attack
        }
    }
}
