mod order;
mod order_book;

use order::{Order, Side, Type};
use order_book::OrderBook;
use rand::random_range;

fn main() {
    let mut ob = OrderBook::new();

    let mut big_order = Order::new(Type::Limit, Side::Bid, 100.00, 100);

    let mut order = Order::new(Type::Limit, Side::Ask, 95.00, 10);
    let mut market_order = Order::new(Type::Market, Side::Ask, 0.00, 100);

    ob.submit_order(&mut big_order);
    ob.submit_order(&mut order);
    ob.submit_order(&mut market_order);

    println!("{}", ob);
}
