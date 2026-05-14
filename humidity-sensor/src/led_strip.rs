use embassy_rp::peripherals::PIO0;
use embassy_rp::pio_programs::ws2812::PioWs2812;
use smart_leds::RGB8;

// Sets up the number of LEDs in the WS2812 strip
pub const NUM_LEDS: usize = 8;

// Calculates the RGB colors for each of the LEDs on the strip based on relative humidity.
// Will use a green to yellow to orange to red spectrum to indicate comfort (or low to high) humidity, respectively.

/*
LED 1, 2 - Green : 20.0 to 34.9 % humidity (comfy/dry)
LED 3, 4 - Yellow : 35.0 to 39.9% humidity (moderate)
LED 5, 6 - Orange : 40.0 to 100.00% humidity (bit un-comfy/moist)
LED 7, 8 - Red : 60.0 to 100.00% humidity (very un-comfy/damp!)
*/ 

pub fn calc_colors(humidity: f32) -> [RGB8; NUM_LEDS] {
  let mut colors = [RGB8::default(); NUM_LEDS];

  let ranges = [20.0, 27.5, 35.0, 42.5, 50.0, 57.5, 65.0, 72.5];

  for i in 0..NUM_LEDS {
    if humidity >= ranges[i] {
      colors[i] = match i {
        0|1 => RGB8 {r: 0, g: 255, b:0},
        2|3 => RGB8 {r: 255, g: 255, b:0},
        4|5 => RGB8 {r: 255, g: 128, b:0},
        6|7 => RGB8 {r: 255, g: 0, b:0},
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
