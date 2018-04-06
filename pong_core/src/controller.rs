enum Direction {
    Up,
    Down,
    None
}

trait Controller {
    fn start() -> bool;
    fn get_direction() -> Direction;
}