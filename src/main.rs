use std::thread;
use std::time::Duration;
use std::error::Error;
use rppal::gpio::Gpio;

fn main()->Result<(), Box<dyn Error>>{
let mut pin = Gpio::new()?.get(23)?.into_output();
loop{
pin.set_high();
thread::sleep(Duration::from_secs(1));
pin.set_low();
thread::sleep(Duration::from_secs(1));
}
}
