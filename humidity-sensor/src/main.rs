//! Reads a DHT20 Temp/Humidity sensor and displays humidity levels
//!
//! This project uses readings from a DHT20 Temperature and Humidity sensor.
//! It displays the humidity levels via an LED array and an LCD screen.
//! Adapted code from: https://rust-classes.com/chapter_embedded_pi_input_dht20
//! Adapted code from https://pico.implrust.com/lcd-display/hello-rust.html
//! This project structure was generated with a template from: https://github.com/rp-rs/rp2040-project-template

// Disable standard library and fn main() since we're working with an embedded system
#![no_std]
#![no_main]
mod sensor;
mod led_strip;

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Level, Output},
    i2c::{self, Config, InterruptHandler},
    peripherals::{I2C0, I2C1},
};
use embassy_time::Delay;
use embassy_time::Timer;
use hd44780_driver::HD44780;
use heapless::String;
use panic_probe as _;
use sensor::{initialize, read_temperature_and_humidity};
use leds_strip::update_leds;

use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler as PioInterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use led_strip::update_strip;

    
use {defmt_rtt as _, panic_probe as _};

// Local imports
mod sensor;
use sensor::{initialize, read_temperature_and_humidity};

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
    I2C0_IRQ => InterruptHandler<I2C0>;
});

