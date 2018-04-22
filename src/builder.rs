//! Builder design pattern.
//!
//! # Theory
//! Creational design pattern trying to solve the problem of different representations of a complex object. This is
//! particularly useful when the algorithms for building an object should be independent of the parts that make up the
//! object. The construction process then allows for different representations for the object that is constructed.
//!
//! # Participants
//! - `Builder`: the builder, it specifies n bastract interface for creating parts of a product object.
//! - `CarBuilder`: a concrete builder implementing the `` interface. It defines and keeps track of the representation
//!   it creates. It also provides an interface for retrieving the product it creates.
//! - `CarBuilderDirector`: the director, responsible for the construction of the object using the builder interface.
//! - `Car`: the product. It represents the complex object under construction. A concrete builder builds the product's
//!   internal representation and defines the process by which it is assembled.
//!
//! # Modifications and Strategies
//! Note that this pattern is similar to the abstract factory pattern. The main difference is that the builder pattern
//! focuses on the individual stages of building the object step by step. Moreover, an abstract factory usually builds
//! a familty of related objects instead of a single complex one. Moreover, the abstract factory returns an object
//! after each call to a constructor function in the factory, whereas the builder only returns the final object once
//! the `build()` function is called.
//!
//! Note that a builder usually builds a composite object. Hence these patterns are usually used together.
//!
//! # Known Uses
//! - Text converters



/// A car
#[derive(Clone)]
struct Car {
    /// Number of wheels the car has
    pub wheels: u8,
    /// Number of seats in the car
    pub seats: u8,
    /// The colour of the car
    pub colour: String,
}

impl Car {
    fn new() -> Self {
        Car {
            wheels: 0,
            seats: 0,
            colour: "black".to_string(),
        }
    }

    /// Returns a description of the car
    fn description(&self) -> String {
        format!("This is a {} car with {} wheels and {} seats.", self.colour, self.wheels, self.seats)
    }
}

trait Builder {
    type Product;

    /// Set the wheel count of the car. Note that this is a blank method in order to allow concrete builders to only
    /// implement the methods they require to build the complex object.
    #[allow(unused_variables)]
    fn set_wheels(&mut self, num: u8) {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn set_seats(&mut self, num: u8) {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn set_colour(&mut self, colour: String) {
        unimplemented!();
    }

    /// This method is required for each builder.
    fn build(&self) -> Self::Product;
}

struct CarBuilder {
    car: Car,
}

impl CarBuilder {
    fn new() -> Self {
        CarBuilder {
            car: Car::new(),
        }
    }
}

impl Builder for CarBuilder {
    type Product = Car;

    fn set_wheels(&mut self, num: u8) {
        self.car.wheels = num;
    }

    fn set_seats(&mut self, num: u8) {
        self.car.seats = num;
    }

    fn set_colour(&mut self, colour: String) {
        self.car.colour = colour;
    }

    fn build(&self) -> Car {
        self.car.clone()
    }
}

struct CarBuilderDirector {
    builder: CarBuilder,
}

impl CarBuilderDirector {
    fn new() -> Self {
        CarBuilderDirector {
            builder: CarBuilder::new(),
        }
    }

    fn construct(&mut self) -> Car {
        self.builder.set_colour("red".to_string());
        self.builder.set_wheels(4);
        self.builder.set_seats(5);
        self.builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let mut director = CarBuilderDirector::new();
        let car = director.construct();

        assert_eq!(car.description(), "This is a red car with 4 wheels and 5 seats.");
    }
}
