//! Example of the singleton design pattern.
//!
//! # Theory
//! The purpose of this design pattern is to ensure only a single instance of a class exists and provide a single point
//! of access to it. The class itself ensures that no other instance is created, alternative to global variables in a
//! namespace.
//!
//! Client can exclusively access the singleton instance using the `get_instance()` method which provides controlled
//! access.
//!
//! Note that a static object has the following downsides compared to a singleton object:
//! 1. Runtime information might be required for the initial creation of the singleton object.
//! 2. Static methods cannot implement an interface.
//! 3. It may be necessary to support having a limited number of singletons (larger than 1), which is not possible to
//!    perform using static classes.
//!
//! # Participants
//! - `Singleton`: the singleton class defining the `get_instance()` method which lets clients access the unique
//!   instance of the class. Any attempt to initialise the singleton from the outside must be prevented.
//!
//! # Modifications and Strategies
//! - `get_instance()` might use lazy initialisation, hence only creating the object when first required.
//! - The singleton might be subclassed to allow refinement of operations and representation (obviously not in Rust).
//!
//! # Note
//! If the singleton is used to provide global data, there might be a more elegant solution. Global data is usually not
//! necessary and should largely be reduced.
//!
//! Moreover, a singleton usually promotes tight coupling between classes, which should be avoided. On top of that, this
//! coupling can be difficult to identify.
//!
//! # Known Uses
//! - A music player only playing a single song at a time.
//! - Cache providing fast access to objects used often - a single instance controls the cache.
//! - Only a single instance should access some hardware item.
//! - State pattern instances are usually implemented as singletons.


// The following code is copied from stack exchange
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::mem;

#[derive(Clone)]
struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Arc<Mutex<u8>>,
}


/// Note that by the inner workings of Rust, one cannot make this `get_instance()` class to be part of `SingletonReader`.
/// Moreover, note that this entire code can be simplied significantly using the `lazy-static` crate.
fn get_instance() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                inner: Arc::new(Mutex::new(0)),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

// Note that in order to access the data of the singleton, one must lock its internal lock. This can be automated using
// the `Deref` and `DerefMut` traits.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton() {
        {
            let s_1 = get_instance();
            let mut data_1 = s_1.inner.lock().unwrap();
            *data_1 = 0_u8;
        }

        {
            let s_2 = get_instance();
            let mut data_2 = s_2.inner.lock().unwrap();
            *data_2 = 1_u8;
        }

        let tester = get_instance();
        let data = tester.inner.lock().unwrap();
        assert_eq!(*data, 1_u8);
    }
}