// Indicates entry point into the program since fn main() is disabled
#[embassy_executor::main]
//#[entry]
async fn main(_spawner: Spawner) -> ! {
    info!("Starting the humidity_sensor project program");

    // Sensor Setup
    // Sensor SDA: Connected to GP Pin 2
    // Sensor SCL: Connected to GP Pin 3
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());
    let sda: embassy_rp::Peri<'_, embassy_rp::peripherals::PIN_2> = p.PIN_2;
    let scl: embassy_rp::Peri<'_, embassy_rp::peripherals::PIN_3> = p.PIN_3;
    // LCD Display Setup
    // LCD SDA: Connected to GP Pin 16
    // LCD SCL: Connected to GP Pin 17
    let lcd_sda: embassy_rp::Peri<'_, embassy_rp::peripherals::PIN_16> = p.PIN_16;
    let lcd_scl: embassy_rp::Peri<'_, embassy_rp::peripherals::PIN_17> = p.PIN_17;

    info!("Starting to set up sensor i2c");
    let mut i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, Config::default());

    // led set up - will initialize to off until the first reading is received.
    let mut led1 = Output::new(p.PIN_10, Level::Low);
    let mut led2 = Output::new(p.PIN_11, Level::Low);
    let mut led3 = Output::new(p.PIN_12, Level::Low);
    let mut led4 = Output::new(p.PIN_13, Level::Low);
    let mut led5 = Output::new(p.PIN_14, Level::Low);
    let mut led6 = Output::new(p.PIN_15, Level::Low);

    // led STRIP set up
    let Pio {mut common, sm0, ..} = Pio::new(p.PIO0, Irqs);
    let program = PioWs2812Program::new(&mut common);
    let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, Irqs, p.PIN_16, &program);

    
    info!("Starting to set up LCD i2c");
    let mut lcd_i2c_config = Config::default();
    lcd_i2c_config.frequency = 100_000;
    let lcd_i2c = i2c::I2c::new_async(p.I2C0, lcd_scl, lcd_sda, Irqs, lcd_i2c_config);

    // initialize variable to determine how often to get the sensor readings
    let reading_interval_milliseconds: u64 = 5000;

    // check if the sensor is ready
    let ready = initialize(&mut i2c).await;
    info!("Sensor Readiness: {}", ready);

    // set up LCD screen
    let mut lcd =
        HD44780::new_i2c(lcd_i2c, 0x27, &mut Delay).expect("Failed to initialize LCD Screen");
    lcd.reset(&mut Delay).expect("LCD Screen Reset Failed!");
    lcd.clear(&mut Delay).expect("LCD Screen Clear Failed!");

    // this is the main program loop
    // consider adding exit criteria to break out when disconnected, etc?
    /*
    loop {
        // get the temp and humidity readings from the sensor
        let (temperature, humidity) = read_temperature_and_humidity(&mut i2c).await;

        // output the readings to the console
        info!("Temperature: {}C, Humidity: {}%", temperature, humidity);

        update_leds(humidity, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);

        // updates the WS2812 strip based on the reading
        update_strip(&mut ws2812, humidity).await;
        
        // delay a period of time before the next reading
        // f32 to string conversions
        let mut humidity_string = String::<16>::new();
        let mut temperature_string = String::<16>::new();
        let _ = core::write!(&mut humidity_string, "Humid.: {:.2}%", humidity);
        let _ = core::write!(&mut temperature_string, "Temp.: {:.2}C", temperature);
        debug!("Humidity String: {}", humidity_string.as_str());
        debug!("Temperature String: {}", temperature_string.as_str());

        // LCD commands
        // First clear info from screen
        lcd.clear(&mut Delay).expect("LCD Screen Clear Failed!");
        Timer::after_millis(1000).await;
        // Write humidity reading to screen
        match lcd.write_str(&humidity_string, &mut Delay) {
            Ok(_) => debug!("Success writing humidity string to LCD Screen"),
            Err(_) => info!("Error writing humidity string to LCD screen"),
        }
        // Move Cursor position to Second Line to write the temperature value
        lcd.set_cursor_pos(40, &mut Delay).expect("Failed to set cursor position");

        // Write temperature reading to screen
        match lcd.write_str(&temperature_string, &mut Delay) {
            Ok(_) => debug!("Success writing temperature string to LCD Screen"),
            Err(_) => info!("Error writing temperature string to LCD screen"),
        }

        // delay a period of time before the next reading / loop
        Timer::after_millis(reading_interval_milliseconds).await;
    }
    */

    /*
    // Integrated testing for each individual LED (as part of the strip), it will cycle through and every 3 seconds should light up the next one starting with no LEDs lit.
    // Each LED represents 10% so it goes up by 10 on each threshold.
    loop {
        info!("Testing 0.0% - Should show 0 LEDs");
        update_strip(&mut ws2812, 0.0).await;
        Timer::after_millis(3000).await;
    
        info!("Testing 10.0% - Should show 1 Green LED");
        update_strip(&mut ws2812, 10.0).await;
        Timer::after_millis(3000).await;
    
        info!("Testing 20.0% - Should show 2 Green LEDs");
        update_strip(&mut ws2812, 20.0).await;
        Timer::after_millis(3000).await;
    
        info!("Testing 30.0% - Should show 2 Green and 1 Yellow LEDs");
        update_strip(&mut ws2812, 30.0).await;
        Timer::after_millis(3000).await;
    
        info!("Testing 40.0% - Should show 2 Green and 2 Yellow LEDs");
        update_strip(&mut ws2812, 40.0).await;
        Timer::after_millis(3000).await;
    
        info!("Testing 50.0% - Should show 2 Green, 2 Yellow, and 1 OJ LEDs");
        update_strip(&mut ws2812, 50.0).await;
        Timer::after_millis(3000).await;
    
        info!("Testing 60.0% - Should show 2 Green, 2 Yellow, and 2 OJ LEDs");
        update_strip(&mut ws2812, 60.0).await;
        Timer::after_millis(3000).await;
    
        info!("Testing 70.0% - Should show 2 Green, 2 Yellow, 2 OJ , and 1 Red LEDs");
        update_strip(&mut ws2812, 70.0).await;
        Timer::after_millis(3000).await;
    
        info!("Testing 80.0% - Should show 2 Green, 2 Yellow, 2 OJ , and 2 Red LEDs");
        update_strip(&mut ws2812, 80.0).await;
        Timer::after_millis(3000).await;
    
        info!("Testing 100.0% - Should show ALL LEDs");
        update_strip(&mut ws2812, 100.0).await;
        Timer::after_millis(3000).await;    
    }    
    */
    
    // Integrated testing for each individual LED (as part of the 6 individual LED set up, it will cycle through and every 3 seconds should light up the next one starting with no LEDs lit.
    // Each LED represents 10% so it goes up by 10 on each threshold.    
    loop {
        info!("Testing 0.0% - Should show 0 LEDs");
        update_leds(0.0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);
        Timer::after_millis(3000).await;
    
        info!("Testing 19.9% - Should still show 0 LEDs");
        update_leds(19.9, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);
        Timer::after_millis(3000).await;
    
        info!("Testing 20.0% - Should show 1 LED");
        update_leds(20.0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);
        Timer::after_millis(3000).await;
    
        info!("Testing 30.0% - Should show 2 LEDs");
        update_leds(30.0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);
        Timer::after_millis(3000).await;
    
        info!("Testing 40.0% - Should show 3 LEDs");
        update_leds(40.0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);
        Timer::after_millis(3000).await;
    
        info!("Testing 50.0% - Should show 4 LEDs");
        update_leds(50.0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);
        Timer::after_millis(3000).await;
    
        info!("Testing 60.0% - Should show 5 LEDs");
        update_leds(60.0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);
        Timer::after_millis(3000).await;
    
        info!("Testing 69.9% - Should still show 5 LEDs");
        update_leds(69.9, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);
        Timer::after_millis(3000).await;
    
        info!("Testing 70.0% - Should show all 6 LEDs");
        update_leds(70.0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);
        Timer::after_millis(3000).await;
    
        info!("Testing 100.0% - Should show all 6 LEDs");
        update_leds(100.0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);
        Timer::after_millis(3000).await;
    }
}
