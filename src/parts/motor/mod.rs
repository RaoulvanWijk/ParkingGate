use rust_gpiozero::Servo;

const SERVO_PIN: u8 = 14;

pub mod motor {
  pub fn open() {
    let servo = Servo::new(SERVO_PIN);
    servo.max();
  }

  pub fn close() {
    let servo = Servo::new(SERVO_PIN);
    servo.min();
  }
}