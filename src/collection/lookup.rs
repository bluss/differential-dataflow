use std::hash::Hash;
use std::collections::HashMap;
use radix_sort::Unsigned;

pub trait Lookup<K: Eq, V> {
    fn new() -> Self;
    fn get_ref<'a>(&'a self, &K)->Option<&'a V>;
    fn get_mut<'a>(&'a mut self, &K)->Option<&'a mut V>;
    fn entry_or_insert<F: FnMut()->V>(&mut self, K, F) -> &mut V;
    fn remove_key(&mut self, &K) -> Option<V>;
}

impl<K: Hash+Eq+'static, V: 'static> Lookup<K,V> for HashMap<K,V> {
    fn new() -> Self { HashMap::new() }
    #[inline]
    fn get_ref<'a>(&'a self, key: &K) -> Option<&'a V> { self.get(key) }
    #[inline]
    fn get_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> { self.get_mut(key) }
    #[inline]
    fn entry_or_insert<F: FnMut()->V>(&mut self, key: K, func: F) -> &mut V {
        self.entry(key).or_insert_with(func)
    }
    #[inline]
    fn remove_key(&mut self, key: &K) -> Option<V> { self.remove(key) }
}

impl<K: Eq, V> Lookup<K, V> for Vec<(K, V)> {
    fn new() -> Self { Vec::new() }
    #[inline]
    fn get_ref<'a>(&'a self, key: &K)->Option<&'a V> {
        if let Some(position) = self.iter().position(|x| &x.0 == key) {
            Some(&self[position].1)
        }
        else { None }
    }
    #[inline]
    fn get_mut<'a>(&'a mut self, key: &K)->Option<&'a mut V> {
        if let Some(position) = self.iter().position(|x| &x.0 == key) {
            Some(&mut self[position].1)
        }
        else { None }
    }
    #[inline]
    fn entry_or_insert<F: FnMut()->V>(&mut self, key: K, mut func: F) -> &mut V {
        if let Some(position) = self.iter().position(|x| x.0 == key) {
            &mut self[position].1
        }
        else {
            self.push((key, func()));
            let last = self.len() - 1;
            &mut self[last].1
        }
    }
    #[inline]
    fn remove_key(&mut self, key: &K) -> Option<V> {
        if let Some(position) = self.iter().position(|x| &x.0 == key) {
            Some(self.swap_remove(position).1)
        }
        else { None }
    }
}

impl<V: 'static, U: Unsigned> Lookup<U,V> for (Vec<Option<V>>, u64) {
    fn new() -> Self { (Vec::new(), 0) }
    #[inline]
    fn get_ref<'a>(&'a self, key: &U) -> Option<&'a V> {
        let key = (key.as_u64() >> self.1) as usize;
        if self.0.len() > key { self.0[key].as_ref() } else { None }
    }
    #[inline]
    fn get_mut<'a>(&'a mut self, key: &U) -> Option<&'a mut V> {
        let key = (key.as_u64() >> self.1) as usize;
        if self.0.len() > key { self.0[key].as_mut() } else { None }
    }
    #[inline]
    fn entry_or_insert<F: FnMut()->V>(&mut self, key: U, mut func: F) -> &mut V {
        let key = (key.as_u64() >> self.1) as usize;
        while self.0.len() <= key { self.0.push(None); }
        if self.0[key].is_none() { self.0[key] = Some(func()); }
        self.0[key].as_mut().unwrap()
    }
    #[inline]
    fn remove_key(&mut self, key: &U) -> Option<V> {
        let key = (key.as_u64() >> self.1) as usize;
        if self.0.len() > key { self.0[key].take() } else { None }
    }
}
