// Code adapted from https://pico.implrust.com/lcd-display/hello-rust.html and 
// https://github.com/ImplFerris/pico2-template.git

#![no_std]
#![no_main]

use embassy_rp as hal;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_time::Timer;

// I2C
use embassy_rp::i2c::Config as I2cConfig;
use embassy_rp::i2c::{self}; // for convenience, importing as alias

// LCD Driver
use hd44780_driver::HD44780;
use embassy_time::Delay;


//Panic Handler
use {panic_probe as _};
// Defmt Logging
use defmt_rtt as _;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

const LCD_I2C_ADDRESS: u8 = 0x27;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
        let p = embassy_rp::init(Default::default());
        let sda = p.PIN_0;
        let scl = p.PIN_1;

        let mut i2c_config = I2cConfig::default();
        
        i2c_config.frequency = 100_000; //100kHz

        let mut delay = Delay;
        let i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, i2c_config);
        
        // LCD Init
        let mut lcd =
        HD44780::new_i2c(i2c, LCD_I2C_ADDRESS, &mut delay).expect("failed to initialize lcd");
        

        // Write to the top line
        lcd.write_str("Hello, Rust!", &mut delay)
        .expect("failed to write text to LCD");
    
    loop{
        Timer::after_millis(10000).await;
    }   
}


// End of file
