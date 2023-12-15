use parts;
use std::{thread, time};

fn main() {
    parts::motor::open();
    let ten_millis = time::Duration::from_millis(500);
    thread::sleep(ten_millis);
    parts::motor::close();
}
