// TODO: Add this trait to ahash and indexmap

/// A trait that combines standard capabilities across Maps
///
/// Structs that implement this trait:
/// - `std::collections::BTreeMap`
/// - `std::collections::HashMap`
///
/// - `indexmap::IndexMap` (Under `indexmap` flag)
pub const trait MapLike<K, V> {
    /// Insert a value using a key, return `Some(old_value)` if key already existed
    fn insert(&mut self, key: K, val: V) -> Option<V>;
    /// Get a value using the key
    fn get(&self, key: &K) -> Option<&V>;
    /// Get a mutable value using the key
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    /// Remove a value using the key
    fn remove(&mut self, key: &K) -> Option<V>;
    /// Get all keys and all values
    ///
    /// TODO: Somehow remove the Box while keeping the trait dyn
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a K, &'a V)> + '_>;
    /// Find the key of a value
    fn find_key(&self, value: &V) -> Option<&K>
    where
        V: PartialEq;

    /// The equivilant of `len()`
    fn len(&self) -> usize;
    /// Checks if the current list is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// A trait for maps that support indexing operations
pub const trait IndexedMap<K, V>: MapLike<K, V> {
    /// Index the key of a value, return the index at which the value was found
    fn index(&self, value: &V) -> Option<usize>
    where
        V: PartialEq;
    /// Get the value at the index
    fn get_index(&self, index: usize) -> Option<&V>;
    /// Get the mutable value at the index
    fn get_index_mut(&mut self, index: usize) -> Option<&mut V>;
}
#[cfg(feature = "std")]
impl<K: core::hash::Hash + core::cmp::Ord, V: core::hash::Hash> MapLike<K, V>
    for std::collections::BTreeMap<K, V>
{
    fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.insert(key, val)
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
    fn iter(&self) -> Box<dyn Iterator<Item = (&K, &V)> + '_> {
        Box::new(self.iter())
        // let keys = self.keys();

        // let mut output = Vec::new();
        // for i in keys {
        //     output.push((i, self.get(i).easy_unwrap_unchecked()));
        // }

        // output
    }
    fn find_key(&self, value: &V) -> Option<&K>
    where
        V: PartialEq,
    {
        self.iter().find(|x| x.1.eq(value)).map(|x| x.0)
    }
    fn len(&self) -> usize {
        self.len()
    }
}
#[cfg(feature = "std")]
impl<
    K: core::hash::Hash + core::cmp::Ord,
    V: core::hash::Hash,
    S: ::std::hash::BuildHasher,
> MapLike<K, V> for std::collections::HashMap<K, V, S>
{
    fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.insert(key, val)
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
    fn iter(&self) -> Box<dyn Iterator<Item = (&K, &V)> + '_> {
        Box::new(self.iter())
        // let keys = self.keys();

        // let mut output = Vec::new();
        // for i in keys {
        //     output.push((i, self.get(i).easy_unwrap_unchecked()));
        // }

        // output
    }
    fn find_key(&self, value: &V) -> Option<&K>
    where
        V: PartialEq,
    {
        self.iter().find(|x| x.1.eq(value)).map(|x| x.0)
    }
    fn len(&self) -> usize {
        self.len()
    }
}

// /// A trait to index into a [Value]
// pub trait IndexValue<K, V, Mapping: Map<K, V>> {
//     /// Index into the given [Value] and return a [Value] if the requested [Value] exists, otherwise return [None]
//     fn index_value<'a>(&self, v: &'a Mapping) -> Option<&'a V>;

//     /// Index into the given [Value] and return a mutable [Value] if the requested [Value] exists, otherwise return [None]
//     fn index_value_mut<'a>(&self, v: &'a mut Mapping) -> Option<&'a mut V>;
// }

// impl<W: IndexedMap<K, V> + Map<K, V>, K, V> IndexValue<K, V, W> for usize {
//     fn index_value<'a>(&self, v: &'a W) -> Option<&'a V> {
//         v.get_index(*self)
//     }
//     fn index_value_mut<'a>(&self, v: &'a mut W) -> Option<&'a mut V> {
//         v.get_index_mut(*self)
//     }
// }

// impl<W: IndexedMap<K, V> + Map<K, V>, K, V> IndexValue<K, V, W> for str {
//     fn index_value<'a>(&self, v: &'a W) -> Option<&'a V> {
//         match v {
//             Value::Container(container) => match container {
//                 ContainerValue::Map(val) => {
//                     let origin = self.to_string();
//                     for (k, v) in val {
//                         if let Some(s) = k.as_string_ref()
//                             && origin.eq(s)
//                         {
//                             return Some(v);
//                         }
//                     }
//                     None
//                 }
//                 ContainerValue::Vec(_) => None,
//             },
//             Value::Simple(_) => None,
//         }
//     }
//     fn index_value_mut<'a>(
//         &self,
//         v: &'a mut Value<W>,
//     ) -> Option<&'a mut W::Inner> {
//         match v {
//             Value::Container(container) => match container {
//                 ContainerValue::Map(val) => {
//                     let origin = self.to_string();
//                     for (k, v) in val {
//                         if let Some(s) = k.as_string_ref()
//                             && origin.eq(s)
//                         {
//                             return Some(v);
//                         }
//                     }
//                     None
//                 }
//                 ContainerValue::Vec(_) => None,
//             },
//             Value::Simple(_) => None,
//         }
//     }
// }
