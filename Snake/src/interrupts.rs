#![allow(static_mut_refs)]

use crate::{gdt, hlt_loop, print, println, serial_print};
use core::fmt;
use pc_keyboard::{DecodedKey, HandleControl, KeyState, Keyboard, ScancodeSet1, layouts};
use pic8259::ChainedPics;
use spin::lazy::Lazy;
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::snake::{C_COUNT, FOOD, INDEX, SEED, SNAKE_ON, get_snake, init_snake, random_pos};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static mut PICS: pic8259::ChainedPics = unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) };

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX as u16);
    }
    idt[InterruptIndex::Timer as usize].set_handler_fn(timer_interrupt_handler);
    idt[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_interrupt_handler);

    idt
});

pub fn init_idt() {
    IDT.load();
    unsafe {
        PICS.initialize();
        interrupts::enable();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

static mut DIRECTION: char = 'w';

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {

    if unsafe { INDEX } == 6 {
        unsafe {
            if SNAKE_ON == false {
                FOOD = random_pos(C_COUNT);
                C_COUNT += 1;
                let snake = get_snake();
                init_snake();
                SNAKE_ON = true;
            }
        }
        let snake = get_snake();
        unsafe { snake.update_position(DIRECTION) };
    }
    unsafe {
        PICS.notify_end_of_interrupt(InterruptIndex::Timer as u8);
    };
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use lazy_static::lazy_static;
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    let scancode: u8 = unsafe { Port::new(0x60).read() };

    unsafe {
        if INDEX < 6 {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                let state = key_event.state;
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    match key {
                        DecodedKey::Unicode(character) => match character {
                            '0' => unsafe { SEED = SEED * 10 + 0 },
                            '1' => unsafe { SEED = SEED * 10 + 1 },
                            '2' => unsafe { SEED = SEED * 10 + 2 },
                            '3' => unsafe { SEED = SEED * 10 + 3 },
                            '4' => unsafe { SEED = SEED * 10 + 4 },
                            '5' => unsafe { SEED = SEED * 10 + 5 },
                            '6' => unsafe { SEED = SEED * 10 + 6 },
                            '7' => unsafe { SEED = SEED * 10 + 7 },
                            '8' => unsafe { SEED = SEED * 10 + 8 },
                            '9' => unsafe { SEED = SEED * 10 + 9 },
                            _ => (),
                        },
                        DecodedKey::RawKey(key) => (),
                    }
                }
                if state == KeyState::Down {
                    //println!("INDEX BEFORE {:?}", INDEX);
                    INDEX += 1;
                    //println!("INDEX AFTER {:?}", INDEX);
                }
            }
        } else {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                let state = key_event.state;
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    match key {
                        DecodedKey::Unicode(character) => match character {
                            'w' => unsafe { DIRECTION = 'w' },
                            'a' => unsafe { DIRECTION = 'a' },
                            's' => unsafe { DIRECTION = 's' },
                            'd' => unsafe { DIRECTION = 'd' },
                            _ => (),
                        },
                        DecodedKey::RawKey(key) => (),
                    }
                }
            }
        }
    }
    unsafe { PICS.notify_end_of_interrupt(InterruptIndex::Keyboard as u8) };
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    //Error code provides info about the type of memory access that caused page fault
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    //Cr2 contains the accessed virtual address that
    //caused the page fault, read() reads and prints it
    println!("Error Code: {:?}", stack_frame);
    println!("{:#?}", error_code);
    hlt_loop();
}
