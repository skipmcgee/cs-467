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
```
        match lcd.write_str(&humidity_string, &mut Delay) {
            Ok(_) => debug!("Success writing to LCD Screen"),
            Err(_) => info!("Error writing humidity value to LCD screen"),
        }
```
<img width="1956" height="955" alt="IMG_0985" src="https://github.com/user-attachments/assets/9ae27e69-89ec-4e09-99c3-3f10d84bff75" />

9. To write temp on the second line, first move cursor, then write string to LCD
```
    lcd.set_cursor_pos(40, &mut Delay).expect("Failed to set cursor position");
        match lcd.write_str(&temperature_string, &mut Delay) {
            Ok(_) => debug!("Success writing to LCD Screen"),
            Err(_) => info!("Error writing temperature value to LCD screen"),
        }
```
<img width="2757" height="1899" alt="IMG_0991" src="https://github.com/user-attachments/assets/4dd4592e-147e-400a-8031-c9bd561a94c6" />

## Optional: Level Shifter 
The GPIO pins on the RP2040 are tolerant to 3.3V, meaning  any higher voltage on the SDA or SCL can damage the Pico. The LCD1602, however, is designed to run at 5V. We have 2 options: 
1. Connect to the 3.3V power, but the display will be dim.
2. Protect the pico using a level shifter to handle the signal converison between 3.3V and 5V safely.

### Circuit Diagram
<img width="1422" height="673" alt="raspberry-pi-pico-2-lcd-1602-level-shifter" src="https://github.com/user-attachments/assets/b3aac150-4ab3-433c-811b-e4bf1c65a1d9" />

### Level Shifter Close-Up
<img width="436" height="336" alt="4 Channel (I2C or SPI) 3 3V-5V Bi-Directional Logic Level Converter" src="https://github.com/user-attachments/assets/7f9f9bbd-1235-4dce-a166-c671b6f76b3f" />

### LCD With 3.3V (No Level Shifter) 
<img width="2757" height="1899" alt="IMG_0991 2" src="https://github.com/user-attachments/assets/a0662ec6-0b9b-41a8-a744-62dcc93dcfac" />

### LCD with Level Shifter
<img width="4032" height="3024" alt="IMG_1127" src="https://github.com/user-attachments/assets/7265e8db-1bf0-4ff1-b61f-7b4c0c4e09d7" />

### Setup with Level Shifter
<img width="4284" height="5712" alt="IMG_1082 3" src="https://github.com/user-attachments/assets/c8eb7197-e9aa-4d12-bbd5-7209d401aa83" />


## Resources
https://pico.implrust.com/lcd-display/hello-rust.html
