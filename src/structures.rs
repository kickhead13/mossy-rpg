pub struct Position(pub u16, pub u16);
impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Position(x,y)
    }
}
