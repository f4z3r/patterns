//! Oberver design pattern.
//!
//! # Theory
//! Defines a one-to-many dependency between objects so that when one object (the subject) changes the state, all its
//! dependents (observers) are notified and updated automatically. This ensures consistency between related objects.
//! Moreover, it avoids tight coupling, as one object does not need to know about the other, hence objects remain
//! flexible and reusable. The separation of the underlying application data and representation is usually the goal (for
//! example in the Model-View-Controller model). It follows a kind of publish-subscribe principle which is used when
//! objects depend on each other and a change to one of the objects requires changing others.
//!
//! # Participants
//! - `Observable`: the subject interface. It holds a list of observers and notifies them when its state changes. It
//!    also provides an interface for adding and removing observers. Such a subject may have any number of dependent
//!    observers.
//! - `Observer`: defines the `update` method from the interface to publish notifications. The observer gets notified
//!   about any change in data through this method.
//! - `Model`: the concrete subject being observed. It has state that is of interest to the observers.
//! - `View`: the concrete observers. They are dependent on `Model`, hence requiring the observation. They should
//!   implement the `Observer` interface to remain consistent with the current state of `View` (the subject).
//!
//! # Modifications and Strategies
//! - Observers can postpone its updates until it gets a notification from the subject.
//! - One can change the implemention such that observers might not have associated subjects. If the source of the event
//!   is known, the observer can then choose to ignore it.
//! - One can implement a many-to-many relationship between observers and subjects. If this is the case, the interface
//!   might need modification if the observer requires knowledge of which object sent it a notification.
//! - _Change managers_: such an implementation waits before triggering the notification (different update strategy).
//!   Then several changes can be fused. This is faster, but more error-prone.
//! - Observers might want to register for specific event types only.
//! - _Push model_: the update interface provides the update information. This is what is implemented below. The reason
//!   for this is that in Rust, a _pull request_ would require the Observer to (at least "temporarily") hold a reference
//!   to the subject. This would prevent the subject from being mutable. Moreover, this would not be thread safe if the
//!   subject and the observers live in different threads (e.g. the observer is a GUI in its own thread).
//! - A mediator can be used if there are many observers and subjects. It then acts as an observer for subjects and as a
//!   subject for observers.
//!
//! # Attention
//! Note that a simple operation can easily cause many unexpected updates via a cascade effect. This can be solved by
//! providing only updates during idle times. Another solution is to notify observers only if a change state actually
//! occurs in the state object. Otherwise, observers might only want to register for certain events, which reduces the
//! overall number of update calls.
//!
//! On top of that, references to deleted subjects must be updated (i.e. deleted if they only depend on the deleted
//! subject). Note that this can be solved by the subject owning is observers in a one-to-many model. However, this
//! can obviously not be done in many-to-many models.
//!
//! Note that cyclic references here can be a problem as this results in an infinite chain of udpates. This is not a
//! problem with Rust if writing safe code as such cyclic references are not allowed.
//!
//! # Known Uses
//! - ColorPicker
//! - JDRAW: draw model
//! - AWT event handling in Java
//!
//! # Note
//! Note that the way this is implemented below allows for any type of listener (i.e. as long as it implements the
//! `Observer` interface). This might not be desired in some situations where one wants to restrict the listeners to
//! have a single type. In such a case, use generics to make every observable only observable by a single observer type.
//!
//! Moreover, note that the `update` functions here return `String`. This is usually not the case but is used here as
//! a means of testing if the code works as expected.

/// The trait implemented by observers
trait Observer {
    /// The function called by the observed object. Note that usually this does not return anything. The fact that it
    /// returns a string is simply for testing purposes.
    fn update(&self, data: u64) -> String;
}

/// The trait implemented by an observable object.
trait Observable<'a> {
    /// Registers a new observer for this object
    fn register_observer(&mut self, observer: &'a Observer);
    /// Notifies all observers registered with this object
    fn notify_observers(&self, data: u64) -> String;
}

/// The observer
struct View {
    name: String,
}

impl Observer for View {
    fn update(&self, data: u64) -> String {
        format!("View {} got data: {}", self.name, data)
    }
}

/// The subject
struct Model<'a> {
    data: u64,
    observers: Vec<&'a Observer>,
}

impl<'a> Model<'a> {
    fn set_data(&mut self, data: u64) -> String {
        self.data = data;
        self.notify_observers(self.data)
    }
}

impl<'a> Observable<'a> for Model<'a> {
    fn register_observer(&mut self, observer: &'a Observer) {
        self.observers.push(observer);
    }

    fn notify_observers(&self, data: u64) -> String {
        let mut result = "".to_string();
        for observer in &self.observers {
            result = format!("{}\n{}", result, observer.update(data));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer() {
        let view_0 = View { name: "view_0".to_string() };
        let view_1 = View { name: "view_1".to_string() };
        let view_2 = View { name: "view_2".to_string() };

        let mut subject = Model { data: 0_u64, observers: Vec::new() };
        subject.register_observer(&view_0);
        subject.register_observer(&view_1);
        subject.register_observer(&view_2);

        let mut res = subject.set_data(24);
        assert_eq!(res, "\nView view_0 got data: 24\nView view_1 got data: 24\nView view_2 got data: 24");

        res = subject.set_data(100);
        assert_eq!(res, "\nView view_0 got data: 100\nView view_1 got data: 100\nView view_2 got data: 100");

        res = subject.set_data(1130113);
        assert_eq!(res, "\nView view_0 got data: 1130113\nView view_1 got data: 1130113\nView view_2 got data: 1130113");
    }
}
