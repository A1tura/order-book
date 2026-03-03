#[cfg(test)]
mod tests {
    use crate::{
        order::{OrderReq, Side, Type},
        order_book::OrderBook,
    };

    #[test]
    fn basic_match() {
        let mut ob = OrderBook::new();

        let bid_order = OrderReq::new(Type::Limit, Side::Bid, 10.00, 100);
        let ask_order = OrderReq::new(Type::Limit, Side::Ask, 10.00, 100);


        let _ = ob.submit_order(&ask_order);
        let _ = ob.submit_order(&bid_order);


        assert_eq!(ob.orders.len(), 0);
        assert!(ob.bids.is_empty());
        assert!(ob.asks.is_empty());
    }
}
