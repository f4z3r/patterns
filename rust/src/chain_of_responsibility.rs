//! Chain of Responsibility pattern.
//!
//! # Theory
//! Behavioural design pattern consisting of command objects and a series of processing objects. This avoids coupling
//! the sender of a request to its receiver by giving more than one object a chance to handle the request. Chain the
//! receiving objects and pass the request along the chain until an object handles it.
//!
//! Each processing object contains logic that defines the types of command objects that it can handle. The rest are
//! passed to the next processing object in the chain. In some sense, it is an object oriented version of a repeated
//! `if` idiom, with the benefit that the condition-action blocks can be dynamically rearranged and reconfigured at
//! runtime.
//!
//! # Participants
//! - `PurchasePower`: the handler interface for handling requests. It may implement the successor link if it is
//!   implemented using abstract classes.
//! - `Employee`: a concrete handler handling requests it is responsible for. It can access its successor and forward
//!   requests to it.
//! - `Client`: the `tests` module. Initiates the request to a concrete handler object in the chain.
//!
//! # Modifications and Strategies
//! Some processing objects can act as dispatchers, capable of sending commands out in a variety of directions, forming
//! a _tree of responsibilities_. This is the case in typical logging architectures.
//!
//! This is structurally nearly identical to the decorator pattern, the difference being that for decorators, all
//! classes handle the request, while for the chain or responsibility, exactly one of the classes in the chain
//! handles the request.
//!
//! Moreover, note that processing objects can have different types as long as they implement the correct interface. In
//! the example below, a single type is used as the objects are extremely similar. On top of that, note that here
//! concrete handlers own their successor. This is mainly done to enforce safe use. However, it is usually implemented
//! in a way that handlers only hold a reference to their successors.


/// Trait to be implemented by all processing objects (handlers)
trait PurchasePower {
    fn set_successor(&mut self, successor: Box<PurchasePower>);
    fn get_successor(&self) -> Option<&Box<PurchasePower>>;
    fn process_request(&self, request: PurchaseRequest) -> String {
        if request.get_amount() < self.get_allowable() {
            format!("{} will approve ${} for {}", self.get_role(), request.get_amount(), request.get_purpose())
        } else {
            match self.get_successor() {
                Some(successor) => successor.process_request(request),
                None            => String::from("Request amount is too high"),
            }
        }
    }
    fn get_allowable(&self) -> u32;
    fn get_role(&self) -> &str;

}


/// A concrete processing object (handler)
struct Employee<'a> {
    allowable: u32,
    successor: Option<Box<PurchasePower>>,
    category: &'a str,
}
impl<'a> Employee<'a> {
    fn new(category: &'a str, allowable: u32) -> Employee {
        Employee {
            allowable,
            successor: None,
            category,
        }
    }
}
impl<'a> PurchasePower for Employee<'a> {
    fn set_successor(&mut self, successor: Box<PurchasePower>) {
        self.successor = Some(successor);
    }

    fn get_successor(&self) -> Option<&Box<PurchasePower>> {
        self.successor.as_ref()
    }

    fn get_allowable(&self) -> u32 {
        self.allowable
    }

    fn get_role(&self) -> &str {
        self.category
    }
}

/// A request/command to be sent to processing objects
struct PurchaseRequest {
    amount: u32,
    purpose: String,
}
impl PurchaseRequest {
    fn new<S>(amount: u32, purpose: S) -> PurchaseRequest where S: Into<String> {
        PurchaseRequest {
            amount,
            purpose: purpose.into(),
        }
    }

    fn get_amount(&self) -> u32 {
        self.amount
    }

    fn get_purpose(&self) -> &str {
        self.purpose.as_ref()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_of_responsibility() {
        let mut manager = Employee::new("manager", 5000);
        let mut director = Employee::new("director", 10_000);
        let mut vp = Employee::new("vice-president", 20_000);
        let president = Employee::new("president", 40_000);

        vp.set_successor(Box::new(president));
        director.set_successor(Box::new(vp));
        manager.set_successor(Box::new(director));

        let request_0 = PurchaseRequest::new(12_000, "general expenses");
        assert_eq!(manager.process_request(request_0), "vice-president will approve $12000 for general expenses");

        let request_1 = PurchaseRequest::new(1_000_000, "buy a house");
        assert_eq!(manager.process_request(request_1), "Request amount is too high");

        let request_2 = PurchaseRequest::new(500, "desk repair");
        assert_eq!(manager.process_request(request_2), "manager will approve $500 for desk repair");

        let request_3 = PurchaseRequest::new(35_000, "company car");
        assert_eq!(manager.process_request(request_3), "president will approve $35000 for company car");

        let request_4 = PurchaseRequest::new(9000, "retreat event");
        assert_eq!(manager.process_request(request_4), "director will approve $9000 for retreat event");
    }
}
