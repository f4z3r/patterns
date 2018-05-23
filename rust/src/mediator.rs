//! Mediator pattern example.
//!
//! # Theory
//! This behavioural pattern allows to define an object that encapsulates how a set of objects interact. The mediator
//! promotes loose coupling by keeping objects from referring to each other explicitly, and it lets you vary their
//! interaction independently.
//!
//! This also allows to easier reuse objects due to the fact that they do not communicate with many others, but only
//! with the mediator.
//!
//! # Participants
//! - `Mediator`: defines an interface for communicating with colleague objects.
//! - `ParticipantMediator`: a concrete mediator implementing the cooperative behaviour by coordinating colleague
//!   objects.
//! - `ButtonView`, `ButtonSearch`, `ButtonBook` and `Display`: colleague classes handled by the mediator. Note that
//!   usually each of these classes should have a reference to the mediator, but this is not the cases here as it would
//!   result in cyclic references, forbidden in Rust.
//!
//! # Note
//! Due to the fact that cyclic references are not allowed in Rust, this makes the mediator class very unelegant and
//! complex to implement. Hence the sample code below should probably not be used as a template if the behaviour of
//! the mediator pattern is required.

use std::cell::Cell;

/// The mediator interface implemented by a concrete mediator
trait Mediator<'a> {
    fn book(&self) -> &str;
    fn view(&self) -> &str;
    fn search(&self) -> &str;
    fn register_view(&mut self, view: &'a ButtonView);
    fn register_search(&mut self, search: &'a ButtonSearch);
    fn register_book(&mut self, book: &'a ButtonBook);
    fn register_display(&mut self, display: &'a Display);
    fn get_counts(&self) -> (u8, u8, u8);
}

/// A concrete mediator playing the middle agent between the book, search and view buttons and the display.
struct ParticipantMediator<'a> {
    view: Option<&'a ButtonView>,
    search: Option<&'a ButtonSearch>,
    book: Option<&'a ButtonBook>,
    display: Option<&'a Display>,
}
impl<'a> ParticipantMediator<'a> {
    fn new() -> ParticipantMediator<'a> {
        ParticipantMediator {
            view: None,
            search: None,
            book: None,
            display: None,
        }
    }
}
impl<'a> Mediator<'a> for ParticipantMediator<'a> {
    fn register_book(&mut self, book: &'a ButtonBook) {
        self.book = Some(book);
    }
    fn register_display(&mut self, display: &'a Display) {
        self.display = Some(display);
    }
    fn register_search(&mut self, search: &'a ButtonSearch) {
        self.search = Some(search);
    }
    fn register_view(&mut self, view: &'a ButtonView) {
        self.view = Some(view);
    }
    fn book(&self) -> &str {
        self.book.expect("No book button registered with mediator").press();
        self.display.expect("No display registered with mediator").print("booking")
    }
    fn view(&self) -> &str {
        self.view.expect("No view button registered with mediator").press();
        self.display.expect("No display registered with mediator").print("viewing")
    }
    fn search(&self) -> &str {
        self.search.expect("No search button registered with mediator").press();
        self.display.expect("No display registered with mediator").print("searching")
    }
    fn get_counts(&self) -> (u8, u8, u8) {
        let c_s = self.search.expect("No search button registered with mediator").get_press_count();
        let c_v = self.view.expect("No view button registered with mediator").get_press_count();
        let c_b = self.book.expect("No book button registered with mediator").get_press_count();
        (c_v, c_s, c_b)
    }
}


trait Button {
    fn press(&self);
    fn get_press_count(&self) -> u8;
}

struct ButtonBook {
    count: Cell<u8>,
}
impl ButtonBook {
    fn new() -> ButtonBook {
        ButtonBook {
            count: Cell::new(0),
        }
    }
}
impl Button for ButtonBook {
    fn press(&self) {
        let count = self.count.get();
        self.count.set(count + 1);
    }
    fn get_press_count(&self) -> u8 {
        self.count.get()
    }
}

struct ButtonView {
    count: Cell<u8>,
}
impl ButtonView {
    fn new() -> ButtonView {
        ButtonView {
            count: Cell::new(0),
        }
    }
}
impl Button for ButtonView {
    fn press(&self) {
        let count = self.count.get();
        self.count.set(count + 1);
    }
    fn get_press_count(&self) -> u8 {
        self.count.get()
    }
}

struct ButtonSearch {
    count: Cell<u8>,
}
impl ButtonSearch {
    fn new() -> ButtonSearch {
        ButtonSearch {
            count: Cell::new(0),
        }
    }
}
impl Button for ButtonSearch {
    fn press(&self) {
        let count = self.count.get();
        self.count.set(count + 1);
    }
    fn get_press_count(&self) -> u8 {
        self.count.get()
    }
}

struct Display;
impl<'a> Display {
    fn print(&self, string: &'a str) -> &'a str {
        string
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_mediator_panic() {
        let mediator = ParticipantMediator::new();
        mediator.view();
    }

    #[test]
    fn test_mediator() {
        let view = ButtonView::new();
        let search = ButtonSearch::new();
        let book = ButtonBook::new();
        let display = Display;

        let mut mediator = ParticipantMediator::new();
        mediator.register_book(&book);
        mediator.register_view(&view);
        mediator.register_search(&search);
        mediator.register_display(&display);

        assert_eq!(mediator.view(), "viewing");
        assert_eq!(mediator.book(), "booking");
        assert_eq!(mediator.search(), "searching");
        assert_eq!(mediator.view(), "viewing");

        assert_eq!(mediator.get_counts(), (2_u8, 1_u8, 1_u8));
    }
}
