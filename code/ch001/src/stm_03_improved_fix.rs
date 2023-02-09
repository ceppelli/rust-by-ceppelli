use std::fmt::Debug;


#[derive(Debug, Clone)]
pub enum Event {
  ResetEvent,
  KeyEvent{key_code:char}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum States {
  Unknown,
  Begin,
  Home,
  Processing,
  End
}

trait ConsumeEvent : Debug {
  fn on_event(&mut self, event:Event, trace:bool) -> Option<States> {
    if trace {
      println!("[{:?}] on_event event:{:?}", self, event);
    }
    None
  }
}

#[derive(Debug)]
struct UnknownState;
#[derive(Debug)]
struct BeginState;
#[derive(Default, Debug)]
struct HomeState {
  counter: u32,
}
#[derive(Default, Debug)]
struct ProcessingState {
  counter: u32,
}
#[derive(Debug)]
struct EndState;

impl ConsumeEvent for UnknownState{}

impl ConsumeEvent for BeginState {
  fn on_event(&mut self, event:Event, trace:bool) -> Option<States> {
    let to_state = match event {
      Event::KeyEvent{key_code:'h'} => {
        Some(States::Home)
      },
      _ => {
        Some(States::Unknown)
      }
    };

    if trace {
      println!("[BeginState] on_event event:{:?} to_state:{:?}", event, to_state);
    }

    to_state
  }
}

impl ConsumeEvent for HomeState {
  fn on_event(&mut self, event:Event, trace:bool) -> Option<States> {
    let to_state = match event {
      Event::KeyEvent{key_code:'p'} => {
        Some(States::Processing)
      },
      Event::KeyEvent{key_code:'i'} => {
        self.counter += 1;
        None
      },
      Event::KeyEvent{key_code:'e'} => {
        Some(States::End)
      },
      _ => {
        Some(States::Unknown)
      }
    };

    if trace {
      println!("[HomeState] on_event event:{:?} counter:{}, to_state:{:?}", event, self.counter, to_state);
    }

    to_state
  }
}

impl ConsumeEvent for ProcessingState {
  fn on_event(&mut self, event:Event, trace:bool) -> Option<States> {
    let to_state = match event {
      Event::KeyEvent{key_code:'h'} => {
        Some(States::Home)
      },
      Event::KeyEvent{key_code:'i'} => {
        self.counter += 1;
        None
      },
      _ => {
        Some(States::Unknown)
      }
    };

    if trace {
      println!("[ProcessingState] on_event event:{:?} counter:{}, to_state:{:?}", event, self.counter, to_state);
    }

    to_state
  }
}

impl ConsumeEvent for EndState {}

pub trait STMConsumeEvent {
  fn on_event(&mut self, event:Event);
}

#[derive(Debug)]
#[allow(unused)]
pub struct STM<'a> {
  pub name:&'a str,
  pub current_state:States,
  unknown_state: UnknownState,
  begin_state: BeginState,
  home_state: HomeState,
  processing_state: ProcessingState,
  end_state: EndState,
  trace: bool
}

impl STM<'_> {
  pub fn new<'a>(name:&'a str, trace:bool) -> STM<'a> {
    STM {
      name:name,
      current_state:States::Unknown,
      unknown_state:UnknownState,
      begin_state:BeginState,
      home_state:HomeState{counter:0},
      processing_state:ProcessingState{counter:0},
      end_state:EndState,
      trace
    }
  }

  fn switch_state(&mut self, to_state:States) {
    self.current_state = to_state;
  }
}

