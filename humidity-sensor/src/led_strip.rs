//use embassy_rp::gpio::Output;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio_programs::ws2812::PioWs2812;
use smart_leds::RGB8;

// Sets up the number of LEDs in the WS2812 strip
pub const NUM_LEDS: usize = 8;

// Calculates the RGB colors for each of the LEDs on the strip based on relative humidity.
// Will use a green to yellow to orange to red spectrum to indicate comfort (or low to high) humidity, respectively.

/*
LED 1, 2 - Green : 10.0 to 29.9 % humidity (comfy/dry)
LED 3, 4 - Yellow : 30.0 to 49.9% humidity (moderate)
LED 5, 6 - Orange : 50.0 to 69.9% humidity (bit un-comfy/moist)
LED 7, 8 - Red : 70.0 to 100.0% humidity (very un-comfy/damp!)
*/

pub fn calc_colors(humidity: f32) -> [RGB8; NUM_LEDS] {
    let mut colors = [RGB8::default(); NUM_LEDS];

    let ranges = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];

    for i in 0..NUM_LEDS {
        if humidity >= ranges[i] {
            colors[i] = match i {
                0 | 1 => RGB8 { r: 0, g: 255, b: 0 },
                2 | 3 => RGB8 {
                    r: 255,
                    g: 255,
                    b: 0,
                },
                4 | 5 => RGB8 {
                    r: 255,
                    g: 128,
                    b: 0,
                },
                6 | 7 => RGB8 { r: 255, g: 0, b: 0 },
                _ => RGB8::default(),
            };
        }
    }
    colors
}

pub async fn update_strip(
    ws2812: &mut PioWs2812<'static, PIO0, 0, NUM_LEDS, embassy_rp::pio_programs::ws2812::Grb>,
    humidity: f32,
) {
    let colors = calc_colors(humidity);
    ws2812.write(&colors).await;
}

// This is for the 6 individual LED humidity bar based on the sensor's humidiity reading.
// Each LED represents a 10% relative humidity interval starting from 20% up to 70%.
// LEDs will light up cumulatively where 65% humidity would be 5 LEDs lit up.
// Utilized GPIO pins GP10 to GP15)
/*
LED 1, 2 - Green : 0.0 to 39.9 % humidity (dry)
LED 3, 4 - Yellow : 40.0 to 59.9% humidity (moderate)
LED 5, 6 - Red : 60.0 to 100.00% humidity (moist!)


pub fn update_leds(
    humidity: f32,
    led1: &mut Output<'static>,
    led2: &mut Output<'static>,
    led3: &mut Output<'static>,
    led4: &mut Output<'static>,
    led5: &mut Output<'static>,
    led6: &mut Output<'static>,
) {
    // Main threshold conditionals. The else ensures that the LEDs will turn off and not stay on forever!
    if humidity >= 20.0 {
        led1.set_high();
    } else {
        led1.set_low();
    }
    if humidity >= 30.0 {
        led2.set_high();
    } else {
        led2.set_low();
    }
    if humidity >= 40.0 {
        led3.set_high();
    } else {
        led3.set_low();
    }
    if humidity >= 50.0 {
        led4.set_high();
    } else {
        led4.set_low();
    }
    if humidity >= 60.0 {
        led5.set_high();
    } else {
        led5.set_low();
    }
    if humidity >= 70.0 {
        led6.set_high();
    } else {
        led6.set_low();
    }
}
*/
