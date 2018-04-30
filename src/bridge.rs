//! Bridge design pattern.
//!
//! # Theory
//! Structural design pattern that is meant to decouple an abstraction from its implementation so that the two can
//! vary independently. This can be thought of as two layers of abstraction. In its essence, a bridge is build by
//! giving an object its implementation as it is built. Then, one can add implemetations independently of how the
//! object evolves over time. Moreover, in OO languages, subclassing said object allows to obtain a subtype of the
//! object without having to worry about the exact implementation the original object is using.
//!
//! # Participants
//! - `DrawingAPI`: the implementor interface. This does not have to correspond exactly to the abstraction's interface.
//!   In fact, they can be quite different. Usually, the implementors interface only provides primitive operations and
//!   the abstraction defines higher level operations based on these primitives.
//! - `DrawingRed`, `DrawingBlue`: concrete implementations of the implementor's interface.
//! - `Shape`: the abstraction which defines the abstraction's interface. It maintains a reference to (or owns) an
//!   object of type implementor (`DrawingAPI`).
//! - `ColouredShape`: extends the interface of `Shape` to provide a concrete abstraction.
//!
//! # Modifications and Strategies
//! Note that in the implementation below, `Shape` does not define many complex methods making use of the primitives
//! defined in the implementor's API. In general, one usually wants to reuse this code as much as possible without
//! explicit knowledge of the concrete implementation used. Hence one can define more methods such as `draw()` that
//! have absolutely no knowledge of the concrete implementation used but can aggregate primitive calls to build very
//! complex structures. Moreover, such a default implementation in the trait's definition results in consequences more
//! closely relatable to code inheritance in subclassing for OO languages.
//!
//! A modification of this pattern is to allow the client to change the implementation used on the fly. Using the
//! code below, this is now allowed as any concrete abstraction is closely linked to the implementor it is using.
//! However, using trait objects, this can be modified. It is mostly important when the client does not know the
//! implementation to use when it creates the concrete abstraction. One can then either define the implementor as an
//! `Option` (which results in checks for every call to the implementor / not very elegant), or using trait objects and
//! defining one implementor as the default implementor for construction.
//!
//! Implementor can also be shared across objects. This might be more challenging to implement in Rust than in other
//! more flexible languages such as C++ but it allows to construct less implementor objects.
//!
//! # Known Uses:
//! - `libg++` uses this pattern for some of its common data structures.


/// The API implemented by the implementation
trait DrawingAPI {
    fn draw_circle(&self) -> &str;
    fn draw_rectangle(&self) -> &str;
}

/// A potential implementation of the `DrawingAPI`
struct DrawingRed;
impl DrawingAPI for DrawingRed {
    fn draw_circle(&self) -> &str {
        "red circle"
    }
    fn draw_rectangle(&self) -> &str {
        "red rectangle"
    }
}

/// Another potential implementation of the `DrawingAPI`
struct DrawingBlue;
impl DrawingAPI for DrawingBlue {
    fn draw_circle(&self) -> &str {
        "blue circle"
    }
    fn draw_rectangle(&self) -> &str {
        "blue rectangle"
    }
}

/// The abstraction
trait Shape<T> where T: DrawingAPI {
    fn new(api: T) -> Self;
    fn draw_circle(&self) -> &str;
    fn draw_rectangle(&self) -> &str;
    fn draw(&self) -> String {
        format!("Shape drawing a {} and a {}", self.draw_circle(), self.draw_rectangle())
    }
}

/// A concrete object using the abstraction
struct ColouredShape<T> where T: DrawingAPI {
    api: T,
}

impl<T> Shape<T> for ColouredShape<T> where T: DrawingAPI{
    fn new(api: T) -> Self {
        ColouredShape {
            api,
        }
    }

    fn draw_circle(&self) -> &str {
        self.api.draw_circle()
    }

    fn draw_rectangle(&self) -> &str {
        self.api.draw_rectangle()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge() {
        let shape_blue = ColouredShape::new(DrawingBlue);
        assert_eq!(shape_blue.draw(), "Shape drawing a blue circle and a blue rectangle");

        let shape_red = ColouredShape::new(DrawingRed);
        assert_eq!(shape_red.draw(), "Shape drawing a red circle and a red rectangle");
    }
}

