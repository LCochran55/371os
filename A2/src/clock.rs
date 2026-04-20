use core::fmt::{self,Display,Formatter};

static mut _CLOCK: Clock = Clock { hrs:0,min:0,sec:0 };

pub fn get_clock() -> &'static mut Clock {
    unsafe{
        &mut _CLOCK
    }
}

pub struct Clock {
    hrs: u8,
    min: u8,
    sec: u8,
}

pub static mut CHARS: [u8; 6] = [0;6];
pub static mut INDEX: usize = 0;

pub fn init_clock() {

}

impl Clock {

    pub unsafe fn new_unchecked(hrs:u8,min:u8,sec:u8) -> Clock {
        Clock {hrs,min,sec}
    }
    pub fn new(hrs:u8,min:u8,sec:u8) -> Option<Clock> {
        // TODO: check hr min seconds
        Some(Clock {hrs,min,sec})
    }
    //Modifies an existing time
    //Static mut resource initialized and ticked w/ every timer
    pub fn tick(&mut self) {
        //Basically an inreffutable pattern match
        let Clock {hrs,min,sec} = *self;
        //Seconds plus one, wraps around when 60
        let inc_sec = sec + 1;
        let new_sec = inc_sec % 60;


        //If seconds wraps around, then sec / 60
        //will be one or it will be zero
        let inc_min = min + (inc_sec / 60);
        let new_min = inc_min % 60;
        
        //Hours is never going to go over 24
        let new_hrs = hrs + (inc_min / 60); //% 24

        //Acts like a second is passing
        *self = Clock {hrs: new_hrs, min: new_min, sec: new_sec};

    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Clock {hrs, min, sec} = *self;
        write!(f,"{hrs:02}:{min:02}:{sec:02}")
    }
}
