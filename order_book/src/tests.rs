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

        let (ask_id, ask_trades) = ob.submit_order(&ask_order);
        let (bid_id, bid_trades) = ob.submit_order(&bid_order);

        if let Some(ref trades) = bid_trades {
            let trade = &trades[0];

            assert_eq!(trade.maker, ask_id);
            assert_eq!(trade.taker, bid_id);
            assert_eq!(trade.price.as_float(), 10.00);
            assert_eq!(trade.quantity, 100);
        }

        assert!(ask_trades.is_none());
        assert!(bid_trades.is_some());
        assert_eq!(ob.orders.len(), 0);
        assert!(ob.bids.is_empty());
        assert!(ob.asks.is_empty());
    }

    #[test]
    fn partial_fill_resting_remains() {
        let mut ob = OrderBook::new();

        let (bid_order_id, bid_trades) = ob.submit_order(&OrderReq::new(Type::Limit, Side::Bid, 10.00, 100));
        let (ask_order_id, ask_trades) = ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.00, 50));

        if let Some (ref trades) = ask_trades {
            let trade = &trades[0];

            assert_eq!(trade.maker, bid_order_id);
            assert_eq!(trade.taker, ask_order_id);

            assert_eq!(trade.quantity, 50);
            assert_eq!(trade.price.as_float(), 10.00);
        }

        let order = ob.get_order(&bid_order_id).unwrap();

        assert!(bid_trades.is_none());
        assert!(ask_trades.is_some());

        assert_eq!(ob.orders.len(), 1);
        assert_eq!(ob.asks.len(), 0);
        assert_eq!(ob.bids.len(), 1);
        assert_eq!(order.order.quantity, 50);
    }

    #[test]
    fn multi_level_fill() {
        let mut ob = OrderBook::new();

        let (first_ask_order_id, first_ask_trades) = ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.00, 50));
        let (second_ask_order_id, second_ask_trades) = ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.05, 50));
        let (bid_order_id, bid_trades) = ob.submit_order(&OrderReq::new(Type::Limit, Side::Bid, 10.10, 100));


        if let Some(ref trades) = bid_trades {
            assert_eq!(trades.len(), 2);

            assert_eq!(trades[0].maker, first_ask_order_id);
            assert_eq!(trades[1].maker, second_ask_order_id);

            assert_eq!(trades[0].taker, bid_order_id);
            assert_eq!(trades[1].taker, bid_order_id);

            assert_eq!(trades[0].quantity, 50);
            assert_eq!(trades[1].quantity, 50);

            assert_eq!(trades[0].price.as_float(), 10.00);
            assert_eq!(trades[1].price.as_float(), 10.05);
        }


        assert!(first_ask_trades.is_none());
        assert!(second_ask_trades.is_none());
        assert!(bid_trades.is_some());

        assert!(ob.orders.is_empty());
    }

    #[test]
    fn fifo_same_price() {
        let mut ob = OrderBook::new();

        let (first_ask_order_id, _) = ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.00, 50));
        let (second_ask_order_id, _) =
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

        let (order_id, _) = ob.submit_order(&OrderReq::new(Type::Limit, Side::Ask, 10.00, 100));
        let _ = ob.cancel_order(&order_id).unwrap();

        assert!(ob.asks.is_empty());
    }
}
