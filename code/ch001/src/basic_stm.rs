use std::fmt::Debug;


#[derive(Debug, Clone)]
pub enum Event {
  ResetEvent,
  KeyEvent{key_code:char}
}

pub trait ConsumeEvent : Debug {
  fn on_event(&mut self, event:Event) {
    println!("[{:?}] on_event event:{:?}", self, event);
  }
}

#[derive(Debug)]
pub struct UnknownState;
#[derive(Debug)]
pub struct BeginState;
#[derive(Default, Debug)]
pub struct HomeState {
  counter: u32,
}
#[derive(Default, Debug)]
pub struct ProcessingState {
  counter: u32,
}
#[derive(Debug)]
pub struct EndState;

impl ConsumeEvent for UnknownState{}

impl ConsumeEvent for BeginState {}

impl ConsumeEvent for HomeState {
  fn on_event(&mut self, event:Event) {
    match event {
      Event::KeyEvent{key_code:'i'} => {
        self.counter += 1;
      },
      _ => {}
    }

    println!("[HomeState] on_event event:{:?} counter:{}", event, self.counter);
  }
}

impl ConsumeEvent for ProcessingState {
  fn on_event(&mut self, event:Event) {
    match event {
      Event::KeyEvent{key_code:'i'} => {
        self.counter += 1;
      },
      _ => {}
    }

    println!("[ProcessingState] on_event event:{:?} counter:{}", event, self.counter);
  }
}

impl ConsumeEvent for EndState {}

#[derive(Debug)]
pub struct STM<'a> {
  pub name:&'a str,
  pub current_state:Box<dyn ConsumeEvent>,
}

impl STM<'_> {
  pub fn new<'a>(name:&'a str) -> STM<'a> {
    STM {
      name:name,
      current_state:Box::new(UnknownState),
    }
  }
}

impl ConsumeEvent for STM<'_> {
  fn on_event(&mut self, event:Event) {
    println!("-----------------------------------");
    println!("[STM] on_event event:{:?}", event);

    // forward the event
    self.current_state.on_event(event.clone());

    match event {
      Event::ResetEvent => {
        println!("[STM]     cuttent_state set to BeginState");
        self.current_state = Box::new(BeginState);
      },
      Event::KeyEvent {key_code: 'h' } => {
        println!("[STM]     cuttent_state set to HomeState");
        self.current_state = Box::new(HomeState::default());
      },
      Event::KeyEvent {key_code: 'p' } => {
        println!("[STM]     cuttent_state set to ProcessingState");
        self.current_state = Box::new(ProcessingState::default());
      },
      Event::KeyEvent {key_code: 'e' } => {
        println!("[STM]     cuttent_state set to EndState");
        self.current_state = Box::new(EndState);
      },
      Event::KeyEvent { key_code: 'i' } => {},
      _ => {
        println!("[STM]     cuttent_state set to UnknownState");
        self.current_state = Box::new(UnknownState);
      }
    }
  }
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_home_state() {
    let mut state = HomeState::default();

    state.on_event(Event::KeyEvent{key_code:'*'});
    assert_eq!(state.counter, 0);
    state.on_event(Event::KeyEvent{key_code:'i'});
    assert_eq!(state.counter, 1);
    state.on_event(Event::KeyEvent{key_code:'i'});
    assert_eq!(state.counter, 2);
  }

  #[test]
  fn test_process_state() {
    let mut state = ProcessingState::default();

    state.on_event(Event::KeyEvent{key_code:'*'});
    assert_eq!(state.counter, 0);
    state.on_event(Event::KeyEvent{key_code:'i'});
    assert_eq!(state.counter, 1);
    state.on_event(Event::KeyEvent{key_code:'i'});
    assert_eq!(state.counter, 2);
  }

}