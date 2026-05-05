use crate::print;
use crate::vga::{erase, food_to_vga, snake_to_vga};
use alloc::collections::VecDeque;

use spin::lazy::Lazy;

use crate::{QEMU_FAIL, exit_qemu};

const HEIGHT: u32 = 25;
const WIDTH: u32 = 80;

pub static mut SNAKE_ON: bool = false;
pub static mut FOOD: (u32, u32) = (12, 14);

pub static mut ERASE: (u32, u32) = (0, 0);

pub static mut SNAKE: Snake = Snake {
    body: VecDeque::new(),
};

pub fn init_snake() {
    unsafe {
        SNAKE.body.reserve_exact(25 * 80);
        SNAKE.body.push_front((17, 40));
    }
}

pub fn update_apple() {
    unsafe {
        let rand_x = (FOOD.0 * 97) % 25;
        let rand_y = (FOOD.1 * 97 * 97) % 80;
        FOOD = (rand_x, rand_y);
    }
}

pub struct Snake {
    body: VecDeque<(u32, u32)>,
}

impl Snake {
    pub fn update_position(&mut self, direction: char) {
        unsafe { food_to_vga(FOOD.0, FOOD.1) };
        let (mut head_r, mut head_c) = self.body[0];
        let mut new_head = (0, 0);

        //Gets the direction which the head of the snake should update in
        match direction {
            'a' => new_head = (head_r, head_c - 1),
            'w' => new_head = (head_r - 1, head_c),
            'd' => new_head = (head_r, head_c + 1),
            's' => new_head = (head_r + 1, head_c),
            _ => (),
        }

        //Check if the updated snake position will hit a wall or itself -> itself todo
        if self.check_collision(new_head) == true {
            exit_qemu(QEMU_FAIL);
        }

        if unsafe { FOOD == new_head } {
            self.body.push_front(new_head);
            unsafe {
                erase(FOOD.0, FOOD.1);
                update_apple();
            }
        } else {
            self.body.push_front(new_head);
            unsafe { ERASE = self.body.pop_back().expect("REASON") };
        }

        unsafe {
            for (index, coord) in self.body.iter().enumerate() {
                if index == 0 {
                    snake_to_vga(coord.0, coord.1, 0x01);
                } else {
                    snake_to_vga(coord.0, coord.1, 0x00);
                }
            }

            erase(ERASE.0, ERASE.1);
        }
    }

    pub fn check_collision(&mut self, head: (u32, u32)) -> bool {
        if !(0 <= head.0 || head.0 < HEIGHT || 0 <= head.1 || head.1 < WIDTH) {
            return true;
        }

        return false;
    }
}
