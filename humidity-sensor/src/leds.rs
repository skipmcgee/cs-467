use embassy_rp::gpio::Output;

// This is for the 6 individual LED humidity bar based on the sensor's humidiity reading. 
// Each LED represents a 10% relative humidity interval starting from 20% up to 70%. 
// LEDs will light up cumulatively where 65% humidity would be 5 LEDs lit up.

pub fn update_leds(
  humidity: f32,
  led1: &mut Output<'static>,
  led2: &mut Output<'static>,
  led3: &mut Output<'static>,
  led4: &mut Output<'static>,
  led5: &mut Output<'static>,
  led6: &mut Output<'static>,
) {
  if humidity >= 20.0 {led1.set_high();} else {led1.set_low();}
  if humidity >= 30.0 {led2.set_high();} else {led1.set_low();}
  if humidity >= 40.0 {led3.set_high();} else {led1.set_low();}
  if humidity >= 50.0 {led4.set_high();} else {led1.set_low();}
  if humidity >= 60.0 {led5.set_high();} else {led1.set_low();}
  if humidity >= 70.0 {led6.set_high();} else {led1.set_low();}
}
