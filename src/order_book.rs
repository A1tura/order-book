use std::{
    collections::{BTreeMap, HashMap},
    fmt::{self, Display},
};

use crate::order::{Order, Price, Side};

pub struct OrderBook {
    pub bids: BTreeMap<Price, Vec<u32>>,
    pub asks: BTreeMap<Price, Vec<u32>>,

    pub orders: HashMap<u32, Order>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),

            orders: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.insert(order.id, order.clone());
        match &order.side {
            Side::Bid => {
                if self.bids.contains_key(&order.price) {
                    self.bids.get_mut(&order.price).unwrap().push(order.id);
                } else {
                    let mut orders: Vec<u32> = Vec::new();
                    orders.push(order.id);
                    self.bids.insert(order.price.clone(), orders);
                }
            }
            Side::Ask => {
                if self.asks.contains_key(&order.price) {
                    self.asks.get_mut(&order.price).unwrap().push(order.id);
                } else {
                    let mut orders: Vec<u32> = Vec::new();
                    orders.push(order.id);
                    self.asks.insert(order.price.clone(), orders);
                }
            }
        }
    }

    pub fn get_best_bid(&self) -> u32 {
        return *self.bids.iter().next_back().unwrap().1.iter().next().unwrap();
    }

    pub fn get_best_ask(&self) -> u32 {
        return *self.asks.iter().next().unwrap().1.iter().next().unwrap();
    }
}

impl Display for OrderBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "Bids: \n");
        for (price, orders) in &self.bids {
            let _ = write!(f, "\t{}:\n", price.as_float());
            for order in orders {
                let order = self.orders.get(order).unwrap();
                let _ = write!(f, "\t\tid: {}; quantity: {}\n", order.id, order.quantity);
            }
        }

        let _ = write!(f, "Asks: \n");
        for (price, orders) in &self.asks {
            let _ = write!(f, "\t{}:\n", price.as_float());
            for order in orders {
                let order = self.orders.get(order).unwrap();
                let _ = write!(f, "\t\tid: {}; quantity: {}\n", order.id, order.quantity);
            }
        }

        return Ok(());
    }
}
