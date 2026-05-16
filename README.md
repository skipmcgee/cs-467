# OSU CS467: Microcontroller Home Humidity Sensor Project

## Contents
1. [Overview](https://github.com/skipmcgee/cs-467/edit/revised-documentation/README.md#getting-started)
2. [Getting Started](https://github.com/skipmcgee/cs-467/edit/revised-documentation/README.md#getting-started)
3. [Setup Documentation](https://github.com/skipmcgee/cs-467/edit/revised-documentation/README.md#setup-documentation)
4. [Examples](https://github.com/skipmcgee/cs-467/edit/revised-documentation/README.md#examples)
5. [Authors](https://github.com/skipmcgee/cs-467/edit/revised-documentation/README.md#authors)
6. [Sources and References](https://github.com/skipmcgee/cs-467/edit/revised-documentation/README.md#sources-and-references)
   

## Overview 
Project creates a basic home humidity sensor written in Rust. The sensor ouputs readings to and LCD display and uses LED lights to indicate humidity levels. This project aims to create a low-tech and low-cost humidity sensor as a means to learning the basics of developin in Rust in embedded systems. 

## Getting Started 
#### Necessary Components:
  - Raspberry Pi Pico (RP2040)
  - I2C Temperature and Humidity Sensor (DHT20)
  - I2C LCD 1602x1 Display
  - RGB LED Strip (WS2812)
    
#### Suggested Kit: 
https://a.co/d/07D1i1G4

## Setup Documentation
#### Environment Setup 
* [Ubuntu Setup](docs/UbuntuSetup.md)
* [Mac OS Setup](docs/macOSSetup.md)
* [Windows Setup](docs/windows-SetUp.md)

#### Hardware Setup 
* [Sensor Setup](docs/SENSOR_SETUP.md)
* LCD Display Setup
* LED Strip Setup 

[Images of Setup](docs/images)


## Examples 
### sensor readings example

``` text
4.834829 [INFO ] humidity = 21.455097% (humidity_sensor humidity-sensor/src/main.rs:100)
5.415976 [DEBUG] data = [24, 55, 13, 53, 185, 39] (humidity_sensor humidity-sensor/src/sensor.rs:50)
5.416013 [DEBUG] raw_humidity_data = 370d3 (humidity_sensor humidity-sensor/src/sensor.rs:54)
5.416041 [DEBUG] raw_temperature_data = 5b927 (humidity_sensor humidity-sensor/src/sensor.rs:59)
5.416083 [INFO ] temperature = 21.540642C (humidity_sensor humidity-sensor/src/main.rs:99)
5.416113 [INFO ] humidity = 21.504498% (humidity_sensor humidity-sensor/src/main.rs:100)
5.997262 [DEBUG] data = [24, 54, 237, 245, 185, 112] (humidity_sensor humidity-sensor/src/sensor.rs:50)
5.997300 [DEBUG] raw_humidity_data = 36edf (humidity_sensor humidity-sensor/src/sensor.rs:54)
5.997328 [DEBUG] raw_temperature_data = 5b970 (humidity_sensor humidity-sensor/src/sensor.rs:59)
5.997369 [INFO ] temperature = 21.554565C (humidity_sensor humidity-sensor/src/main.rs:99)
5.997399 [INFO ] humidity = 21.456814% (humidity_sensor humidity-sensor/src/main.rs:100)
```

## Authors 
* skipmcgee
* EricSoOSU
* colomc19

## Sources and References

- DHT20 Sensor / Pico Setup: https://rust-classes.com/chapter_embedded_pi_input_dht20
- LCD Screen Setup: https://pico.implrust.com/lcd-display/hello-rust.html
- DHT20 sensor datasheet: https://cdn.sparkfun.com/assets/8/a/1/5/0/DHT20.pdf
- Template Generation: https://github.com/rp-rs/rp2040-project-template
- DHT20 Specs: https://cdn-shop.adafruit.com/product-files/5183/5193_DHT20.pdf
