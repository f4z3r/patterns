//! State design pattern.
//!
//! # Theory
//! Behavioural software design pattern that implements a state machine in an object-oriented way. With this pattern,
//! a state machine is implemented by implementing each individual state as a derived class of the state pattern
//! interface, and implementing state transitions by invoking methods defined by the pattern's superclass.
//!
//! It allows an object to alter its behaviour when its internal state changes. The object will appear to change its
//! class. This helps avoiding conditional structure in the code where each branch of the conditional structure forms
//! a separate class. State specific behaviour is then localised and new states can be added easily. This prevents
//! inconsistent internal states and state transitions are made explicit.
//!
//! # Participants
//! - `Post`: the context defining a common interface to be used by the clients. This holds an instance of a concrete
//!   state that represents the current state. It delegates state-specific requests to the current concrete state.
//! - `State`: defines the interface for encapsulating the state depedent behaviour.
//! - `Draft`, `PendingReview` and `Published`: concrete states representing different operational states. These
//!   implement behaviour associated with a specific state of the context `Post`. These may access the content of the
//!   context. In general, these can modify the context. However, in Rust, this is not possible (if writing safe code)
//!   as it creates a cycle in the references between the state and the context.
//!
//! # Modifications and Strategies
//! State transitions can be handled either by the context or by the concrete states. Handling in the context makes
//! implementations much easier in Rust. However, in such a centralised case, the states need to be notified when they
//! are activated and deactivated. Otherwise, another solution is to have the states request a state change and have the
//! context execute that state change. Some key can be used to represent the new state that should be adopted (some sort
//! of enumeration).
//!
//! Creating state objects ahead of time and never destroying thme saves time if the state transitions occur rapidly,
//! however the context must hold references to all of them.
//!
//! # Known Uses
//! - "tools" in drawing programs
//! - JDRAW: Handles, SnapToGrid
//! - ticket machines


/// The state interface
trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

/// A concrete state.
struct Draft;
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview)
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

/// A concrete state.
struct PendingReview;
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published)
    }
}

/// A concrete state.
struct Published;
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}


/// The state machine
struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft)),
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}


