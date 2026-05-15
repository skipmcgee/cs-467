//! Reads a DHT20 Temp/Humidity sensor and displays humidity levels
//!
//! This project uses readings from a DHT20 Temperature and Humidity sensor.
//! It displays the humidity levels via an LED array and an LCD screen.
//! Adapted code from: https://rust-classes.com/chapter_embedded_pi_input_dht20
//! This project structure was generated with a template from: https://github.com/rp-rs/rp2040-project-template
#![no_std]
#![no_main]
mod sensor;
mod leds;
mod led_strip;

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Level, Output},
    i2c::{self, Config, InterruptHandler},
    peripherals::I2C1,
};
use embassy_time::Timer;
use panic_probe as _;
use sensor::{initialize, read_temperature_and_humidity};
use leds::update_leds;

use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler as PioInterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program};
use led_strip::update_strip;

    
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
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
    DMA_IRQ_0 => embassy_rp::dma::InterruptHandler<DMA_CH0>;
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
    /*
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

        update_leds(humidity, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6);

        // updates the WS2812 strip based on the reading
        update_strip(&mut ws2812, humidity).await;
        
        // delay a period of time before the next reading
        Timer::after_millis(reading_interval_milliseconds).await;
    }
    */

    // Integrated testing for each individual LED, it will cycle through and every 3 seconds should light up the next one starting with no LEDs lit.
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
}
