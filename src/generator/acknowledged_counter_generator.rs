use super::{CounterGenerator, Generator, NumberGenerator};
use rand::prelude::*;
use std::sync::Mutex;

const WINDOW_SIZE: u64 = 1 << 20;
const WINDOW_MASK: u64 = WINDOW_SIZE - 1;

struct Core {
    window: Vec<bool>,
    limit: u64,
}
pub struct AcknowledgedCounterGenerator {
    counter: CounterGenerator,
    core: Mutex<Core>,
}

impl AcknowledgedCounterGenerator {
    pub fn new(count_start: u64) -> Self {
        let counter = CounterGenerator::new(count_start);
        let mut window = Vec::with_capacity(WINDOW_SIZE as usize);
        window.resize(WINDOW_SIZE as usize, false);
        let core = Core {
            window,
            limit: count_start - 1,
        };
        Self {
            counter,
            core: Mutex::new(core),
        }
    }

    pub fn acknowledge(&self, value: u64) {
        let current_slot = value & WINDOW_MASK;
        if let Ok(mut core) = self.core.try_lock() {
            let before_first_slot = core.limit & WINDOW_MASK;
            let mut index = core.limit + 1;
            let new_index = loop {
                if index != before_first_slot {
                    let slot = (index & WINDOW_MASK) as usize;
                    if !core.window[slot] {
                        break index;
                    }
                    core.window[slot] = false;
                } else {
                    break index;
                }
                index += 1;
            };
            core.limit = new_index - 1;
        }
    }

    pub fn last_value(&self) -> u64 {
        let core = self.core.lock().unwrap();
        core.limit
    }
}

impl Generator<u64> for AcknowledgedCounterGenerator {
    fn next_value(&self) -> u64 {
        self.counter.next_value()
    }
}
