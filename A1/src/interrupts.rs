#![allow(static_mut_refs)]
use crate::{gdt, hlt_loop, print, println};
use core::fmt;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use pic8259::ChainedPics;
use spin::lazy::Lazy;
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

// PIC
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

extern "x86-interrupt" fn timer_interrupt_handler(stack_frame: InterruptStackFrame) {
    //print!(".");
    //println!("INTERRUPT: TIMER\n{:#?}", stack_frame);

    unsafe { PICS.notify_end_of_interrupt(InterruptIndex::Timer as u8) };
}

//static mut KEYBOARD: Keyboard<layouts::Us104Key, ScancodeSet1> = Keyboard::new(
//  ScancodeSet1::new(),
// layouts::Us104Key,
// HandleControl::Ignore);

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
    let scancode: u8 = unsafe { Port::new(0x60).read() }; // Figures out which key is pressed
    // Port read a byte from the keyboard’s data port.
    // Byte is called the scancode and it represents
    // the key press/release
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => (), //println!("{:?}", key),
            }
        }
    }

    unsafe { PICS.notify_end_of_interrupt(InterruptIndex::Keyboard as u8) };
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    //Error code provides
    //info about the type of
    //memory access that caused page fault
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read()); //Cr2 contains the accessed virtual address that 
    //caused the page fault, read() reads and
    //prints it
    println!("Error Code: {:?}", stack_frame);
    println!("{:#?}", error_code);
    hlt_loop();
}
