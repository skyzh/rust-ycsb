mod acknowledged_counter_generator;
mod constant_generator;
mod counter_generator;
mod discrete_generator;
mod uniform_long_generator;
mod zipfian_generator;

pub use acknowledged_counter_generator::AcknowledgedCounterGenerator;
pub use constant_generator::ConstantGenerator;
pub use counter_generator::CounterGenerator;
pub use discrete_generator::DiscreteGenerator;
pub use uniform_long_generator::UniformLongGenerator;
pub use zipfian_generator::ZipfianGenerator;

use enum_dispatch::enum_dispatch;
use std::string::ToString;

#[enum_dispatch(Generator)]
pub enum Generators {
    UniformLong(UniformLongGenerator),
    Zipfian(ZipfianGenerator),
    Constant(ConstantGenerator<u64>),
}

#[enum_dispatch]
pub trait Generator<T: ToString + Clone> {
    fn next_value(&self) -> T;
}

pub trait NumberGenerator<T: ToString + Clone>: Generator<T> {
    fn mean(&self) -> T;
}

pub struct GeneratorImpl<T: ToString + Clone, G: Generator<T>> {
    last_value: Option<T>,
    generator: G,
}

impl<T, G> GeneratorImpl<T, G>
where
    G: Generator<T>,
    T: ToString + Clone,
{
    pub fn new(generator: G) -> Self {
        Self {
            generator,
            last_value: None,
        }
    }

    pub fn next_value(&mut self) -> T {
        let v = self.generator.next_value();
        self.last_value = Some(v.clone());
        v
    }

    pub fn last_value(&self) -> T {
        self.last_value.clone().unwrap()
    }

    pub fn next_string(&mut self) -> String {
        self.next_value().to_string()
    }

    pub fn last_string(&self) -> String {
        self.last_value().to_string()
    }
}
