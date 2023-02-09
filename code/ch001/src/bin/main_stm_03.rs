use ch001::stm_03_improved_fix::{STM, STMConsumeEvent, Event};


fn main() {
    s01();
}

fn s01() {
    let mut stm = STM::new("improved_fix_stm", true);

    println!("[main] ====================================");
    println!("[main] start STM name:{} status:{:?}", stm.name, stm.current_state);

    // to Begin State
    stm.on_event(Event::ResetEvent);

    // to Home State
    stm.on_event(Event::KeyEvent{key_code:'h'});
    stm.on_event(Event::KeyEvent{key_code:'i'});
    stm.on_event(Event::KeyEvent{key_code:'i'});

    // to Processing State
    stm.on_event(Event::KeyEvent{key_code:'p'});
    stm.on_event(Event::KeyEvent{key_code:'i'});
    stm.on_event(Event::KeyEvent{key_code:'i'});

    // to Home State
    stm.on_event(Event::KeyEvent{key_code:'h'});
    stm.on_event(Event::KeyEvent{key_code:'i'});
    stm.on_event(Event::KeyEvent{key_code:'i'});

    // to End State
    stm.on_event(Event::KeyEvent{key_code:'e'});

    println!("[main] end STM name:{} status:{:?}\n", stm.name, stm.current_state);
}


