//! Provides utility functions on top of "max7219"-crate to display data (like text) on a
//! MAX7219 powered matrix display.

#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::fallible_impl_from,
    clippy::needless_doctest_main,
    clippy::redundant_pub_crate,
    clippy::suboptimal_flops
)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
#[cfg_attr(not(feature = "std"), macro_use)]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use crate::setup::Max7219;

#[cfg(feature = "std")]
use std::{thread::sleep, time::Duration};

use crate::encoding::encode_string;
use crate::mappings::SingleDisplayData;
use max7219::DecodeMode;

/// We use 8x8 square matrices (per single display)
pub const LED_SQUARE_MATRIX_DIM: usize = 8;

/// Maximum supported chained displays by MAX7219.
pub const MAX_DISPLAYS: usize = 16;

pub mod encoding;
pub mod mappings;
#[cfg(feature = "std")]
mod setup;
#[cfg(feature = "std")]
pub use setup::{setup as setup_adapter, Max7219 as Max7219Adapter};

/// Shift all row bits one to the left (to the next col). This way you can animate a moving text.
///
/// * `moving_bits` Vector with the data of all content to display. Each index describes
///                 the 8x8 bit data for a single display.
/// * `repeat` shift 1 bits on the very left to the ending of the vector. Without repeat
//            the vector will be all zeros after enough iterations.
pub fn shift_all_rows_one_bit_left(moving_bits: &mut [SingleDisplayData] /*, repeat: bool*/) {
    // move all bits to next position

    // so we iterate through the whole vector
    // note that probably only [0]..[DISPLAY_COUNT] are shown; this are the active displays
    // while that bits are shifted though the vec!

    let len = moving_bits.len();
    for display_i in 0..len {
        for row_i in 0..8 {
            // we need to shift to next segment if MSB per row is 1
            if moving_bits[display_i][row_i] & 0b10000000 != 0 {
                if display_i == 0
                /*&& repeat*/
                {
                    // to the last display
                    moving_bits[len - 1][row_i] |= 1;
                } else {
                    // to display from previous iteration
                    moving_bits[display_i - 1][row_i] |= 1;
                }
            }
            // shift all in row on to the left
            moving_bits[display_i][row_i] <<= 1;
        }
    }
}

/// Convenient function that turns on the display, clears the display
/// and sets the brightness to the highest possible value. It also sets
/// the DecodeMode to NoDecode which is necessary for displaying content on
/// the 8x8 matrix display. (Max7219 can also be used for 7 segment displays).
///
/// * `display` - mutable reference to Max7219 display driver
/// * `display_count` - count of displays connected to the MAX7219
/// * `intensity` - brightness for the display; value between `0x00` and `0x0F`
#[cfg(feature = "std")]
pub fn prepare_display(display: &mut Max7219, display_count: usize, intensity: u8) {
    let display_count = display_count % MAX_DISPLAYS;

    display.power_on().unwrap();
    for i in 0..display_count {
        display.set_decode_mode(i, DecodeMode::NoDecode).unwrap();
        display.clear_display(i).unwrap();
        display.set_intensity(i, intensity).unwrap();
    }
}

/// Shows a moving text in loop. After each iteration all bits are shifted one col to the left.
/// **Make sure to call `prepare_display()` first!**
///
/// * `display` - mutable reference to Max7219 display driver
/// * `text` - the text to display
/// * `display_count` - count of displays connected to the MAX7219
/// * `ms_sleep` - timeout after each iteration
/// * `gap_width` - (optional) set's the maximum width/count of empty cols between characters. 0 to deactivate.
///                 Downside is that spaces will be removed.
#[cfg(feature = "std")]
pub fn show_moving_text_in_loop(
    display: &mut Max7219,
    text: &str,
    display_count: usize,
    ms_sleep: u64,
    gap_width: Option<usize>,
) {
    let display_count = display_count % MAX_DISPLAYS;

    let raw_bits = encode_string(text);
    let mut display_data_vec: std::vec::Vec<SingleDisplayData> = if let Some(gap_width) = gap_width {
        remove_gaps_in_display_text(&raw_bits, gap_width)
    } else {
        raw_bits
    };

    loop {
        for (i, display_data) in display_data_vec.iter().enumerate().take(display_count) {
            display.write_raw(i, display_data).unwrap();
        }
        sleep(Duration::from_millis(ms_sleep));
        // shift all rows one bit to the left
        shift_all_rows_one_bit_left(&mut display_data_vec);
    }
}

