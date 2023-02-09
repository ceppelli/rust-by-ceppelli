use ch001::stm_01_basic::{self as s01, STMConsumeEvent as s01STMConsumeEvent};
use ch001::stm_02_improved::{self as s02, STMConsumeEvent as s02STMConsumeEvent};
use ch001::stm_03_improved_fix::{self as s03, STMConsumeEvent as s03STMConsumeEvent};

use std::time::{Instant, Duration};

fn main() {

    const LOOPS: u32 = 1000*1000;

    let s01_elapsed = s01(LOOPS);
    let s02_elapsed = s02(LOOPS);
    let s03_elapsed = s03(LOOPS);

    println!("S01 Elapsed: {:.2?}", s01_elapsed);
    println!("S02 Elapsed: {:.2?}", s02_elapsed);
    println!("S03 Elapsed: {:.2?}", s03_elapsed);
}


fn s01(loops:u32) -> Duration {
    let mut stm = s01::STM::new("basic_stm", false);

    println!("[main] ====================================");
    println!("[main] start STM name:{} status:{:?}", stm.name, stm.current_state);

    let now = Instant::now();

    for _i in 0..loops {

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
    }

    let elapsed = now.elapsed();

    println!("[main] end STM name:{} status:{:?}\n", stm.name, stm.current_state);

    elapsed
}

fn s02(loops:u32) -> Duration {
    let mut stm = s02::STM::new("improved_stm", false);

    println!("[main] ====================================");
    println!("[main] start STM name:{} status:{:?}", stm.name, stm.current_state);

    let now = Instant::now();

    for _i in 0..loops {
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
    }

    let elapsed = now.elapsed();

    println!("[main] end STM name:{} status:{:?}\n", stm.name, stm.current_state);

    elapsed
}

fn s03(loops:u32) -> Duration {
    let mut stm = s03::STM::new("improved_fix_stm", false);

    println!("[main] ====================================");
    println!("[main] start STM name:{} status:{:?}", stm.name, stm.current_state);

    let now = Instant::now();

    for _i in 0..loops {

        // to Begin State
        stm.on_event(s03::Event::ResetEvent);

        stm.on_event(s03::Event::KeyEvent{key_code:'h'});
        stm.on_event(s03::Event::KeyEvent{key_code:'i'});
        stm.on_event(s03::Event::KeyEvent{key_code:'i'});

        // to Processing State
        stm.on_event(s03::Event::KeyEvent{key_code:'p'});
        stm.on_event(s03::Event::KeyEvent{key_code:'i'});
        stm.on_event(s03::Event::KeyEvent{key_code:'i'});

        // to Home State
        stm.on_event(s03::Event::KeyEvent{key_code:'h'});
        stm.on_event(s03::Event::KeyEvent{key_code:'i'});
        stm.on_event(s03::Event::KeyEvent{key_code:'i'});

        // to End State
        stm.on_event(s03::Event::KeyEvent{key_code:'e'});
        stm.on_event(s03::Event::KeyEvent{key_code:'i'});
    }

    let elapsed = now.elapsed();

    println!("[main] end STM name:{} status:{:?}\n", stm.name, stm.current_state);

    elapsed
}


