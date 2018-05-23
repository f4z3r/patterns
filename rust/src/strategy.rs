//! Strategy design pattern.
//!
//! # Theory
//! Behavioural software design pattern that enables selecting an algorithm at runtime. Instead of implementing a single
//! algorithm directly, code receives runtime instructions as to which in a family of algorithms to use. Using
//! algorithms in such a way as to make them interchangeable avoids the use of conditional statements. Moreover,
//! new algorithms can be added very easily. On top of that, complex, algorithm-specific data structures are not exposed
//! to clients. The interface is then an abstraction for the algorithm.
//!
//! # Participants
//! - `Algorithm`: the strategy interface common to all supported algorithms, it forwards the responsability to the
//!   selected algorithm.
//! - `FastAlgorithm`, `SlowAlgorithm`: concrete strategies implementing the `Algorithm` interface.
//! - `SomeObject`: the context holding a reference to the used strategy. It may define an interface that lets
//!   strategies access its data.
//!
//! # Modifications and Strategies
//! _Data exchange_: passing all necessary parameters from the context to the strategy leaves them decoupled but might
//! cause information overhead. Alternatively, the context can pass itself as an argument or the strategy can hold a
//! reference to the context and request data as needed. Note that this is not possible in safe Rust as it creates
//! a cyclic dependency. Hence passing it as a parameter is a better solution, but this requires that the strategy does
//! not directly modify the context (this is not recommended anyway as it creates tighter coupling between the context
//! and the strategy).
//!
//! On top of that, note that in some languages a null strategy might need to be defined in case no strategy was
//! attached to the object in order to ensure that no null pointer derefenrecing occurs. Again, this is not a problem
//! when using safe Rust code.
//!
//! # Attention
//! Communication overheads can easily occur between a strategy and a context. The interface of the strategy defines
//! that all algorithms are provided with the same data. Hence the interface must be powerful enough to accomodate all
//! immaginable algorithms. This is where using the context as a parameter is often useful as it requires no copying
//! of data as simply a fat pointer is passed to the strategy.
//!
//! # Known Uses
//! - Line breaking algorithms
//! - Sorting algorithms
//! - Encryption/Decryption algorithms
//!
//! # State vs Strategy pattern
//! In the strategy pattern, concrete strategies are usually not aware of other strategies. Moreover, the strategy
//! usually provides only very few public methods, whereas in state machines the list of public methods can be quite
//! extensive. On top of that, states can contain state-specific data and methods, which should not be the case of
//! strategies.


/// An interface trait that all algorithms implement
trait Algorithm {
    /// Run the algorithm
    fn run(&self) -> &str;
}

/// Concrete algorithm
struct FastAlgorithm;
impl Algorithm for FastAlgorithm {
    fn run(&self) -> &str {
        "very fast algorithm"
    }
}


/// Another concrete algorithm
struct SlowAlgorithm;
impl Algorithm for SlowAlgorithm {
    fn run(&self) -> &str {
        "very slow algorithm ..."
    }
}


/// Concrete object whose behaviour is modified based on the attached algorithm
struct SomeObject {
    behaviour: Box<Algorithm>,
}

impl SomeObject {
    fn new(alg: Box<Algorithm>) -> SomeObject {
        SomeObject {
            behaviour: alg,
        }
    }

    fn set_behaviour(&mut self, alg: Box<Algorithm>) {
        self.behaviour = alg;
    }

    fn run(&self) -> &str {
        self.behaviour.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy() {
        let mut object = SomeObject::new(Box::new(SlowAlgorithm));
        assert_eq!(object.run(), "very slow algorithm ...");

        object.set_behaviour(Box::new(FastAlgorithm));
        assert_eq!(object.run(), "very fast algorithm");
    }
}

