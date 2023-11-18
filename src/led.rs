use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;

use super::pattern::Pattern;

static NUM_LEDS: usize = 110;

pub fn turn_leds_on(pattern: &Pattern) {
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();

    let rgb_values = pattern_to_rgb(pattern);

    adapter.write_rgb(&rgb_values).unwrap();
}

fn col_row_to_rgb_value(pattern: &Pattern, col: u32, row: u32) -> (u8, u8, u8) {
    let mut led = (0, 0, 0);

    let index = row * pattern.width() + col;

    let is_on = pattern.fields()[index as usize].is_on;

    if is_on {
        led = (255, 255, 255);
    }

    led
}

fn pattern_to_rgb(pattern: &Pattern) -> Vec<(u8, u8, u8)> {
    let mut rgb_values = vec![(0, 0, 0); NUM_LEDS];

    let mut counter = 0;
    let mut top_to_bottom = false;

    for column in 0..pattern.width() {
        if top_to_bottom {
            for row in 0..pattern.height() {
                rgb_values[counter as usize] = col_row_to_rgb_value(pattern, column, row);
                counter += 1;
            }
        } else {
            for row in (0..pattern.height()).rev() {
                rgb_values[counter as usize] = col_row_to_rgb_value(pattern, column, row);
                counter += 1;
            }
        };

        top_to_bottom = !top_to_bottom;
    }

    rgb_values
}
