# Benchmarking

[Source Code](https://github.com/ceppelli/rust-by-ceppelli/blob/main/code/ch001/src/bin/main_stm_perf.rs)

Let's implement a really simple benchmark just for testing if performance improvements have introduced during the refactoring of the STM code.


Each STM instance will receive the same sequesnce of events and the sequence will be repeted  1M time.

```rust,noplayground
  ...
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
  ...
}
```

Here the results I collected on my machine.

```bach
cargo run --bin stm_perf

[main] ====================================
[main] start STM name:basic_stm status:UnknownState
[main] end STM name:basic_stm status:EndState

[main] ====================================
[main] start STM name:improved_stm status:Unknown
[main] end STM name:improved_stm status:End

[main] ====================================
[main] start STM name:improved_fix_stm status:Unknown
[main] end STM name:improved_fix_stm status:End

S01 Elapsed: 389.74ms
S02 Elapsed: 413.35ms
S03 Elapsed: 110.42ms

```

The test shows that the latest implementation is 4 times quicker than the others. This shouldn't be a surprise as using the heap is an expensive process.

[Source Code](https://github.com/ceppelli/rust-by-ceppelli/blob/main/code/ch001/src/bin/main_stm_perf.rs)
