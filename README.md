# cs-467

# sensor readings example:

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

Sources and References:

- DHT20 Sensor / Pico Setup: https://rust-classes.com/chapter_embedded_pi_input_dht20
- LCD Screen Setup: https://pico.implrust.com/lcd-display/hello-rust.html
- DHT20 sensor datasheet: https://cdn.sparkfun.com/assets/8/a/1/5/0/DHT20.pdf
- Template Generation: https://github.com/rp-rs/rp2040-project-template
