# Implementing a basic STM

[Source Code](https://github.com/ceppelli/rust-by-ceppelli/blob/main/code/001-stm/src/basic_stm.rs)

This first rough attempt is a simple starting point just to have something to resoning about.

Let's define 2 events:

```rust,noplayground
pub enum Event {
  ResetEvent,
  KeyEvent{key_code:char}
}
```

The **ResetEvent** is for resetting the STM and the **KeyEvent** represents a generic key stroke.

The STM consists of 5 states:

```rust,noplayground
pub struct UnknownState;
pub struct BeginState;
pub struct HomeState {
  counter: u32,
}
pub struct ProcessingState {
  counter: u32,
}
pub struct EndState;

```

The behaviors and rules that will be implemented:
- the **UnknownState** represents an indeterminate condition
- the **ResetEvent** will force the STM state to **BeginState**
- the KeyCode 'h' will transition to the **HomeState**
- the KeyCode 'p' will transition to the **ProcessingState**
- the KeyCode 'i' will increment the counter in both **HomeState** and **ProcessingState** states
- the KeyCode 'e' will transition to the **EndState**


Now let's define a trait for consuming the events

```rust,noplayground
pub trait ConsumeEvent {
  fn on_event(&mut self, event:Event) {
  }
}

```
and write the states implementation

```rust,noplayground
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

    counter);
  }
}

impl ConsumeEvent for EndState {}
```

Let's define the STM data structure. The current state is allocated on the heap and uniquely owned by the current_state pointer.

```rust,noplayground
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

```

The implementation of the **ConsumeEvent** trait for the STM contains a simple logic for transitioning between states. Also is responsible for fowwargind the event the the current state.


```rust,noplayground
impl ConsumeEvent for STM<'_> {
  fn on_event(&mut self, event:Event) {

    // forward the event
    self.current_state.on_event(event.clone());

    match event {
      Event::ResetEvent => {
        self.current_state = Box::new(BeginState);
      },
      Event::KeyEvent {key_code: 'h' } => {
        self.current_state = Box::new(HomeState::default());
      },
      Event::KeyEvent {key_code: 'p' } => {
        self.current_state = Box::new(ProcessingState::default());
      },
      Event::KeyEvent {key_code: 'e' } => {
        self.current_state = Box::new(EndState);
      },
      Event::KeyEvent { key_code: 'i' } => {},
      _ => {
        self.current_state = Box::new(UnknownState);
      }
    }
  }
}

```

Below is an example of how to interact with the STM.

```rust,noplayground
let mut stm = STM::new("my_stm");

// to Begin State
stm.on_event(ResetEvent);

// to Home State
stm.on_event(KeyEvent{key_code:'h'});
stm.on_event(KeyEvent{key_code:'i'});
stm.on_event(KeyEvent{key_code:'i'});

// to Processing State
stm.on_event(KeyEvent{key_code:'p'});
stm.on_event(KeyEvent{key_code:'i'});
stm.on_event(KeyEvent{key_code:'i'});

// to Home State
stm.on_event(KeyEvent{key_code:'h'});
stm.on_event(KeyEvent{key_code:'i'});
stm.on_event(KeyEvent{key_code:'i'});

// to End State
stm.on_event(KeyEvent{key_code:'e'});

```

As first attempt this code carries different problems which will discuss and workaround in the next sections.


[Source Code](https://github.com/ceppelli/rust-by-ceppelli/blob/main/code/001-stm/src/basic_stm.rs)