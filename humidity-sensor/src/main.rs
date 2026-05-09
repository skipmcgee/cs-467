//! Reads a DHT20 Temp/Humidity sensor and displays humidity levels
//!
//! This project uses readings from a DHT20 Temperature and Humidity sensor.
//! It displays the humidity levels via an LED array and an LCD screen.
//! Adapted code from: https://rust-classes.com/chapter_embedded_pi_input_dht20
//! This project structure was generated with a template from: https://github.com/rp-rs/rp2040-project-template
#![no_std]
#![no_main]
use core::fmt::Write;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    i2c::{self, Config, InterruptHandler},
    peripherals::{I2C0, I2C1},
};
use embassy_time::Delay;
use embassy_time::Timer;
use hd44780_driver::HD44780;
use heapless::String;
use panic_probe as _;
use {defmt_rtt as _, panic_probe as _};

// local imports
mod sensor;
use sensor::{initialize, read_temperature_and_humidity};

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
    I2C0_IRQ => InterruptHandler<I2C0>;
});

#[embassy_executor::main]
//#[entry]
async fn main(_spawner: Spawner) -> ! {
    info!("Starting the humidity_sensor project program");

    // sensor setup
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());
    let sda: embassy_rp::Peri<'_, embassy_rp::peripherals::PIN_2> = p.PIN_2;
    let scl: embassy_rp::Peri<'_, embassy_rp::peripherals::PIN_3> = p.PIN_3;
    let lcd_sda: embassy_rp::Peri<'_, embassy_rp::peripherals::PIN_16> = p.PIN_16;
    let lcd_scl: embassy_rp::Peri<'_, embassy_rp::peripherals::PIN_17> = p.PIN_17;

    info!("Starting to set up sensor i2c");
    let mut i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, Config::default());

    info!("Starting to set up LCD i2c");
    let mut lcd_i2c_config = Config::default();
    lcd_i2c_config.frequency = 100_000;
    //let lcd_i2c = i2c::I2c::new_async(p.I2C0, lcd_scl, lcd_sda, Irqs, Config::default());
    let lcd_i2c = i2c::I2c::new_async(p.I2C0, lcd_scl, lcd_sda, Irqs, lcd_i2c_config);

    // initialize variable to determine how often to get the sensor readings
    let reading_interval_milliseconds: u64 = 1000;

    // check if the sensor is ready
    let ready = initialize(&mut i2c).await;
    info!("Sensor Readiness: {}", ready);

    // set up lcd screen
    let mut lcd =
        HD44780::new_i2c(lcd_i2c, 0x27, &mut Delay).expect("Failed to initialize LCD Screen");

    lcd.reset(&mut Delay).expect("LCD Screen Reset Failed!");
    lcd.clear(&mut Delay).expect("LCD Screen Clear Failed!");

    // this is the main program loop
    // consider adding exit criteria to break out when disconnected, etc?
    loop {
        // get the temp and humidity readings from the sensor
        let (temperature, humidity) = read_temperature_and_humidity(&mut i2c).await;

        // output the readings to the console
        info!("Temperature: {}C, Humidity: {}%", temperature, humidity);

        // string conversion
        let mut humidity_string = String::<16>::new();
        let _ = core::write!(&mut humidity_string, "{}C", humidity);
        let mut temperature_string = String::<16>::new();
        let _ = core::write!(&mut temperature_string, "{}C", temperature);
        debug!("Humidity String: {}", humidity_string.as_str());
        debug!("Temperature String: {}", temperature_string.as_str());

        // LCD commands
        //lcd.reset(&mut Delay).expect("LCD Screen Reset Failed!");
        Timer::after_millis(500).await;
        //lcd.clear(&mut Delay).expect("LCD Screen Clear Failed!");
        Timer::after_millis(500).await;
        match lcd.write_str(&humidity_string, &mut Delay) {
            Ok(_) => debug!("Success writing to LCD Screen"),
            Err(_) => info!("Error writing humidity value to LCD screen"),
        }

        // delay a period of time before the next reading
        Timer::after_millis(reading_interval_milliseconds).await;
    }
}
