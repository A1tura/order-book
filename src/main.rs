mod order;
mod order_book;

use order::{Order, Side};
use order_book::OrderBook;

fn main() {
    let mut ob = OrderBook::new();
    let order = Order::new(Side::Bid, 0.8, 100);
    let order2 = Order::new(Side::Bid, 1.0, 50);
    let order3 = Order::new(Side::Ask, 1.0, 20);

    let order4 = Order::new(Side::Ask, 0.5, 800);

    ob.add_order(order);
    ob.add_order(order2);
    ob.add_order(order3);
    ob.add_order(order4);

    println!("{}", ob);

    println!("{}", ob.get_best_ask());
}
