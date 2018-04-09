pub enum Direction {
    Up = 1,
    Down = -1,
    None = 0
}

pub trait Controller {
    fn start(&mut self) -> bool;
    fn get_direction(&mut self) -> Direction;
}