//! Abstract factory example.
//!
//! # Theory
//! The mainly consists of a combination between the factory method design pattern and the strategy pattern. It provides
//! an interface for creating families of related or dependent objects without specifying their concrete classes.
//! It allows to make code more flexible by avoiding hard coding. Clients can then create objeects solely through the
//! abstract interface defined by an abstract class, not a particular concrete class.
//!
//! This also guarantees consistency among products and enhances configurability and extensitibility.
//!
//! # Participants
//! - `Button`: a generic object to be created by the factory.
//! - `Window`: another generic object to be created by the factory.
//! - `Factory`: an abstract factory interface that factories will use to create concrete objects.
//! - `OSXButton`, `LinuxButton`: concrete products of type `Button` that are created by the concrete factories.
//! - `OSXWindow`, `LinuxWindow`: concrete products of type `Window` that are created by the concrete factories.
//! - `OSX`: a concrete factory building `OSXButton` and `OSXWindow` objects (implements the operations to create these
//!   concrete objects).
//! - `Linux`: a concrete factory building `LinuxButton`and `LinuxWIndow` objects (implements the operations to create
//!   these concrete objects).
//! - The client. This is `main` in this case. It would usually be another class (potentially unknown as created by
//!   library user).
//!
//! # Modifications and Strategies
//! Usually this can be combined with a Singleton pattern as only a single factory is required for the creation of the
//! concrete objects.
//!
//! Note that a default configuration might be provided by the interface if appropriate.
//!
//! # Attention
//! Adding new kinds of products to the abstract factory require a default implementation. This can be handled "the
//! Java way" by raising an exception (optional methods) or it could provide a default object.
//!
//! # Known Uses
//! Creation of UI controls.

/// A trait defining a button.
pub trait Button {
    /// Returns the label on the button.
    ///
    /// # Returns
    /// The label of the button (&str).
    fn paint(&self) -> &str;
}


/// A generic window trait.
pub trait Window {
    /// Returns the size of the window.
    ///
    /// # Returns
    /// The size of the window as a (u32, u32) tuple.
    fn size(&self) -> (u32, u32);
}

/// A trait defining an abstract factory.
pub trait Factory {
    /// Creates a button.
    ///
    /// # Returns
    /// A button trait object (Box<Button>).
    fn create_button(&self) -> Box<Button>;

    /// Creates a window.
    ///
    /// # Returns
    /// A window trait object (Box<Window>).
    fn create_window(&self) -> Box<Window>;
}

/// A linux button.
pub struct LinuxButton;
/// A linux window
pub struct LinuxWindow;

impl Button for LinuxButton {
    fn paint(&self) -> &str {
        "LinuxButton"
    }
}

impl Window for LinuxWindow {
    fn size(&self) -> (u32, u32) {
        (400, 400)
    }
}

/// A OSX button.
pub struct OSXButton;
/// A OSX window.
pub struct OSXWindow;

impl Button for OSXButton {
    fn paint(&self) -> &str {
        "OSXButton"
    }
}

impl Window for OSXWindow {
    fn size(&self) -> (u32, u32) {
        (800, 800)
    }
}

/// A linux system.
pub struct Linux;
/// A OSX system.
pub struct OSX;

impl Factory for Linux {
    fn create_button(&self) -> Box<Button> {
        Box::new(LinuxButton {})
    }

    fn create_window(&self) -> Box<Window> {
        Box::new(LinuxWindow {})
    }
}

impl Factory for OSX {
    fn create_button(&self) -> Box<Button> {
        Box::new(OSXButton {})
    }

    fn create_window(&self) -> Box<Window> {
        Box::new(OSXWindow {})
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abstract_factory() {
        let osx_factory = OSX {};
        let osx_button = osx_factory.create_button();
        let osx_window = osx_factory.create_window();

        let linux_factory = Linux {};
        let linux_button = linux_factory.create_button();
        let linux_window = linux_factory.create_window();

        assert_eq!(osx_button.paint(), "OSXButton");
        assert_eq!(osx_window.size(), (800_u32, 800_u32));

        assert_eq!(linux_button.paint(), "LinuxButton");
        assert_eq!(linux_window.size(), (400_u32, 400_u32));
    }
}
