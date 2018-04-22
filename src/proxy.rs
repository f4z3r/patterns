//! Proxy design pattern.
//!
//! # Theory
//! A proxy object has the objective to be used as a surrogate or placeholder for another object to control access to
//! it. This allows lazy contruction of the actual underlying object. Such a proxy can be used to cache information
//! to reduce the calls to the actual object. It can also be used to check preconditions before operations on the real
//! object are invoked.
//!
//! # Participants
//! - `ICar`: the subject defining the common interface for the proxy (`ProxyCar`) and the real object (`Car`) such that
//!   they are exchangable.
//! - `ProxyCar`: a proxy that keeps a reference to the real `Car` and forwards requests to it when appropriate. In this
//!   case, the proxy makes an additional check to see if the driver is old enough to drive.
//! - `Car`: defines the real object that is represented by the proxy.
//!
//! # Modifications and Strategies
//! ## Proxy types:
//! - _remote proxy_: provides a local representative for an object in a different address space.
//! - _virtual proxy_: placeholder for an expensive object. This is used from the system until the client demands to
//!   create it.
//! - _protection proxy_: controls access to the original object (as in this case).
//! - _smart references_: replacements for bare pointers that perform additional actions when an object is accessed.
//!   Examples of those are smart pointers and reference counts. Moreover, copy on write references are also such
//!   smart references.
//!
//! # Attention
//! Identity operations on objects and their proxies can lead to unexpected results.
//!
//! # Known Uses
//! Smart pointers and copy on write objects.


/// An interface for a Car
trait ICar {
    fn drive(&self) -> String;
}

/// A simple car
struct Car;

impl ICar for Car {
    /// Return that the car is driving
    fn drive(&self) -> String{
        "car is driving".to_string()
    }
}

/// The proxy of the car
struct ProxyCar<'a> {
    real_car: &'a ICar,
    driver_age: i32,
}

impl<'a> ICar for ProxyCar<'a> {
    fn drive(&self) -> String {
        if self.driver_age > 18 {
            self.real_car.drive()
        } else {
            "driver is too young".to_string()
        }
    }
}

impl<'a> ProxyCar<'a> {
    /// Constructor
    fn new(driver_age: i32, other_car: &'a ICar) -> ProxyCar<'a> {
        ProxyCar {
            real_car: other_car,
            driver_age,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy() {
        let car = Car {};
        let proxy = ProxyCar::new(16, &car);
        assert_eq!(proxy.drive(), "driver is too young");

        let proxy_2 = ProxyCar::new(19, &car);
        assert_eq!(proxy_2.drive(), "car is driving");
    }
}
