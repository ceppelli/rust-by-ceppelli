# Fixing the Improved STM

[Source Code](https://github.com/ceppelli/rust-by-ceppelli/blob/main/code/ch001/src/stm_03_improved_fix.rs)

The problem of the previous implementation is that, whenever the STM current state changes, the information bound with the previous state is lost for good.

For example, both the **HomeState** and tbe **ProcessState** have **counter** as local internalt varible. Let's assume the current state is **ProcessState**, in this case the **KeyEvents** with **key_code** 'i' trigggers the increment of the counter. If anther **KeyEvents** with **key_code** 'i' is received the count will increase again and so on. Now if a **KeyEvents** with **key_code** 'h' is received, the STM will swith to **HomeState** and the counter information bound with the previous **ProcessState** will be lost.


This unwanted behavior is caused by the **state** field inside the STM struct which points to the current **state implemetation**.

```rust,noplayground
pub struct STM<'a> {
  ...
  state:Box<dyn ConsumeEvent>,
  ...
}
```

This **pointer** is updated every time the STM changes state inseide the **switch_state** STM method with a new state.

```rust,noplayground
fn switch_state(&mut self, to_state:States) {
    match to_state {
      States::Unknown => {
        self.state = Box::new(UnknownState);
      },
      States::Begin => {
        self.state = Box::new(BeginState);
      },
      States::Home => {
        self.state = Box::new(HomeState::default());
      },
      States::Processing => {
        self.state = Box::new(ProcessingState::default());
      },
      States::End => {
        self.state = Box::new(EndState);
      }
    }
    self.current_state = to_state;
  }
```

To fix this problem, let's modify the STM struct in the following way

```rust,noplayground
trait ConsumeEvent {
  fn on_event(&mut self, event:Event) -> Option<States> {
    None
  }
}

```
The implementation of the trait for the state as been updated too.

```rust,noplayground
pub struct STM<'a> {
  pub name:&'a str,
  pub current_state:States,
  unknown_state: UnknownState,
  begin_state: BeginState,
  home_state: HomeState,
  processing_state: ProcessingState,
  end_state: EndState
}

```

Just replacing the **state:Box<dyn ConsumeEvent>** with all the states and using the **current_state:States** enum as discriminator fow knowing which states to forward the incoming event.


```rust,noplayground
  fn on_event(&mut self, event:Event) -> Option<States>{

    match (self.current_state, event.clone()) {
      ...
      ...
      (States::Begin, Event::KeyEvent { key_code:'h' }) => {

        if let Some(to_state) = self.state.begin_state(event) {
          self.switch_state(to_state);
        }
      },
      (States::Home, Event::KeyEvent { key_code: 'p' }) |
      (States::Home, Event::KeyEvent { key_code: 'i' }) |
      (States::Home, Event::KeyEvent { key_code: 'e' })  => {

        if let Some(to_state) = self.home_state.on_event(event) {
          self.switch_state(to_state);
        }
      },
      ...
      ...
      (_, _) => {
        self.switch_state(States::Unknown);
      }
    }

    None
  }

```

and the **switch_state** method has simplified to

```rust,noplayground
  fn switch_state(&mut self, to_state:States) {
    self.current_state = to_state;
  }
```

With those simple modifications the local state is preserved across state transitions.

At first glance this solution should lead to an improvement in performance, hypotheses that we will test in the next section.


[Source Code](https://github.com/ceppelli/rust-by-ceppelli/blob/main/code/ch001/src/stm_03_improved_fix.rs)
