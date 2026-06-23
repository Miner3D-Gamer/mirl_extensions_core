/// A trait for structs that are supposed to act like lists
///
/// The required helper trait [`ListLikeHelper`] is automatically implemented for all structs that implement this trait
pub const trait ListLike<T, Idx>:
    core::ops::Index<Idx>
    + core::ops::IndexMut<Idx>
    + ListLikeHelper<T, Idx>
    // + std::iter::Iterator
    + std::iter::IntoIterator
{
    /// The iterator for the list type
    type Iterator<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;

    /// Iter over the held items 
    fn iter(&self) -> Self::Iterator<'_>;

    /// The mutable iterator for the list type
    type IteratorMut<'a>: Iterator<Item = &'a mut T>
    where
        Self: 'a,
        T: 'a;

    /// Iter over the held items mutably
    fn iter_mut(&mut self) -> Self::IteratorMut<'_>;

    /// Get the internal value by reference without checking the bounds
    ///
    /// # Safety
    /// The caller must assure that the idx is valid
    unsafe fn get_unchecked(&self, index: Idx) -> &T;

    /// Get the internal value by mutable reference without checking the bounds
    ///
    /// # Safety
    /// The caller must assure that the idx is valid
    unsafe fn get_unchecked_mut(&mut self, index: Idx) -> &mut T;
    /// Add an item into the container and return a mutable reference to it
    fn push_mut(&mut self, value: T) -> &mut T;
    /// Try to remove the value
    fn try_remove(&mut self, index: Idx) -> Option<T>;
    #[must_use]
    /// Swap 2 values without disrupting the rest of the list
    ///
    /// Returns true if everything went well. False when the idx was too big/small or something else failed
    fn swap_values(&mut self, a: Idx, b: Idx) -> bool;
    #[must_use]
    /// Get how many items are in the container
    fn len(&self) -> usize;
    /// Remove the last item from the container and return it if it exists
    fn pop(&mut self) -> Option<T>;
    /// Insert an item anywhere into the container
    fn try_insert_mut(&mut self, index: usize, value: T) -> Option<&mut T>;
    /// Replace the value in the container with the given value
    fn try_replace(&mut self, index: usize, value: T) -> Option<T>;
    /// Reserve space for X more values if possible
    ///
    /// # Errors
    /// When it fails to allocate more which most often happens when the total length goes beyond [`isize::MAX`]
    fn try_reserve(
        &mut self,
        amount: usize,
    ) -> Result<(), std::collections::TryReserveError>;
    /// Returns the idx at which the given item was found
    fn find_position(&self, item: &T) -> Option<Idx>
    where
        T: std::cmp::PartialEq;
    /// Remove all values within the container
    fn clear(&mut self);
    /// Remove an item from the container
    ///
    /// This function is unsafe and may cause a panic, for the non panic version call [`try_remove`](ListLike::try_remove)
    fn remove(&mut self, index: Idx) -> T {
        self.try_remove(index).unwrap()
    }
    // fn iter<'a>(&'a self) -> &'a dyn core::iter::Iterator<Item = &T>;
}

/// A few helper functions for any list like object to reduce code duplication
///
/// It is automatically implemented for all structs that implement [`ListLike`]
pub const trait ListLikeHelper<T, Idx> {
    // fn remove(&mut self, index: Idx) -> T;
    /// Add an item to the container
    fn push(&mut self, value: T);
    /// Checks if the current amount of items is equal to zero
    fn is_empty(&mut self) -> bool;
    /// Insert an item anywhere into the container
    fn insert(&mut self, index: usize, value: T);
    /// Replace the value in the container with the given value
    ///
    /// This function is unsafe and may cause a panic, for the non panic version call [`try_replace`](ListLike::try_replace)
    fn replace(&mut self, index: usize, value: T) -> T;
    /// Check if the container contains the given item
    fn contains(&self, item: &T) -> bool
    where
        T: std::cmp::PartialEq;
    /// Reserve space for X more values
    ///
    /// This function is unsafe and may cause a panic, for the non panic version call [`try_reserve`](ListLike::try_reserve)
    fn reserve(&mut self, amount: usize);
}

impl<T> ListLike<T, usize> for Vec<T> {
    type Iterator<'a>
        = std::slice::Iter<'a, T>
    where
        T: 'a;

    type IteratorMut<'a>
        = std::slice::IterMut<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.as_slice().iter()
    }

    fn iter_mut(&mut self) -> Self::IteratorMut<'_> {
        self.as_mut_slice().iter_mut()
    }
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe { self.as_slice().get_unchecked(index) }
    }
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.as_mut_slice().get_unchecked_mut(index) }
    }
    default fn push_mut(&mut self, value: T) -> &mut T {
        self.push_mut(value)
    }
    default fn try_remove(&mut self, index: usize) -> Option<T> {
        mirl_std_exposed::vec::vec_try_remove(self, index)
    }
    default fn swap_values(&mut self, a: usize, b: usize) -> bool {
        if a > self.len() || b > self.len() {
            return false;
        }
        unsafe {
            self.swap_unchecked(a, b);
        }
        true
    }
    default fn len(&self) -> usize {
        self.len()
    }
    default fn try_insert_mut(
        &mut self,
        index: usize,
        value: T,
    ) -> Option<&mut T> {
        if index > self.len() {
            return None;
        }
        Some(self.insert_mut(index, value))
    }
    default fn pop(&mut self) -> Option<T> {
        self.pop()
    }
    default fn try_replace(&mut self, index: usize, value: T) -> Option<T> {
        if index >= self.len() {
            return None;
        }
        let mut value = value;

        core::mem::swap(&mut self[index], &mut value);

        Some(value)
    }
    default fn find_position(&self, item: &T) -> Option<usize>
    where
        T: std::cmp::PartialEq,
    {
        self.as_slice().iter().position(|x| (*x).eq(item))
    }
    default fn try_reserve(
        &mut self,
        amount: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.try_reserve(amount)
    }
    default fn clear(&mut self) {
        self.clear();
    }
}
// TODO: Merge [`ListLikeHelper`] into [`ListLike`]
impl<S: ListLike<T, Idx>, T, Idx> ListLikeHelper<T, Idx> for S
where
    for<'a> &'a Self: IntoIterator,
    for<'a> &'a mut Self: IntoIterator,
{
    fn push(&mut self, value: T) {
        self.push_mut(value);
    }
    fn is_empty(&mut self) -> bool {
        self.len() == 0
    }
    fn insert(&mut self, index: usize, value: T) {
        self.try_insert_mut(index, value);
    }
    fn replace(&mut self, index: usize, value: T) -> T {
        self.try_replace(index, value).unwrap()
    }
    fn contains(&self, item: &T) -> bool
    where
        T: std::cmp::PartialEq,
    {
        self.find_position(item).is_some()
    }
    fn reserve(&mut self, amount: usize) {
        self.try_reserve(amount).unwrap();
    }
}
