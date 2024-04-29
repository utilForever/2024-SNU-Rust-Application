use std::collections::HashMap;
use std::hash::Hash;

// Counter counts the number of times each value of type T has been seen.
struct Counter<T: Eq + Hash> {
    values: HashMap<T, u64>,
}

impl<T: Eq + Hash> Counter<T> {
    // Create a new Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    // Count an occurrence of the given value.
    fn count(&mut self, value: T) {
        *self.values.entry(value).or_default() += 1;
    }

    // Return the number of times the given value has been seen.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_integer() {
        let mut counter = Counter::new();
        counter.count(13);
        counter.count(14);
        counter.count(16);
        counter.count(14);
        counter.count(14);
        counter.count(11);

        assert_eq!(counter.times_seen(10), 0);
        assert_eq!(counter.times_seen(11), 1);
        assert_eq!(counter.times_seen(12), 0);
        assert_eq!(counter.times_seen(13), 1);
        assert_eq!(counter.times_seen(14), 3);
        assert_eq!(counter.times_seen(15), 0);
        assert_eq!(counter.times_seen(16), 1);
        assert_eq!(counter.times_seen(17), 0);
    }

    #[test]
    fn counter_string() {
        let mut counter = Counter::new();
        counter.count("apple");
        counter.count("banana");
        counter.count("apple");
        counter.count("orange");

        assert_eq!(counter.times_seen("apple"), 2);
        assert_eq!(counter.times_seen("grape"), 0);
        assert_eq!(counter.times_seen("banana"), 1);
        assert_eq!(counter.times_seen("orange"), 1);
        assert_eq!(counter.times_seen("kiwi"), 0);
    }
}
