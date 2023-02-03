# Improving the basic STM

[Source Code](https://github.com/ceppelli/rust-by-ceppelli/blob/main/code/001-stm/src/improved_basic_stm.rs)

In this section we discuss some problems with the previous implementation and try to fix some of them.

In the basic implementation the method **on_event** in the STM implementation is incapable to discriminate which events to propagate based on the current state. As workaround let's introduce the **States** enumation and use it to track the current state.

```rust,noplayground
pub enum States {
  Unknown,
  Begin,
  Home,
  Processing,
  End
}
```

Now the method **on_event** in the STM implementation could apply some logic based on the pair **(current_state, event)**.

```rust,noplayground

 fn on_event(&mut self, event:Event) -> Option<States>{

    match (self.current_state, event.clone()) {
      (_, Event::ResetEvent) => {

        self.switch_state(States::Begin);
      },
      (States::Begin, Event::KeyEvent { key_code:'h' }) => {

        if let Some(to_state) = self.state.on_event(event) {
          self.switch_state(to_state);
        }
      },
      (States::Home, Event::KeyEvent { key_code: 'p' }) |
      (States::Home, Event::KeyEvent { key_code: 'i' }) |
      (States::Home, Event::KeyEvent { key_code: 'e' })  => {

        if let Some(to_state) = self.state.on_event(event) {
          self.switch_state(to_state);
        }
      },
      (States::Processing, Event::KeyEvent { key_code: 'h' }) |
      (States::Processing, Event::KeyEvent { key_code: 'i' }) => {

        if let Some(to_state) = self.state.on_event(event) {
          self.switch_state(to_state);
        }
      },
      (States::End, Event::KeyEvent { key_code: _ }) => {},
      (_, _) => {
        self.switch_state(States::Unknown);
      }
    }

    None
  }

```

Some of the matching conditions are:
- the **ResetEvent** will always change the state to Begin.
- the **HomeState** accepts only **KeyEvents** with one of those **key_code**:'p', 'i', 'e'.
the **ProcessingState** accepts only **KeyEvents** with one of those **key_code**:'h', 'i'.


As second improvement the **ConsumeEvent** trait has been updated by adding to the **on_event** method the return value **Option<States>**. The return value if not **None** rappresents the state in which the STM will transition to.

```rust,noplayground
pub trait ConsumeEvent {
  fn on_event(&mut self, event:Event) -> Option<States> {
    None
  }
}

```
The implementation of the trait for the state as been updated too.

```rust,noplayground
impl ConsumeEvent for HomeState {
  fn on_event(&mut self, event:Event) -> Option<States> {
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

    to_state
  }
}

```
Here if the incoming **KeyEvent** has 'p' key_code the **to_state** will be Some(States::Processing). Otherwise if the key_code is 'e' the **to_state** will be Some(States::End).


To complete the update inside the **on_event** STM function the optional **to_state** hab been handled in the following way

```rust,noplayground
  fn on_event(&mut self, event:Event) -> Option<States>{

    match (self.current_state, event.clone()) {
      ...
      ...
      (States::Begin, Event::KeyEvent { key_code:'h' }) => {

        if let Some(to_state) = self.state.on_event(event) {
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

If the return value of **self.state.on_event(event)** is not **None** the local **switch_state(to_state)** function will be called.

Finally, in case the pair **(current_state, event)** is not matched the STM current state will be set to **Unknown**.


[Source Code](https://github.com/ceppelli/rust-by-ceppelli/blob/main/code/001-stm/src/improved_basic_stm.rs)
