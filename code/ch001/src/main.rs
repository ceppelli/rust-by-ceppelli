mod basic_stm;
mod improved_basic_stm;

use crate::basic_stm::{self as s01, ConsumeEvent as s01ConsumeEvent};
use crate::improved_basic_stm::{self as s02, ConsumeEvent as s02ConsumeEvent};

fn main() {
    let mut stm = s01::STM::new("basic_stm");

    println!("[main] ====================================");
    println!("[main] start STM name:{} status:{:?}", stm.name, stm.current_state);

    // to Begin State
    stm.on_event(s01::Event::ResetEvent);

    // to Home State
    stm.on_event(s01::Event::KeyEvent{key_code:'h'});
    stm.on_event(s01::Event::KeyEvent{key_code:'i'});
    stm.on_event(s01::Event::KeyEvent{key_code:'i'});

    // to Processing State
    stm.on_event(s01::Event::KeyEvent{key_code:'p'});
    stm.on_event(s01::Event::KeyEvent{key_code:'i'});
    stm.on_event(s01::Event::KeyEvent{key_code:'i'});

    // to Home State
    stm.on_event(s01::Event::KeyEvent{key_code:'h'});
    stm.on_event(s01::Event::KeyEvent{key_code:'i'});
    stm.on_event(s01::Event::KeyEvent{key_code:'i'});

    // to End State
    stm.on_event(s01::Event::KeyEvent{key_code:'e'});

    println!("[main] end STM name:{} status:{:?}\n", stm.name, stm.current_state);

    let mut stm = s02::STM::new("improved_stm");

    println!("[main] ====================================");
    println!("[main] start STM name:{} status:{:?}", stm.name, stm.current_state);

    // to Begin State
    stm.on_event(s02::Event::ResetEvent);

    stm.on_event(s02::Event::KeyEvent{key_code:'h'});
    stm.on_event(s02::Event::KeyEvent{key_code:'i'});
    stm.on_event(s02::Event::KeyEvent{key_code:'i'});

    // to Processing State
    stm.on_event(s02::Event::KeyEvent{key_code:'p'});
    stm.on_event(s02::Event::KeyEvent{key_code:'i'});
    stm.on_event(s02::Event::KeyEvent{key_code:'i'});

    // to Home State
    stm.on_event(s02::Event::KeyEvent{key_code:'h'});
    stm.on_event(s02::Event::KeyEvent{key_code:'i'});
    stm.on_event(s02::Event::KeyEvent{key_code:'i'});

    // to End State
    stm.on_event(s02::Event::KeyEvent{key_code:'e'});
    stm.on_event(s02::Event::KeyEvent{key_code:'i'});

    println!("[main] end STM name:{} status:{:?}\n", stm.name, stm.current_state);
}
