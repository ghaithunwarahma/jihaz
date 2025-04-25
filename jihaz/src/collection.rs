

pub struct SmallKeyArray<K, V, const N: usize> {
    array: [(K, V); N]
}

impl<K: Copy + PartialEq, V, const N: usize> SmallKeyArray<K, V, N> {
    pub fn new_same_key(key: K, array: [V; N]) -> SmallKeyArray<K, V, N> {
        Self { array: array.map(|v| (key, v)) }
    }
}

// impl<K, V, const N: usize> SmallKeyArray<K, V, N> {
//     /// Returns an array of the same size as self, with function f applied to each element in order.
//     ///
//     /// If you don't necessarily need a new fixed-size array, consider using Iterator::map instead.
//     pub fn map<F, U>(self, f: F) -> SmallKeyArray<K, U, N> 
//     where
//         F: FnMut(V) -> U,
//     {
//         Self { array: self.array.map(|k, v| (k, f(v))) }
//     }
// }

impl<K: PartialEq, V, const N: usize> SmallKeyArray<K, V, N> {
    pub fn new(array: [(K, V); N]) -> SmallKeyArray<K, V, N> {
        Self { array }
    }

    pub fn set(&mut self, key: K, value: V) {
        for (k, v) in &mut self.array {
            if *k == key {
                *v = value;
                return;
            }
        }
    }

    pub fn get_mut(&mut self, key: &K) -> &mut V {
        for (k, v) in &mut self.array {
            if k == key {
                return v;
            }
        }
        panic!("key not found in SmallKeyArray");
    }

    pub fn get(&self, key: &K) -> &V {
        for (k, v) in &self.array {
            if k == key {
                return v;
            }
        }
        panic!("key not found in SmallKeyArray");
    }

    pub fn with_mut<F, R>(&mut self, key: &K, f: F) -> R
    where
        F: FnOnce(&mut V) -> R
    {
        for (k, v) in &mut self.array {
            if k == key {
                return f(v);
            }
        }
        panic!("key not found in SmallKeyArray");
    }

    pub fn with<F, R>(&self, key: &K, f: F) -> R
    where
        F: FnOnce(&V) -> R
    {
        for (k, v) in &self.array {
            if k == key {
                return f(v);
            }
        }
        panic!("key not found in SmallKeyArray");
    }
}