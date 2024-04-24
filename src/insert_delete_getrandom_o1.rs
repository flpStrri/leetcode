// # Insert Delete GetRandom O(1)
//
// Implement the RandomizedSet class:
// - RandomizedSet() Initializes the RandomizedSet object.
// - bool insert(int val) Inserts an item val into the set if not present. Returns true if the item was not present, false otherwise.
// - bool remove(int val) Removes an item val from the set if present. Returns true if the item was present, false otherwise.

// int getRandom() Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called).
// Each element must have the same probability of being returned.
// You must implement the functions of the class such that each function works in average O(1) time complexity.

use rand::rngs::ThreadRng;
use rand::Rng;

struct RandomizedSet {
    data_map: std::collections::HashMap<i32, usize>,
    data_vec: Vec<i32>,
    data_size: usize,
    rnd: ThreadRng,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            data_map: std::collections::HashMap::new(),
            data_vec: Vec::with_capacity(200_000),
            data_size: 0,
            rnd: rand::thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.data_map.contains_key(&val) {
            return false;
        }

        self.data_vec.push(val);
        self.data_map.insert(val, self.data_size);
        self.data_size += 1;
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        let possible_value_index = self.data_map.remove(&val);
        if possible_value_index.is_none() {
            return false;
        }

        let value_index = possible_value_index.unwrap();
        if value_index == self.data_size - 1 {
            self.data_vec.swap_remove(value_index);
        } else {
            let last_value = *self.data_vec.last().unwrap();
            self.data_vec.swap_remove(value_index);
            self.data_map.insert(last_value, value_index);
        }
        self.data_size -= 1;

        true
    }

    fn get_random(&mut self) -> i32 {
        if self.data_size == 1 {
            return self.data_vec[0];
        }

        let i = self.rnd.gen_range(0..self.data_size);
        self.data_vec[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let mut randomized_set = RandomizedSet::new();
        assert_eq!(randomized_set.insert(1), true);
        assert_eq!(randomized_set.remove(2), false);
        assert_eq!(randomized_set.insert(2), true);

        let random_value = randomized_set.get_random();
        assert!(random_value > 0);
        assert!(random_value < 3);

        assert_eq!(randomized_set.remove(1), true);
        assert_eq!(randomized_set.insert(2), false);
        assert_eq!(randomized_set.get_random(), 2);
    }
    #[test]
    fn test_2() {
        let mut randomized_set = RandomizedSet::new();
        assert_eq!(randomized_set.remove(0), false);
        assert_eq!(randomized_set.remove(0), false);
        assert_eq!(randomized_set.insert(0), true);

        let random_value = randomized_set.get_random();
        assert_eq!(random_value, 0);

        assert_eq!(randomized_set.remove(0), true);
        assert_eq!(randomized_set.insert(0), true);
    }
    #[test]
    fn test_3() {
        let mut randomized_set = RandomizedSet::new();
        assert_eq!(randomized_set.insert(1), true);
        assert_eq!(randomized_set.insert(10), true);
        assert_eq!(randomized_set.insert(20), true);
        assert_eq!(randomized_set.insert(30), true);

        let mut random_values = std::collections::HashSet::new();
        for _ in 0..100 {
            random_values.insert(randomized_set.get_random());
        }

        assert_eq!(
            random_values,
            std::collections::HashSet::from_iter(vec![1, 10, 20, 30])
        );
    }
}
