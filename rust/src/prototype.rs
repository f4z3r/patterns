//! Prototype design pattern example.
//!
//! # Theory
//! Specify the kind of objects to create using a prototypical instance, and create the new objects by copying the
//! prototype. Note that the prototype is cloned to create a new instance, and this copying is delegated to the instance
//! itself.
//!
//! This can significantly reduce the number of objects in the system by using object composition instead of
//! subclassing. The concrete classes of the composed objects are hidden from the client, hence making the system
//! independent from the way it is used, and from the way objects are created, composed and represented.
//!
//! This approach should be used when instances of a class can have one of a few different combinations of state. It
//! then simplifies the initialisation process as classes needn't be instantiated manually.
//!
//! # Participants
//! - `Protoptype`: declares a `clone()` interface for cloning itself.
//! - `ConcretePrototype`: implements the `Prototype` interface.
//! - `Client`: creates new objects by cloning the prototype rather than calling constructors.
//!
//! # Modifications and Strategies
//! A `PrototypeManager` could be used to handle possible prototypes if several exist. Moreover, note that one needs
//! to ensure whether one wants to perform deep or shallow copies based on the object's properties and usage.
//!
//! # Known Uses
//! - The copy & paste function.
//! - The gamma function
//!
//! # Notes
//! In rust this boils down to implementing the `Clone` trait and passing the client a clone trait object. Then all
//! clonable objects can be cloned via this object. Hence no real example is given here. Of course, abstract classes
//! can be used for this in other languages to make the intent clearer, but the abstract superclass requires
//! subclassing, which is not supported in rust.




#[cfg(test)]
mod tests {
    #[test]
    fn test_prototype() {
        assert!(true);
    }
}
