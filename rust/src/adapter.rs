//! Adapter design pattern.
//!
//! # Theory
//! Structural design pattern that allows the interface of an existing class to be used as another interface. It is
//! often used to make existing classes work with others without modifying their source code. The idea is to use a mixin
//! to extend the usage of an existing class or use aggregation to forward the request to an existing object already
//! implementing the desired functionality.
//!
//! Note that in rust, one can also use composition to adapt the interface. This makes the actions performed on the
//! adaptee more flexible and easier to implement. However, this is not always possible as the adaptee might be required
//! to perform some other work. The implementation below uses composition instead of aggregation. This can be thought of
//! as the `USBCharger` owning the `IPhone` it is currently charging using a USB-C to iPhone charger.
//!
//! # Participants
//! - `TargetInterface`: the target that defines the domain-specific interface that `Client` uses.
//! - `Client`: the client that requires the objects it handles to implement an interface `TargetInterface`.
//! - `IPhone`: defines the existing interface that needs adapting (adaptee).
//! - `USBCharger`: adapts the interface of the adaptee (`IPhone`) to the target interface `TargetInterface` (adapter).
//!
//! # Modifications and Strategies
//! In languages supporting mixins, multiple subclassing, or private inheritance (code inheritance without subclassing)
//! such as C++, this can trivially be implemented by inheriting code from the adaptee and subclassing the abstract
//! class implementing the target interface.
//!
//! In other cases, one must consider the extra work the adapter is performing in order to adapt the interface. This
//! mostly depends on the similarity of the interfaces. In the example below, it is simply a case of different method
//! names. However, it could be the case that conversions, object creations, etc. are required in order to provide
//! correct arguments to the adaptees methods. This can lead to a performance drop compared to the initial interface.
//!
//! _Two-way adapters_ also provide the adaptees interface. This provides more transparency and allows the adapter
//! object to also be used where adaptee is used.
//!
//! # Known Uses
//! - InterViews uses it for some graphics objects.


/// The target interface to implement for an object.
trait TargetInterface {
    fn recharge(&self) -> String;
}

/// Client requiring the target interface to be used.
struct Client;
impl Client {
    fn do_stuff_with_usb<T>(obj: &T) -> String where T: TargetInterface {
        obj.recharge()
    }
}

/// Object implementing the functionality of the target interface but not adhering to the interface.
struct IPhone;
impl IPhone {
    fn charge(&self) -> &str {
        "Is charging"
    }
}

/// Requires the charging functionality of `Phone` but with an interface of `TargetInterface`
struct USBCharger {
    phone: IPhone,       // Uses aggregation strategy.
}
impl USBCharger {
    fn new() -> USBCharger {
        USBCharger {
            phone: IPhone,
        }
    }
}

impl TargetInterface for USBCharger {
    fn recharge(&self) -> String {
        format!("{} using a USB-C adapter", self.phone.charge())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter() {
        let usb_charger = USBCharger::new();
        assert_eq!(Client::do_stuff_with_usb(&usb_charger), "Is charging using a USB-C adapter");
    }
}
