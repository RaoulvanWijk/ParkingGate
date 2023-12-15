use rust_gpiozero::Servo;

const SERVO_PIN: u8 = 14;

pub fn open() {
    let mut servo = Servo::new(SERVO_PIN);
    servo.max();
}

pub fn close() {
    let mut servo = Servo::new(SERVO_PIN);
    servo.min();
}
