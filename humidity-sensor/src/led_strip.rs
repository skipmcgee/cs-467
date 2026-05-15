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

// Unit test section!

#[cfg(test)]
mod tests {
  use super::*;

  // Tests with no LEDs lit up for a zero reading.
  #[test]
  fn test_zero() {
    let colors = calc_colors(0.0);
    for color in colors {
      assert_eq!(color.r, 0);
      assert_eq!(color.g, 0);
      assert_eq!(color.b, 0);
    }
  }
  
  // Tests with no LEDs lit up which are under the first 10% threshold.
  #[test]
  fn test_none_under_10() {
    let colors = calc_colors(9.0);
    for color in colors {
      assert_eq!(color.r, 0);
      assert_eq!(color.g, 0);
      assert_eq!(color.b, 0);
    }
  }

  // Test with only the first green LED at 10.0 relative humidity (rh).
  #[test]
  fn test_one_green() {
    let colors = calc_colors(10.0);
    assert_eq!(colors[0], RGB8 {r:0, g: 255, b: 0});
    assert_eq!(colors[1], RGB8::default());
  }

  // Test with the first & second green LEDs at 20.0 relative humidity (rh).
  #[test]
  fn test_both_green() {
    let colors = calc_colors(20.0);
    assert_eq!(colors[0], RGB8 {r:0, g: 255, b: 0});
    assert_eq!(colors[1], RGB8 {r:0, g: 255, b: 0});
    assert_eq!(colors[2], RGB8::default());
  }

  // Test with only the first yellow LED at 30.0 relative humidity (rh).
  #[test]
  fn test_one_yellow() {
    let colors = calc_colors(30.0);
    assert_eq!(colors[2], RGB8 {r:255, g: 255, b: 0});
    assert_eq!(colors[3], RGB8::default());
  }

  // Test with the first & second yellow LEDs at 40.0 relative humidity (rh).
  #[test]
  fn test_both_yellow() {
    let colors = calc_colors(40.0);
    assert_eq!(colors[2], RGB8 {r:255, g: 255, b: 0});
    assert_eq!(colors[3], RGB8 {r:255, g: 255, b: 0});
    assert_eq!(colors[4], RGB8::default());
  }

  // Test with only the first oj LED at 50.0 relative humidity (rh).
  #[test]
  fn test_one_oj() {
    let colors = calc_colors(50.0);
    assert_eq!(colors[4], RGB8 {r:255, g: 128, b: 0});
    assert_eq!(colors[5], RGB8::default());
  }

  // Test with the first & second oj LEDs at 60.0 relative humidity (rh).
  #[test]
  fn test_both_oj() {
    let colors = calc_colors(60.0);
    assert_eq!(colors[4], RGB8 {r:255, g: 128, b: 0});
    assert_eq!(colors[5], RGB8 {r:255, g: 128, b: 0});
    assert_eq!(colors[6], RGB8::default());
  }

  // Test with only the first red LED at 70.0 relative humidity (rh).
  #[test]
  fn test_one_red() {
    let colors = calc_colors(70.0);
    assert_eq!(colors[6], RGB8 {r:255, g: 0, b: 0});
    assert_eq!(colors[7], RGB8::default());
  }

  // Test with the first & second red LEDs at 80.0 relative humidity (rh).
  #[test]
  fn test_both_red() {
    let colors = calc_colors(80.0);
    assert_eq!(colors[6], RGB8 {r:255, g: 0, b: 0});
    assert_eq!(colors[7], RGB8 {r:255, g: 0, b: 0});
  }

  // Test all LEDs at 100.0 relative humidity (rh).
  #[test]
  fn test_max() {
    let colors = calc_colors(100.0);
    assert_eq!(colors[0], RGB8 {r:0, g: 255, b: 0});
    assert_eq!(colors[1], RGB8 {r:0, g: 255, b: 0});
    assert_eq!(colors[2], RGB8 {r:255, g: 255, b: 0});
    assert_eq!(colors[3], RGB8 {r:255, g: 255, b: 0});    
    assert_eq!(colors[4], RGB8 {r:255, g: 128, b: 0});
    assert_eq!(colors[5], RGB8 {r:255, g: 128, b: 0});
    assert_eq!(colors[6], RGB8 {r:255, g: 0, b: 0});
    assert_eq!(colors[7], RGB8 {r:255, g: 0, b: 0});
  }
}
