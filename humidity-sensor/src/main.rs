//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod sensor;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    i2c::{self, Config, InterruptHandler},
    peripherals::I2C1,
};
use embassy_time::Timer;
use defmt::*;
use defmt_rtt as _;
//use embedded_hal::digital::OutputPin;
use panic_probe as _;
use sensor::{initialize, read_temperature_and_humidity};
use {defmt_rtt as _, panic_probe as _};
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
    info!("Program start");
    //let mut pac = pac::Peripherals::take().unwrap();
    //let core = pac::CorePeripherals::take().unwrap();
    //let mut watchdog = Watchdog::new(pac.WATCHDOG);
    //let sio = Sio::new(pac.SIO);
    
    // sensor setup
    let p = embassy_rp::init(Default::default());
    let sda = p.PIN_2;
    let scl = p.PIN_3;
    info!("set up i2c ");
    let mut i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, Config::default());
    let ready = initialize(&mut i2c).await;
    info!("Ready: {}", ready);

    // External high-speed crystal on the pico board is 12Mhz
    //let external_xtal_freq_hz = 12_000_000u32;
    //let clocks = init_clocks_and_plls(
    //    external_xtal_freq_hz,
    //    pac.XOSC,
    //    pac.CLOCKS,
    //    pac.PLL_SYS,
    //    pac.PLL_USB,
    //    &mut pac.RESETS,
    //    &mut watchdog,
    //)
    //.ok()
    //.unwrap();

    //let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    //let pins = bsp::Pins::new(
    //    pac.IO_BANK0,
    //    pac.PADS_BANK0,
    //    sio.gpio_bank0,
    //    &mut pac.RESETS,
    //);

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    //
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead.
    // One way to do that is by using [embassy](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/wifi_blinky.rs)
    //
    // If you have a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here. Don't forget adding an appropriate resistor
    // in series with the LED.
    //let mut led_pin = pins.led.into_push_pull_output();

    loop {
        //info!("on!");
        //led_pin.set_high().unwrap();
        //delay.delay_ms(500);
        //info!("off!");
        //led_pin.set_low().unwrap();
        let (temperature, humidity) = read_temperature_and_humidity(&mut i2c).await;
        info!("temperature = {}C", temperature);
        info!("humidity = {}%", humidity);
        Timer::after_millis(500).await;
        //delay.delay_ms(500);
    }
}

// End of file
