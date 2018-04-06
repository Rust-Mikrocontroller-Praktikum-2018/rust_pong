pub enum Direction {
    Up,
    Down,
    None
}

pub trait Controller {
    fn start() -> bool;
    fn get_direction() -> Direction;
}