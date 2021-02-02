use super::Generator;
use rand::prelude::*;

pub struct Pair<T: Clone> {
    weight: f64,
    value: T,
}

impl<T: Clone> Pair<T> {
    pub fn new(weight: f64, value: impl Into<T>) -> Self {
        Self {
            weight,
            value: value.into(),
        }
    }
}

pub struct DiscreteGenerator<T: Clone> {
    values: Vec<Pair<T>>,
    sum: f64,
}

impl<T: ToString + Clone> DiscreteGenerator<T> {
    pub fn new(values: Vec<Pair<T>>) -> Self {
        let mut sum = 0.0;
        for Pair { weight, .. } in &values {
            sum += *weight;
        }
        Self { values, sum }
    }
}

impl<T: ToString + Clone> Generator<T> for DiscreteGenerator<T> {
    fn next_value(&self) -> T {
        let mut val = thread_rng().gen::<f64>();
        for Pair { weight, value } in &self.values {
            let pw = *weight / self.sum;
            if val < pw {
                return value.clone();
            }
            val -= pw;
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discrete_generator() {
        let pairs = vec![Pair::new(0.3, "test"), Pair::new(0.7, "b")];
        let generator = DiscreteGenerator::<String>::new(pairs);
        let mut result = std::collections::HashMap::new();
        for _i in 0..10000 {
            let val = generator.next_value();
            result.entry(val).and_modify(|x| *x += 1).or_insert(1);
        }
        println!("{:?}", result);
    }
}
