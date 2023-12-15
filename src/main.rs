mod parts;
use rust_gpiozero::Servo;
use std::{thread, time};
const SERVO_PIN: u8 = 14;

fn main() {
    // parts::motor::open();
    let mut servo = Servo::new(SERVO_PIN);
    servo.max();
    let ten_millis = time::Duration::from_millis(500);
    thread::sleep(ten_millis);
    servo.min();
}
