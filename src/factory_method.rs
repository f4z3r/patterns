//! Factory method example.
//!
//! # Theory
//! Use a "virtual" constructor. Instantiation is then delegated to the class implementing the interface.
//! This allows the clients to use the interface to interact with concrete objects without knowing their exact
//! types.
//!
//! # Participants
//! - `Car`: the product to create in the "factory". It defines the interface for objects created in said "factory".
//! - `Sedan`: the concrete product that implements the interface of the general product above.
//! - `Factory`: the creator (factory) which declares the factory method, which returns an object of type `Car`. This
//!   this can also define a default `Car` object if this is necessary/appropriate.
//! - `SedanFactory`: the class overriding the factory method from the abstract creator to return a specific instance
//!   of a concrete product (`Sedan`).
//!
//! # Modifications and Strategies
//! The creator can take an additional parameter specifying which product to create. All the concrete creators must then
//! implement the same interface. This makes it easy to extend or change the products that the creator creates.
//!
//! # Known Uses
//! - Junit Test
//!
//! # Comments
//! Note that using traits, the same result can be achieved by making sure all cars implement a `make_car` static method
//! that returns a car of the type given by the class implementing the trait. This results in a factory method shared
//! by all cars to build `Car` objects. Of course, this reduces the information hiding about the type of the `Car`
//! created as usually a class would returns its own type (but note it still has the flexibility to return another
//! type of `Car`).

/// A trait defining the behaviour of a car.
trait Car {
    /// Get the type of a car.
    ///
    /// # Returns
    /// The car type (str).
    fn get_type(&self) -> &'static str;
}

/// A trait defining the behaviour of a CarFactory. This is the factory.
trait CarFactory {
    /// Make a car. This is the factory method
    ///
    /// # Returns
    /// A Car.
    fn make_car(&self) -> Box<Car>;
}

/// A car of type sedan.
struct Sedan;

impl Car for Sedan {
    fn get_type(&self) -> &'static str {
        "Sedan"
    }
}

/// A factory building sedans.
struct SedanFactory;

impl CarFactory for SedanFactory {
    fn make_car(&self) -> Box<Car> {
        Box::new(Sedan {})
    }
}

/// Note that this can also be implemented with advanced traits using types. In general this is much cleaner, but
/// exposes the real type of the object to the client. This might however, not be a problem in some scenarios. In terms
/// of performance, this is prefered, as no dynamic lookup is required on method calls on the objects created by the
/// factory method.
trait CarFactory2 {
    type CarType: Car;

    fn make_car(&self) -> Self::CarType;
}

struct BetterSedanFactory;

impl CarFactory2 for BetterSedanFactory {
    type CarType = Sedan;

    fn make_car(&self) -> Sedan {
        Sedan {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_method() {
        let sedan_factory = SedanFactory{};
        let sedan = sedan_factory.make_car();
        assert_eq!(sedan.get_type(), "Sedan");

        let better_sedan_factory = BetterSedanFactory {};
        let better_sedan = better_sedan_factory.make_car();
        assert_eq!(better_sedan.get_type(), "Sedan");
    }
}
