use pong_core::constants::{LCD_HEIGHT, LCD_WIDTH};
use pong_core::controller::Direction;
use pong_core::pong::Paddle;
use stm32f7::{touch, interrupts::primask_mutex::PrimaskMutex};
use stm32f7::i2c::I2C;
use core::cmp::Ordering;


pub enum Players {
    Player1 = 0,
    Player2 = 1,
}

pub struct DefaultController<'a> {
    pub player: Players,
    pub y_pos: i32,
    pub mutex: &'a PrimaskMutex<I2C>
}

impl<'a> DefaultController<'a> {

    pub fn new(player: Players, mutex: &'a PrimaskMutex<I2C>) -> DefaultController<'a> {
        DefaultController {
            player: player, 
            y_pos: (LCD_HEIGHT/2),
            mutex: mutex,
        }
    }

    pub fn valid_move(&self, x_pos: i32) -> bool {
        match self.player {
            Players::Player1 => (x_pos <= (LCD_WIDTH/2)),
            Players::Player2 => (x_pos > (LCD_WIDTH/2)),
        }
    }

    pub fn update_pos(&mut self, paddle: &Paddle) {
        self.y_pos = paddle.position.y;
    }

    pub fn start(&mut self) -> bool {
        let mut found: bool = false;

        self.mutex.lock(|i2c_3| {
            let moves = &mut touch::touches(i2c_3).unwrap();
            moves.retain(|touch| { self.valid_move(touch.x as i32) });
            found = moves.capacity() > 0;
        });

        found
    }

    pub fn get_direction(&mut self) -> Direction {
        let mut new_y_pos = self.y_pos;

        self.mutex.lock(|i2c_3| {
            for touch in &touch::touches(i2c_3).unwrap() {
                let action_x = touch.x as i32;
                let action_y = touch.y as i32;

                if self.valid_move(action_x) {
                    new_y_pos = action_y; 
                    break; 
                }
            }
        });

        match new_y_pos.cmp(&self.y_pos) {
            Ordering::Less => Direction::Down,
            Ordering::Equal => Direction::None,
            Ordering::Greater => Direction::Up,
        }
    }
}