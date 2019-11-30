use std::{collections::HashMap, hash::Hash};

pub struct Cache<K, V, F> {
    cache: HashMap<K, V>,
    compute: F,
}

impl<K, V, F> Cache<K, V, F>
where
    K: Hash + Eq,
    V: Copy,
    F: Fn(&K, &mut dyn FnMut(K) -> V) -> V,
{
    pub fn new(compute: F) -> Cache<K, V, F> {
        let cache = HashMap::new();
        Cache { cache, compute }
    }

    pub fn get(&mut self, key: K) -> V {
        get(&mut self.cache, &self.compute, key)
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.cache.insert(key, value);
    }
}

fn get<K, V, F>(cache: &mut HashMap<K, V>, compute: &F, key: K) -> V
where
    K: Hash + Eq,
    V: Copy,
    F: Fn(&K, &mut dyn FnMut(K) -> V) -> V,
{
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let value = compute(&key, &mut |k: K| get(cache, compute, k));
    cache.insert(key, value);
    value
}

#[test]
fn test_fib() {
    fn fib(&n: &u128, fib: &mut dyn FnMut(u128) -> u128) -> u128 {
        if n <= 1 {
            n
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

    let mut fib_memo = Cache::new(fib);

    assert_eq!(fib_memo.get(186), 332_825_110_087_067_562_321_196_029_789_634_457_848);
}
