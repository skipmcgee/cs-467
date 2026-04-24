//! Reads a DHT20 Temp/Humidity sensor and displays humidity levels
//!
//! This project uses readings from a DHT20 Temperature and Humidity sensor.
//! It displays the humidity levels via an LED array and an LCD screen.
//! Adapted code from: https://rust-classes.com/chapter_embedded_pi_input_dht20
#![no_std]
#![no_main]
mod sensor;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    i2c::{self, Config, InterruptHandler},
    peripherals::I2C1,
};
use embassy_time::Timer;
use panic_probe as _;
use sensor::{initialize, read_temperature_and_humidity};
use {defmt_rtt as _, panic_probe as _};
//use embedded_hal::digital::OutputPin;
// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
//use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;
//use bsp::hal::{
//    clocks::{init_clocks_and_plls, Clock},
//    pac,
//    sio::Sio,
//    watchdog::Watchdog,
//};

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});

#[embassy_executor::main]
//#[entry]
async fn main(_spawner: Spawner) -> ! {
    info!("Starting the humidity_sensor project program");
    // commented out the items from the starting blinky light test
    /*let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    */

    // sensor setup
    let p = embassy_rp::init(Default::default());
    let sda = p.PIN_2;
    let scl = p.PIN_3;
    info!("Starting to set up i2c");
    let mut i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, Config::default());

    // initialize variable to determine how often to get the sensor readings
    let reading_interval_milliseconds = 500;

    // check if the sensor is ready
    let ready = initialize(&mut i2c).await;
    info!("Sensor Readiness: {}", ready);

    // commented out the items from the blinky light setup template
    /*
    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    //
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead.
    // One way to do that is by using [embassy](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/wifi_blinky.rs)
    //
    // If you have a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here. Don't forget adding an appropriate resistor
    // in series with the LED.
    let mut led_pin = pins.led.into_push_pull_output();
    */

    // this is the main program loop
    // consider adding exit criteria to break out when disconnected, etc?
    loop {
        //commented out the example program
        /*
        //info!("on!");
        //led_pin.set_high().unwrap();
        //delay.delay_ms(500);
        //info!("off!");
        //led_pin.set_low().unwrap();
         */

        // get the temp and humidity readings from the sensor
        let (temperature, humidity) = read_temperature_and_humidity(&mut i2c).await;

        // output the readings to the console
        info!("Temperature: {}C, Humidity: {}%", temperature, humidity);

        // delay a period of time before the next reading
        Timer::after_millis(reading_interval_milliseconds).await;
    }
}
