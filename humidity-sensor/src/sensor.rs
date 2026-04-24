// Adapted code from: https://rust-classes.com/chapter_embedded_pi_input_dht20
// Referenced the DHT20 sensor datasheet: https://cdn.sparkfun.com/assets/8/a/1/5/0/DHT20.pdf
use defmt::debug;
use embassy_rp::{
    i2c::{Async, I2c},
    peripherals::I2C1,
};
use embassy_time::Timer;
use embedded_hal_async::i2c::I2c as I2cAsync;

const DHT20_I2C_ADDR: u8 = 0x38;
const DHT20_GET_STATUS: u8 = 0x71;
const DHT20_READ_DATA: [u8; 3] = [0xAC, 0x33, 0x00];
const DIVISOR: f32 = 2u32.pow(20) as f32;
const TEMP_DIVISOR: f32 = DIVISOR / 200.0;

pub async fn initialize(i2c: &mut I2c<'static, I2C1, Async>) -> bool {
    // per the data sheet: "After power-on, wait no less than 100ms"
    Timer::after_millis(100).await;

    // per the data dheet: " Before reading the temperature and humidity value, get a byte of statusword by sending 0x71"
    let mut data = [0x0; 1];
    i2c.write_read(DHT20_I2C_ADDR, &[DHT20_GET_STATUS], &mut data)
        .await
        .expect("Can not read status");

    // Per the data sheet: "If the status word and 0x18 are not equal to 0x18, initialize the 0x1B, 0x1C,
    // 0x1E registers, details Please refer to our official website routine for the initialization process;
    // if they are equal, proceed to the next step."
    data[0] & 0x18 == 0x18
}

// From the data sheet: "Wait 10ms to send the 0xAC command (trigger measurement). This command parameter has
// two bytes, the first byte is 0x33, and the second byte is 0x00. 3.Wait 80ms for the measurement to be completed,
// if the read status word Bit [7] is 0, it means the measurement iscompleted, and then six bytes can be read continuously;
// otherwise, continue to wait. After receiving six bytes, the next byte is the CRC check data. The user can read it out as needed. "
async fn read_data(i2c: &mut I2c<'static, I2C1, Async>) -> [u8; 6] {
    let mut data = [0x0; 6];

    for _ in 0..10 {
        i2c.write(DHT20_I2C_ADDR, &DHT20_READ_DATA)
            .await
            .expect("Can not write data");
        Timer::after_millis(80).await;

        i2c.read(DHT20_I2C_ADDR, &mut data)
            .await
            .expect("Can not read data");

        if data[0] >> 7 == 0 {
            break;
        }
    }

    data
}

pub async fn read_temperature_and_humidity(i2c: &mut I2c<'static, I2C1, Async>) -> (f32, f32) {
    let data = read_data(i2c).await;

    debug!("data = {:?}", data);

    let raw_hum_data =
        ((data[1] as u32) << 12) + ((data[2] as u32) << 4) + (((data[3] & 0xf0) >> 4) as u32);

    debug!("raw_humidity_data = {:x}", raw_hum_data);

    let humidity = (raw_hum_data as f32) / DIVISOR * 100.0;

    let raw_temp_data =
        (((data[3] as u32) & 0xf) << 16) + ((data[4] as u32) << 8) + (data[5] as u32);

    debug!("raw_temperature_data = {:x}", raw_temp_data);

    let temperature = (raw_temp_data as f32) / TEMP_DIVISOR - 50.0;

    // return the 2 desred parameters, converted to units
    (temperature, humidity)
}
