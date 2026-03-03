#[cfg(test)]
mod tests {
    use crate::{
        order::{OrderReq, Side, Type},
        order_book::OrderBook,
    };

    #[test]
    fn limit_orders_full_match() {
        let mut ob = OrderBook::new();

        let bid_order = OrderReq::new(Type::Limit, Side::Bid, 10.00, 100);
        let ask_order = OrderReq::new(Type::Limit, Side::Ask, 10.00, 100);

        let _ = ob.submit_order(&ask_order);
        let _ = ob.submit_order(&bid_order);

        assert_eq!(ob.orders.len(), 0);
        assert!(ob.bids.is_empty());
        assert!(ob.asks.is_empty());
    }

    #[test]
    fn partial_fill_resting_remains() {
        let mut ob = OrderBook::new();

        let bid_order_id = ob.submit_order(&OrderReq::new(Type::Limit, Side::Bid, 10.00, 100));
        let _ = ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.00, 50));

        let order = ob.get_order(&bid_order_id).unwrap();

        assert_eq!(ob.orders.len(), 1);
        assert_eq!(ob.asks.len(), 0);
        assert_eq!(ob.bids.len(), 1);
        assert_eq!(order.order.quantity, 50);
    }

    #[test]
    fn multi_level_fill() {
        let mut ob = OrderBook::new();

        ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.00, 50));
        ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.05, 50));
        ob.submit_order(&OrderReq::new(Type::Limit, Side::Bid, 10.10, 100));

        assert!(ob.orders.is_empty());
    }

    #[test]
    fn fifo_same_price() {
        let mut ob = OrderBook::new();

        let first_ask_order_id = ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.00, 50));
        let second_ask_order_id =
            ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.00, 50));
        ob.submit_order(&OrderReq::new(Type::Limit, Side::Bid, 10.00, 70));

        let order = ob.get_order(&second_ask_order_id).unwrap();

        assert!(!ob.orders.contains_key(&first_ask_order_id));
        assert_eq!(order.order.quantity, 30);
    }

    #[test]
    fn market_order_insufficient_liquidity() {
        let mut ob = OrderBook::new();

        let _ = ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.00, 10));
        let _ = ob.submit_order(&OrderReq::new(Type::Market, Side::Bid, 0.00, 1000));

        assert!(ob.asks.is_empty());
    }

    #[test]
    fn cancel_removes_order() {
        let mut ob = OrderBook::new();

        let order_id = ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.00, 100));
        ob.cancel_order(&order_id).unwrap();

        assert!(ob.asks.is_empty());
    }
}
