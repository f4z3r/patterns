//! Iterator design pattern.
//!
//! # Theory
//! Iterators provide access to the elements of an aggregate object sequentially without exposing its underlying
//! representation. Moreover, one might want to traverse the internal data of an object in different ways depending on
//! the context. On top of that, several traversals might be possible concurrently.
//!
//! Iterators have many applications in modern programming, ranging from their omnipresence in functional programming
//! to their practical use in OO programming. There is an abundance of general information about iterators online.
//!
//! # Participants
//! - `Iterator`: provides an iterface for accessing and traversing elements (this is a `std` trait in this case).
//! - `Fibonacci`: a concrete iterator implementing the `Iterator` interface. It keeps track of the current position
//!   in the aggregate object.
//! - `IntoIterator`: an interface for aggregators to create iterators (this is a `std` trait in this case).
//! - `CustomList`: a concrete aggregate implementing the `IntoIterator` trait.
//!
//! # Modifications and Strategies
//! An common question is where the traversal algorithm should be defined. In Rust, using the standard library traits,
//! one does not have a choice and needs to implement the algorithm inside the iterator itself. However, the general
//! pattern is more flexible and allows the algorithm to be actually defined inside the aggregate object.
//!
//! Null iterators can be useful for handling boundary scenarios. This is not necessary in Rust because of the `Option`
//! an iterator's `next()` function returns.
//!
//!
//! # Notes
//! The implementation of iterators is trivial in Rust, as it is provided as a mixin trait by the standard library.
//! This trait allows a very simple function to be implemented, yet provide a very large set of features. However,
//! should one want to implement several different kinds of iterators for a single object, one cannot rely on the
//! trait. This is due to the invariant method name to retrieve the iterator from the object, making two implementations
//! of the `IntoIterator` trait impossible as they create conflict for method lookup.
//!
//! Note that modifying values inside an iterator can be dangerous as it modifies the aggregate. This is not the case
//! in rust as creating an iterator consumes the aggregate, hence preventing a programmer from accessing the implicitly
//! modified aggregate. __Robust iterators__ guarantees that insertios and removals on the aggregate object won't
//! interfere with traversals, and it does it without copying the aggregate.
//!
//! # Known Uses
//! Legit everywhere.

/// The object to iterate over
struct CustomList {
    fib: Fibonacci,
    name: String,
}
impl IntoIterator for CustomList {
    type Item = u32;
    type IntoIter = Fibonacci;
    fn into_iter(self) -> Self::IntoIter {
        self.fib
    }
}

struct Fibonacci {
    current: u32,
    next: u32,
}
impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;

        self.current = self.next;
        self.next = new_next;

        Some(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let list = CustomList {
            name: String::from("my list"),
            fib: Fibonacci { current: 0, next: 1 },
        };



        let mut list_iter = list.into_iter();
        assert_eq!(Some(1), list_iter.next());
        assert_eq!(Some(1), list_iter.next());
        assert_eq!(Some(2), list_iter.next());
        assert_eq!(Some(3), list_iter.next());
        assert_eq!(Some(5), list_iter.next());
        assert_eq!(Some(8), list_iter.next());
        assert_eq!(Some(13), list_iter.next());
        assert_eq!(Some(21), list_iter.next());
        assert_eq!(Some(34), list_iter.next());
        assert_eq!(Some(55), list_iter.next());
    }
}
