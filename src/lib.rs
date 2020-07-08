//! Provides utility functions on top of "max7219"-crate to display data (like text) on a
//! MAX7219 powered matrix display.

use crate::mappings::SingleDisplayData;
use crate::setup::Max7219;
use crate::encoding::encode_string;
use std::thread::sleep;
use std::time::Duration;

/// We use 8x8 square matrices (per single display)
pub const LED_SQUARE_MATRIX_DIM: usize = 8;

/// Maximum supported chained displays by MAX7219.
pub const MAX_DISPLAYS: usize = 16;

pub mod mappings;
pub mod encoding;
pub mod setup;

/// Shift all row bits one to the left (to the next col). This way you can animate a moving text.
///
/// * `moving_bits` Vector with the data of all content to display. Each index describes
///                 the 8x8 bit data for a single display.
// * `repeat` shift 1 bits on the very left to the ending of the vector. Without repeat
//            the vector will be all zeros after enough iterations.
pub fn shift_all_rows_one_bit_left(moving_bits: &mut Vec<SingleDisplayData>/*, repeat: bool*/) {
    // move all bits to next position

    // so we iterate through the whole vector
    // note that probably only [0]..[DISPLAY_COUNT] are shown; this are the active displays
    // while that bits are shifted though the vec!

    let len = moving_bits.len();
    for display_i in 0..len {
        for row_i in 0..8 {
            // we need to shift to next segment if MSB per row is 1
            if moving_bits[display_i][row_i] & 0b10000000 != 0 {
                if display_i == 0 /*&& repeat*/ {
                    // to the last display
                    moving_bits[len - 1][row_i] |= 1;
                } else {
                    // to display from previous iteration
                    moving_bits[display_i - 1][row_i] |= 1;
                }
            }
            // shift all in row on to the left
            moving_bits[display_i][row_i] = moving_bits[display_i][row_i] << 1;
        }
    }
}

/// Shows a moving text in loop. After each iteration all bits are shifted one col to the left.
///
/// * `display` - mutable reference to Max7219 display driver
/// * `text` - the text to display
/// * `display_count` - count of displays connected to the MAX7219
/// * `ms_sleep` - timeout after each iteration
/// * `intensity` - brightness for the display; value between `0x00` and `0x0F`
pub fn shop_moving_text_in_loop(display: &mut Max7219, text: &str, display_count: usize, ms_sleep: u64, intensity: u8) {
    let displays = display_count % MAX_DISPLAYS;

    display.power_on().unwrap();
    for i in 0..displays {
        display.set_intensity(i, intensity);
    }

    let mut bits = encode_string(text);
    loop {
        for i in 0..displays {
            display.write_raw(i, &bits[i]).unwrap();
        }

        sleep(Duration::from_millis(ms_sleep));
        // shift all rows one bit to the left
        shift_all_rows_one_bit_left(&mut bits);
    }
}

/// Iterates through the data and removes all gaps between symbols. A gap is two or more cols
/// after each other that are all zero.
fn remove_letter_spacing(_moving_bits: &mut Vec<SingleDisplayData>) {
    unimplemented!(); // TODO!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_all_bits_one_col_left() {
        let data_dis_0 = [
            0b01000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
        ];
        let data_dis_1 = [
            0b11000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000,
        ];
        let mut data = Vec::new();
        data.push(data_dis_0);
        data.push(data_dis_1);

        shift_all_rows_one_bit_left(&mut data);

        let first_row_dis_0_expected = 0b10000001;
        let first_row_dis_1_expected = 0b10000000;
        let first_row_dis_0_actual = data[0][0];
        let first_row_dis_1_actual = data[1][0];

        assert_eq!(first_row_dis_0_actual, first_row_dis_0_expected);
        assert_eq!(first_row_dis_1_actual, first_row_dis_1_expected);
    }

}
