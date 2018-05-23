//! Template method design pattern.
//!
//! # Theory
//! Behavioural design pattern that defines the program skeleton of an algorithm in an operation, deferring some steps
//! to subclasses. This lets subclasses change certain steps of the algorithms, without changing the algorithm's
//! structure. Hence it defines an algorithm in terms of abstract operations that subclasses override to provide
//! concrete behaviour.
//!
//! Note that as in rust no subclassing is supported, this is done with traits. However, this results in a single major
//! difference from subclassing. Mainly, the algorithm method implemented in the trait does not have direct access to
//! the inner data of the objects implementing it. Therefore, if the trait requires access to internal data of the
//! object, this must be performed using a getter function defined in the trait.
//!
//! # Participants
//! - `Ord`: the trait required to implement to ensure the template method works. For `sort()`, the only method required
//!   to implement is `cmp()` as `min()` and `max()` can be deduced from `cmp()`. In OO languages, this is replaced by
//!   an abstract class for more flexibility.
//! - `Object`: the concrete class implementing the sub-operations (`sort()`) to use for the general algorithm.
//!
//! # Modifications and Strategies
//! The template method can also implement hooks that can be overridden in the trait implementations. This is nearly
//! always the case in Rust as the trait has no access to the objects data, making writing default implementations more
//! challenging.
//!
//! # Known Uses
//! - Can be fundamentally found in almost all abstract classes.
//! - This is important in class libraries


use std::cmp::Ordering;

struct Object {
    name: &'static str,
    weight: f32,
}

impl Object {
    fn new(name: &'static str, weight: f32) -> Object {
        Object {
            name,
            weight,
        }
    }

    fn print(&self) -> String {
        format!("{} with weight {}", self.name, self.weight)
    }
}

/// Requirement for `Eq`
impl PartialEq for Object {
    #[inline]
    fn eq(&self, other: &Object) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _               => false,
        }
    }
}

/// Requirement for `Ord` (this can usually be derived, but `Eq` is not implemented for `f32`)
impl Eq for Object {}

/// Requirement for `Ord`
impl PartialOrd for Object {
    #[inline]
    fn lt(&self, other: &Object) -> bool {
        match self.cmp(other) {
            Ordering::Less  => true,
            _               => false,
        }
    }

    fn partial_cmp(&self, other: &Object) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Requirement for `sort()`
impl Ord for Object {
    #[inline]
    fn cmp(&self, other: &Object) -> Ordering {
        if self.weight < other.weight {
            return Ordering::Less;
        } else if self.weight > other.weight {
            return Ordering::Greater;
        } else if self.name < other.name {
            return Ordering::Less;
        } else if self.name > other.name {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_method() {
        let mut objects = vec![
            Object::new("object1", 3_f32),
            Object::new("object2", 2_f32),
            Object::new("object3", 1_f32),
            Object::new("object4", 4_f32),
            Object::new("object5", 5_f32),
        ];

        objects.sort();

        assert_eq!(objects[0].name, "object3");
        assert_eq!(objects[1].name, "object2");
        assert_eq!(objects[2].name, "object1");
        assert_eq!(objects[3].name, "object4");
        assert_eq!(objects[4].name, "object5");
    }
}
