use crate::print_snake;
use crate::vga::snake_to_vga;
use alloc::vec;
use alloc::vec::Vec;

use crate::{exit_qemu, QEMU_FAIL};

const HEIGHT: u32 = 25;
const WIDTH: u32 = 80;

pub static mut SEED: u32 = 0;
pub static mut INDEX: usize = 0;
pub static mut SNAKE_ON: bool = false;

pub static mut FOOD: (u32, u32) = (0, 0);

pub static mut C_COUNT: u32 = 0;

static mut SNAKE: Snake = Snake {
    body: Vec::new(),
    head: (0, 0),
};


pub fn init_snake() {
    unsafe {
        SNAKE.body.push((0,0));
    }
}


pub fn get_snake() -> &'static mut Snake {
    unsafe { &mut SNAKE }
}

pub fn random_pos(c: u32) -> (u32, u32) {
    //VGA = 720x40 0r 80x25
    let a1: u32 = 22695477;
    let m1: u32 = 2u32.pow(31);

    let a2: u32 = 1103515245;
    let m2: u32 = 2u32.pow(31);

    unsafe {
        let seed1 = (a1 * SEED + c) % m1;
        let seed2 = (a2 * SEED + c) % m2;

        let random1 = ((seed1 / m1) * (2000 - 0 + 1) + 0) as u32;
        let random2 = ((seed2 / m2) * (2000 - 0 + 1) + 0) as u32;

        return (random1, random2);
    }
}

pub struct Snake {
    body: Vec<(u32, u32)>,
    head: (u32, u32),
}

impl Snake {
    pub fn new(initial_head: (u32, u32)) -> Self {
        let mut body = Vec::new();
        body.push(initial_head);
        Snake {
            body,
            head: initial_head,
        }
    }

    pub fn update_position(&mut self, direction: char) {
        let (mut head_r, mut head_c) = self.body[0];

        let mut new_head = (0, 0);

        //Gets the direction which the head of the snake should update in
        match direction {
            'w' => new_head = (head_r, head_c - 1),
            'a' => new_head = (head_r - 1, head_c),
            's' => new_head = (head_r, head_c + 1),
            'd' => new_head = (head_r + 1, head_c),
            _ => (),
        }

        //Check if the updated snake position will hit a wall or itself -> itself todo
        if self.check_collision(new_head) == true {
            exit_qemu(QEMU_FAIL);
        }

        self.head = new_head;

        if unsafe { FOOD == new_head } {
            self.get_longer();
            unsafe {
                FOOD = random_pos(C_COUNT);
                C_COUNT += 1;
            }
        } else {
            self.body.insert(0, new_head);
            self.body.pop();
        }

        unsafe {
            for coord in &self.body {
                print_snake!(coord);
            }
        }
    }

    pub fn check_collision(&mut self, head: (u32, u32)) -> bool {
        if !(0 <= head.0 && head.0 < HEIGHT && 0 <= head.1 && head.1 < WIDTH) {
            return true;
        }

        //for &item in self.body.iter() {
        //      if item == new_head {
        //    binkle_os::exit_qemu();
        //   }

        return false;
    }

    pub fn get_longer(&mut self) {
        unsafe { self.body.insert(0, FOOD) };
    }
}
