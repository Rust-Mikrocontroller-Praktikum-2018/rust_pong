pub enum Direction {
    Up,
    Down,
    None
}

pub trait Controller {
    fn start(&self) -> bool;
    fn get_direction(&self) -> Direction;
}