//! Decorator example.
//!
//! # Theory
//! Allows to attach additional responsabilities to an object dynamically. This is a flexible
//! alternative to subclassing for extending functionality. It allows to add responsabilities
//! to a single object instead of an entire class. This adds the flexibility of dynamically
//! deciding which decorator to add how and when. Moreover, it allows for any combination of
//! decorators without adding extra classes. As any number of decorators can be nested recursively,
//! this gives an unlimited number of responsabilities you can add.
//!
//! On top of that, functionality can be incremented at runtime, since the complexity is not
//! statically part of the object but can be added as it is needed.
//!
//! # Praticipants
//! - `Shape`: defines the interface of objects that can have dynamically added responsabilities.
//!   This should be lightweight since several components can be encapsulated into each other.
//! - `Circle`: this is a concrete component implementing the `Shape` component interface. It
//!   defines a primitive object which can have additional responsabilities attached.
//! - `ColouredShape`: this is the decorator, maintaining a reference to a component object and
//!   satisfies the components interface. It forwards requests to the component while adding
//!   additional actions if necessary.
//!
//! # Modifications and Strategies
//! The composite pattern is often used with the decorator. See modifications in its file to see
//! how this affects both patterns.
//!
//! Note that the decorator pattern can be seen as a degenrate version of the composite pattern that
//! only holds one component. The main difference being that the decorator adds additional
//! responsabilities to the component.
//!
//! # Attention
//! The decorator and its component are not identical, hence object comparison can lead to
//! unexpected results. (Comparing a `Circle` with a `ColouredShape` containing a `Circle`)
//!
//! # Known Uses
//! - ScrollPane in Java
//! - JDRAW - Border, FixedPosition, Blinking, etc.
//! - Output streams in Java


/// A simple circle
struct Circle {
    radius: f32,
}

/// A shape
trait Shape {
    /// Give a description of the shape
    fn description(&self) -> String;
}

impl Shape for Circle {
    fn description(&self) -> String {
        format!("Circle of radius {}", self.radius)
    }
}

/// Any coloured shape
struct ColouredShape<T> {
    shape: T,
    colour: String,
}

impl<T: Shape> Shape for ColouredShape<T> {
    fn description(&self) -> String {
        format!("{} which is coloured {}", self.shape.description(), self.colour)
    }
}

impl<T> ColouredShape<T> where T: Shape {
    /// Constructor
    fn new(colour: &str, shape: T) -> Self {
        ColouredShape {
            shape,
            colour: colour.to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decorator() {
        let circle = Circle { radius: 3.5 };
        let red_circle = ColouredShape::new("red", circle);
        assert_eq!("Circle of radius 3.5 which is coloured red", red_circle.description());

        let green_red_circle = ColouredShape::new("green", red_circle);
        assert_eq!("Circle of radius 3.5 which is coloured red which is coloured green",
                   green_red_circle.description());
    }
}
