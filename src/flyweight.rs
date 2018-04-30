//! Flyweight design pattern.
//!
//! # Theory
//! Structural design pattern that aims at reducing memory usage by sharing as much data as possible with other similar
//! objects. For instance, in text editors, characters are stored as glyph containing information about their font
//! outline, font metrics, other formatting data and the position of the character in the document. However, only the
//! position changes from one character to another. Hence all the other data can be shared across all characters to
//! reduce memory usage. The flyweight is the shared object that can be used in multiple contexts simultaneously. Note
//! that this is inheritently not thread safe without locking or some other synchronisation method. The flyweight object
//! then stored the intrinsic state of all its contexts (the state information that is independent of the context) and
//! the contexts in which the flyweight is used store the extrinsic state (context dependent).
//!
//! # Participants
//! - `CheeseBrand`: the flyweight objects. Note that one can usually also define a flyweight interface that all concrete
//!   flyweight object have to implement. This allows to build a variety of objects having a variety of intrinsic
//!   properties implemented by the flyweight. However, in order to simplify this sample code, no interface is given.
//! - `Menu`: the flyweight factory. It creates flyweight objects (`CheeseBrand`s) and manages them. Moreover, it
//!   ensures flyweights are shared properly. Note that this implementation is not typical of flyweights as the shared
//!   intrinsic state of the `CheeseShop`s is mutable. Hence, the factory does not only implement information retrieval
//!   methods. This would be the case in most scenarios, where several factories are defined based on the type of
//!   flyweights required by the client code. The factory then builds flyweights as information is requested but never
//!   based on client information directly.
//! - `CheeseShop`: the client that maintains references to the set of flyweihts via the factory. Again, this is not
//!   necessarily typical.
//!
//! # Modifications and Strategies
//! One can additionally add an interface for the flyweight class such that the factory can create a variety of
//! different flyweight objects (from different classes). Note that it is important that the client cannot build its
//! own flyweight objects, or no shared state across clients will be possible.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use std::result;

type Result<T> = result::Result<T, OutOfStockError>;

#[derive(Debug, PartialEq, Eq)]
struct OutOfStockError;
impl fmt::Display for OutOfStockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of stock")
    }
}
impl Error for OutOfStockError {
    fn description(&self) -> &str {
        "Out of stock"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

/// The flyweight factory, it is shared across all `CheeseShop`s. `CheeseShop`s can use its interface to add specific
/// flyweight objects (`CheeseBrand`) to the menu. Note that this is not thread-safe as it uses a `RefCell`. In a
/// multithreaded scenario, the use of mutexes would be required.
struct Menu {
    items: RefCell<HashMap<String, CheeseBrand>>,
}
impl Menu {
    fn new() -> Menu {
        Menu {
            items: RefCell::new(HashMap::new()),
        }
    }

    fn add(&self, name: &str, cost: f32, quantity: f32) {
        let cheese = CheeseBrand::new(name, cost, quantity);
        let mut items = self.items.borrow_mut();
        let entry = items.entry(String::from(name)).or_insert(cheese);
        entry.cost = cost;
        entry.quantity = quantity;
    }

    fn sell(&self, name: &str, quantity: f32) -> Result<f32> {
        let mut items = self.items.borrow_mut();
        match items.get_mut(name) {
            Some(ch)    => {
                ch.reduce_quantity(quantity)?;
                Ok(ch.cost)
            },
            None        => Err(OutOfStockError),
        }
    }
}

/// The flyweight objects. All cheesebrands (name, quantity, and cost) are shared across all `CheeseShop`s.
struct CheeseBrand {
    name: String,
    cost: f32,
    quantity: f32,
}
impl CheeseBrand {
    fn new(name: &str, cost: f32, quantity: f32) -> CheeseBrand {
        CheeseBrand {
            name: String::from(name),
            cost,
            quantity,
        }
    }

    fn reduce_quantity(&mut self, quantity: f32) -> Result<()> {
        if quantity > self.quantity {
            return Err(OutOfStockError);
        }
        self.quantity -= quantity;
        Ok(())
    }
}
impl PartialEq for CheeseBrand {
    fn eq(&self, other: &CheeseBrand) -> bool {
        self.name == other.name
    }
}
impl Eq for CheeseBrand {}


/// The class sharing the flyweight. This defines individual cheese shops that have their own extrinsic state (the
/// number of cheese units sold and the total revenue made in this particular shop). However, it also shares a global
/// inventory across all shops representing the intrinsic state of the shop.
struct CheeseShop<'a> {
    menu: &'a Menu,
    units_sold: f32,
    revenue: f32,
}
impl<'a> CheeseShop<'a> {
    fn new(menu: &'a Menu) -> CheeseShop {
        CheeseShop {
            menu,
            units_sold: 0_f32,
            revenue: 0_f32,
        }
    }

    fn stock_cheese(&self, name: &str, cost: f32, quantity: f32) {
        self.menu.add(name, cost, quantity);
    }

    fn sell(&mut self, name: &str, quantity: f32) -> Result<()> {
        let cost = self.menu.sell(name, quantity)?;
        self.units_sold += quantity;
        self.revenue += cost * quantity;
        Ok(())
    }

    fn total_units_sold(&self) -> f32 {
        self.units_sold
    }

    fn total_revenue(&self) -> f32 {
        self.revenue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flyweight() {
        let menu = Menu::new();
        let mut shop1 = CheeseShop::new(&menu);
        let mut shop2 = CheeseShop::new(&menu);

        // No cheese added to the menu yet.
        assert_eq!(shop1.sell("blue", 10_f32), Err(OutOfStockError));

        shop1.stock_cheese("blue", 2.5, 10_f32);
        assert_eq!(shop2.sell("blue", 5_f32), Ok(()));

        shop2.stock_cheese("white", 1.25, 20_f32);
        assert_eq!(shop2.sell("white", 10_f32), Ok(()));

        assert_eq!(shop2.total_units_sold(), 15_f32);
        assert_eq!(shop2.total_revenue(), 25_f32);

        // Only 5 left of type "blue" because shop 2 sold 5 units.
        assert_eq!(shop1.sell("blue", 10_f32), Err(OutOfStockError));

        // After this, no white cheese left in menu
        assert_eq!(shop1.sell("white", 10_f32), Ok(()));

        assert_eq!(shop1.sell("white", 1_f32), Err(OutOfStockError));

        assert_eq!(shop1.total_units_sold(), 10_f32);
        assert_eq!(shop1.total_revenue(), 12.5_f32);

    }
}
