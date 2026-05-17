# LCD Display Setup

## Hardware Requirements 
1. LCD1602 Display with I2C Connection
* The I2C adapter has a built-in potentiometer to control contrast.
* There is a voltage difference between the Pico GPIO Pins (3.3V tolerant) and the LCD with I2C
adapter (5V). This can be handled by using a level shifter to make 5V connection safe to use, or
3.3V connection can be used but screen will be a bit dimmer. 

<img width="655" height="747" alt="lcd1602-i2c" src="https://github.com/user-attachments/assets/7a89e4eb-9cc9-4784-9ac4-a515705d6d75" />

Image Reference: https://pico.implrust.com/lcd-display/hello-rust.html

## Hardware Diagram 
<img width="3055" height="1847" alt="LCD" src="https://github.com/user-attachments/assets/0878352a-be6d-4f76-806b-db29ecdca3b5" />
1. Connect to Ground
2. Connect to Power (3.3V or 5V if using a level shifter)
3. Connect to SDA- GP16 
4. Connect to SCL- GP17
* I2C Adapter: Used to make I2C connection possible
* Potentiometer: Turn to adjust contrast 

<img width="1755" height="959" alt="LCD _bb" src="https://github.com/user-attachments/assets/5aebe67a-4ed1-44b8-9bc5-dad20da8e8fd" />

## Rust Setup
1. Add HD44780 Driver to cargo.toml
   ```
   hd44780-driver = "0.4.0"
   ```
3. I2C Setup
   ```
    let lcd_sda: embassy_rp::Peri<'_, embassy_rp::peripherals::PIN_16> = p.PIN_16;
    let lcd_scl: embassy_rp::Peri<'_, embassy_rp::peripherals::PIN_17> = p.PIN_17;
    let mut lcd_i2c_config = Config::default();
    lcd_i2c_config.frequency = 100_000;
   ```
4. Initialize LCD
```   
let mut lcd =
        HD44780::new_i2c(lcd_i2c, 0x27, &mut Delay).expect("Failed to initialize LCD Screen");
```
6. Ensure to clear screen before writing anything new
   ```
   lcd.reset(&mut Delay).expect("LCD Screen Reset Failed!");
   lcd.clear(&mut Delay).expect("LCD Screen Clear Failed!");
   ```
8. Write string to LCD! 
<img width="1956" height="955" alt="IMG_0985" src="https://github.com/user-attachments/assets/9ae27e69-89ec-4e09-99c3-3f10d84bff75" />


## Resources
https://pico.implrust.com/lcd-display/hello-rust.html
