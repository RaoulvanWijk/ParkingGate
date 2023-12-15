use parts::motor;
use std::{thread, time};

fn main() {
    motor::open();
    let ten_millis = time::Duration::from_millis(500);
    thread::sleep(ten_millis);
    motor::close();
}
