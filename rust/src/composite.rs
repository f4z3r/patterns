//! Composite design pattern example.
//!
//! # Theory
//! This is a partitioning design pattern. It describes how a group of obejcts that is trated the
//! same way as a single instance of the same type of object. The idea is to compose objects into
//! tree structures to represent part-whole hierarchies. Composite lets clients treat objects and
//! compositions of objects uniformly.
//!
//! A recursive composition allows to treat primitive and complex objects in the same way. Clients
//! can then ignore which object they are dealing with and treat all of them uniformly. Wherever
//! client code expects a primitive object, it can also take a composite.
//!
//! An abstract class represents both the primitives and their containers. This meakes client code
//! independent of internal representations of objects.
//!
//! In general, a `Composite` object can consist of any number of `Leaf`s and other `Composite`
//! objects.
//!
//! ## Note
//! Composition varies from Aggregation in that when the whole is deleted, all its parts are also
//! deleted. Moreover, a part can only be part of a single whole. In aggregation, individual parts
//! can continue to exist if their parent was deleted. Moreover, they can be part of several
//! wholes.
//!
//! # Participants
//! - `Graphic`: is the component delcaring the interface for objects in the composition and for
//!   accessing and managing the object. If desired, the managing methods much be
//!   defined in here and potentially implement a default behaviour.
//! - `Ellipse`: is a leaf object that represents and defines the behaviour of primitive objects.
//!   This has no children.
//! - `CompositeGraphic`: a composite that manages children. It defines the behaviour for objects
//!   that have children and stores the child components and implements child-related operations
//!   from the `Graphic` interface. Usually requests are forwarded to the leafs (`Ellipse`).
//!
//! # Modifications and Strategies
//! The traversal and management of composite structures can be simplified by maintaining explicit
//! references to the parent object in the component instances.
//!
//! Moreover, note that the composite pattern is usually used in combination with the Decorator
//! pattern so they will share a common parent class. In that case, the decorators also have to
//! support operations such as `add()`, `remove()` and `get_child()` from the component
//! interface.
//!
//! # Attention
//! Note that if the object management is left to the client, it is very difficult to restrict
//! which objects can be part of a composite as they only need to implement the component interface.
//! If this needs to be restricted, runtime checks are necessary.
//!
//! In languages that do not have a borrow checker such as Rust, one must be careful not to create
//! cyclic compositions. This is not possible in Rust as the parent takes ownership of all its
//! children. Moreover, one might need to be careful that components can only be added to a single
//! composite. Again, this is not an issue when using Rust.
//!
//! # Known Uses
//! - JDRAW - Grouping
//! - JComponents in Swing: leafs are JLabel, JCheckbox, etc.


/// A trait defining the (graphical) component.
trait Graphic {
    /// Prints the type of the graphic
    fn print(&self) -> String;
}

/// The composite.
struct CompositeGraphic {
    children: Vec<Box<Graphic>>,
}

impl CompositeGraphic {
    /// Constructor
    fn new() -> Self {
        CompositeGraphic{
            children: Vec::new(),
        }
    }

    /// Add a component
    fn add(&mut self, graphic: Box<Graphic>) {
        self.children.push(graphic);
    }

    /// Remove the last added component
    fn remove(&mut self) {
        self.children.pop();
    }
}

impl Graphic for CompositeGraphic {
    fn print(&self) -> String {
        let mut result = String::new();
        for part in &self.children {
            result.push_str(&part.print());
        }
        result
    }
}


/// Leaf
struct Ellipse;

impl Graphic for Ellipse {
    fn print(&self) -> String {
        String::from("Ellipse")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite() {
        let ellipse1 = Ellipse{};
        let ellipse2 = Ellipse{};
        let ellipse3 = Ellipse{};
        let ellipse4 = Ellipse{};

        let mut graphic1 = CompositeGraphic::new();
        let mut graphic2 = CompositeGraphic::new();
        let mut graphic3 = CompositeGraphic::new();

        graphic2.add(Box::new(ellipse1));
        graphic2.add(Box::new(ellipse2));
        graphic2.add(Box::new(ellipse3));

        graphic3.add(Box::new(ellipse4));

        graphic1.add(Box::new(graphic2));
        graphic1.add(Box::new(graphic3));

        assert_eq!(graphic1.print(), "EllipseEllipseEllipseEllipse");
    }
}