/// Iterates through the data and removes all gaps between symbols. A gap is two or more cols
/// after each other that are all zero. This way, text looks more natural, as letters are closer
/// together. Hence, we do not have a monospace font here. But (currently), also spaces are removed.
///
/// # Parameters
/// - `display_data_arr`: Processed Display Data with letters
/// - `min_gap_size`: Minimum gap size to preserve between symbols
pub fn remove_gaps_in_display_text(
    display_data_arr: &[SingleDisplayData],
    min_gap_size: usize,
) -> std::vec::Vec<SingleDisplayData> {
    // all data in one single vector; each SingleDisplayData is transposed at first
    // => the vector contains all pixels but in (col1, colX, colN)-order.
    let display_data_expanded: std::vec::Vec<u8> = display_data_arr
        .iter()
        // transpose: rows become cols
        .map(transpose_single_display_data)
        .flat_map(|display_data| display_data.into_iter())
        .collect();

    let preserve_at_begin = display_data_expanded
        .iter()
        .take_while(|col| **col == 0)
        .count();
    let preserve_at_end = display_data_expanded
        .iter()
        .rev()
        .take_while(|col| **col == 0)
        .count();

    // This variable holds the whole display data (col1, colX, ..., colN) but with
    // removed gaps in between. We keep empty cols at the begin.
    let mut shrinked_display_data_expanded = vec![0_u8; preserve_at_begin];

    let mut count_since_last_not_empty = 0;
    let skip_begin = preserve_at_begin;
    let skip_end = preserve_at_end;
    let take_elements = display_data_expanded.len() - skip_end - skip_begin + 1;

    // iterate and find gaps in between that can be removed
    for col in display_data_expanded.iter().take(take_elements).copied() {
        if col == 0 {
            count_since_last_not_empty += 1;
        } else {
            count_since_last_not_empty = 0;
        }

        // we preserve `min_gap_size` empty cols, after that, we skip them
        if count_since_last_not_empty <= min_gap_size {
            shrinked_display_data_expanded.push(col);
        }
    }

    // keep empty cols at end
    shrinked_display_data_expanded.extend_from_slice(&vec![0; preserve_at_end]);

    // now transform again to Vec<SingleDisplayData>
    // 1) check if length is multiple of eight
    let add_to_multiple_of_8 = 8 - (shrinked_display_data_expanded.len() % 8);
    shrinked_display_data_expanded.extend_from_slice(&vec![0; add_to_multiple_of_8]);

    // display data without gaps in-between ready for display.
    let shrinked_display_data = shrinked_display_data_expanded
        .as_slice()
        .chunks_exact(8)
        .map(|cols_8| {
            // transform to u8 array of length 8
            let mut transposed_display: SingleDisplayData = [0; 8];
            cols_8
                .iter()
                .copied()
                .enumerate()
                .for_each(|(i, col)| transposed_display[i] = col);
            transpose_single_display_data(&transposed_display)
        })
        .collect();

    shrinked_display_data
}

/// This does a transpose operation on the [`SingleDisplayData`]-Matrix and is a helper function for
/// [`remove_gaps_in_display_text`]. Cols become rows and rows become cols.
/// Example:
/// ```
/// let _ = [
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
/// ];
/// ```
/// becomes
/// ```
/// let _ = [
/// 0b11111111,
/// 0,
/// 0,
/// 0,
/// 0,
/// 0,
/// 0,
/// 0,
/// ];
/// ```
#[allow(clippy::needless_range_loop)]
pub fn transpose_single_display_data(data: &SingleDisplayData) -> SingleDisplayData {
    let mut transposed_data: SingleDisplayData = [0; 8];
    for col_i in 0..8 {
        // the data/bits of the current col
        let mut col = 0;
        for row_i in 0..8 {
            // we get the current col value (bit) at index "data[row_i][col_i]"
            // bit by bit. We move the current bit to the lowest index via
            // bit shifting and do a bitwise and with 1.
            let col_bit = (data[row_i] >> (7 - col_i)) & 1;
            // now we first shift all col bits from previous iterations one index
            // to the left and then we add our current bit to the col at the lowest index.
            col = (col << 1) | col_bit;
        }
        transposed_data[col_i] = col;
    }
    transposed_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_all_bits_one_col_left() {
        let data_dis_0 = [
            0b01000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000,
        ];
        let data_dis_1 = [
            0b11000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
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

    #[test]
    fn test_transpose_single_display_data() {
        let input = [
            0b0100_0000,
            0b0100_0000,
            0b0100_0000,
            0b0100_0000,
            0b0000_0011,
            0b0000_0011,
            0b0000_0011,
            0b0000_0011,
        ];
        let expected = [
            0b0000_0000,
            0b1111_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_1111,
            0b0000_1111,
        ];

        let actual = transpose_single_display_data(&input);

        for i in 0..input.len() {
            assert_eq!(
                actual[i], expected[i],
                "swap_cols_to_rows() doesn't transposed the matrix properly at index {}! is={:#b}, expected={:#b}",
                i, actual[i], expected[i]
            );
        }
    }

    #[test]
    fn test_remove_gaps_in_display_text() {
        let vec = vec![
            [
                0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000,
                0b10000000,
            ],
            [
                0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000,
                0b10000000,
            ],
        ];
        let expected = vec![
            [
                0b10010000, 0b10010000, 0b10010000, 0b10010000, 0b10010000, 0b10010000, 0b10010000,
                0b10010000,
            ],
            [
                // TODO remove last if only empty?!
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ];
        let actual = remove_gaps_in_display_text(&vec, 2);
        for i in 0..2 {
            for j in 0..8 {
                assert_eq!(
                    actual[i][j], expected[i][j],
                    "expected: {:#b}, is: {:#b}",
                    expected[i][j], vec[i][j]
                );
            }
        }
    }
}