impl STMConsumeEvent for STM<'_> {
  fn on_event(&mut self, event:Event) {
    if self.trace {
      println!("-----------------------------------");
      println!("[STM] on_event event:{:?} current state:{:?}", event, self.current_state);
    }

    match (self.current_state, event.clone()) {
      (_, Event::ResetEvent) => {

        self.switch_state(States::Begin);
      },
      (States::Begin, Event::KeyEvent { key_code:'h' }) => {

        if let Some(to_state) = self.begin_state.on_event(event, self.trace) {
          self.switch_state(to_state);
        }
      },
      (States::Home, Event::KeyEvent { key_code: 'p' }) |
      (States::Home, Event::KeyEvent { key_code: 'i' }) |
      (States::Home, Event::KeyEvent { key_code: 'e' })  => {

        if let Some(to_state) = self.home_state.on_event(event, self.trace) {
          self.switch_state(to_state);
        }
      },
      (States::Processing, Event::KeyEvent { key_code: 'h' }) |
      (States::Processing, Event::KeyEvent { key_code: 'i' }) => {

        if let Some(to_state) = self.processing_state.on_event(event, self.trace) {
          self.switch_state(to_state);
        }
      },
      (States::End, Event::KeyEvent { key_code: _ }) => {},
      (_, _) => {
        self.switch_state(States::Unknown);
      }
    }
  }
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn test_begin_state() {
    let mut state = BeginState;

    let to_state = state.on_event(Event::KeyEvent { key_code: '*' }, false);
    assert_eq!(to_state, Some(States::Unknown));
    let to_state = state.on_event(Event::KeyEvent { key_code: 'h' }, false);
    assert_eq!(to_state, Some(States::Home));
  }

  #[test]
  fn test_home_state() {
    let mut state = HomeState::default();

    let to_state = state.on_event(Event::KeyEvent{key_code:'*'}, false);
    assert_eq!(to_state, Some(States::Unknown));
    assert_eq!(state.counter, 0);
    let to_state = state.on_event(Event::KeyEvent{key_code:'p'}, false);
    assert_eq!(to_state, Some(States::Processing));
    assert_eq!(state.counter, 0);
    let to_state = state.on_event(Event::KeyEvent{key_code:'e'}, false);
    assert_eq!(to_state, Some(States::End));
    assert_eq!(state.counter, 0);
    let to_state = state.on_event(Event::KeyEvent{key_code:'i'}, false);
    assert_eq!(to_state, None);
    assert_eq!(state.counter, 1);
    let to_state = state.on_event(Event::KeyEvent{key_code:'i'}, false);
    assert_eq!(to_state, None);
    assert_eq!(state.counter, 2);
  }

  #[test]
  fn test_process_state() {
    let mut state = ProcessingState::default();

    let to_state = state.on_event(Event::KeyEvent{key_code:'*'}, false);
    assert_eq!(to_state, Some(States::Unknown));
    assert_eq!(state.counter, 0);
    let to_state = state.on_event(Event::KeyEvent{key_code:'h'}, false);
    assert_eq!(to_state, Some(States::Home));
    assert_eq!(state.counter, 0);
    let to_state = state.on_event(Event::KeyEvent{key_code:'i'}, false);
    assert_eq!(to_state, None);
    assert_eq!(state.counter, 1);
    let to_state = state.on_event(Event::KeyEvent{key_code:'i'}, false);
    assert_eq!(to_state, None);
    assert_eq!(state.counter, 2);
  }

  #[test]
  fn test_stm() {
    let mut stm = STM::new("test_stm", true);

    assert_eq!(stm.current_state, States::Unknown);

    stm.on_event(Event::ResetEvent);
    assert_eq!(stm.current_state, States::Begin);

    stm.on_event(Event::KeyEvent { key_code: 'h' });
    assert_eq!(stm.current_state, States::Home);

    stm.on_event(Event::KeyEvent { key_code: 'p' });
    assert_eq!(stm.current_state, States::Processing);

    stm.on_event(Event::KeyEvent { key_code: 'h' });
    assert_eq!(stm.current_state, States::Home);

    stm.on_event(Event::KeyEvent { key_code: 'e' });
    assert_eq!(stm.current_state, States::End);

    // The EndState is considered as final. In this state the STM is receptive
    // only to the ResetEvent, which restart the STM
    stm.on_event(Event::KeyEvent { key_code: '*' });
    assert_eq!(stm.current_state, States::End);
  }

}