//! Provides mappings for a mono spaced font I created myself.
//! The font is not complete and many symbols are missing.
//! This ist just an demonstration how it could be done.
//!
//! Each symbol is exactly `SingleDisplayData * SingleDisplayData` big.

use crate::LED_SQUARE_MATRIX_DIM;

/// We have 8 rows and 8 bits per row.
pub type SingleDisplayData = [u8; LED_SQUARE_MATRIX_DIM];

/// Capital letter A mapping.
pub const CAP_A: SingleDisplayData = [
    0b00111000, 0b01000100, 0b01000100, 0b01000100, 0b01111100, 0b01000100, 0b01000100, 0b01000100,
];
/// Capital letter B mapping.
pub const CAP_B: SingleDisplayData = [
    0b01111000, 0b01000100, 0b01000100, 0b01111000, 0b01000100, 0b01000100, 0b01000100, 0b01111000,
];
/// Capital letter C mapping.
pub const CAP_C: SingleDisplayData = [
    0b01111100, 0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01111100,
];
/// Capital letter D mapping.
pub const CAP_D: SingleDisplayData = [
    0b01111000, 0b01000100, 0b01000100, 0b01000100, 0b01000100, 0b01000100, 0b01000100, 0b01111000,
];
/// Capital letter E mapping.
pub const CAP_E: SingleDisplayData = [
    0b01111100, 0b01000000, 0b01000000, 0b01111100, 0b01000000, 0b01000000, 0b01000000, 0b01111100,
];
/// Capital letter F mapping.
pub const CAP_F: SingleDisplayData = [
    0b01111100, 0b01000000, 0b01000000, 0b01111100, 0b01000000, 0b01000000, 0b01000000, 0b01000000,
];
/// Capital letter G mapping.
pub const CAP_G: SingleDisplayData = [
    0b01111000, 0b11000100, 0b10000100, 0b10000000, 0b10011100, 0b10000100, 0b11000100, 0b01111100,
];
/// Capital letter H mapping.
pub const CAP_H: SingleDisplayData = [
    0b01000100, 0b01000100, 0b01000100, 0b01111100, 0b01000100, 0b01000100, 0b01000100, 0b01000100,
];
/// Capital letter I mapping.
pub const CAP_I: SingleDisplayData = [
    0b00010000, 0b00010000, 0b00010000, 0b00010000, 0b00010000, 0b00010000, 0b00010000, 0b00010000,
];
/// Capital letter J mapping.
pub const CAP_J: SingleDisplayData = [0; 8]; // TODO
/// Capital letter K mapping.
pub const CAP_K: SingleDisplayData = [
    0b01000100, 0b01001000, 0b01010000, 0b01100000, 0b01010000, 0b01001000, 0b01000100, 0b01000010,
];
/// Capital letter L mapping.
pub const CAP_L: SingleDisplayData = [
    0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01111100,
];
/// Capital letter M mapping.
pub const CAP_M: SingleDisplayData = [
    0b10000010, 0b11000110, 0b10101010, 0b10111010, 0b10010010, 0b10000010, 0b10000010, 0b10000010,
];
/// Capital letter N mapping.
pub const CAP_N: SingleDisplayData = [
    0b01000100, 0b01100100, 0b01110100, 0b01010100, 0b01011100, 0b01001100, 0b01001100, 0b01000100,
];
/// Capital letter O mapping.
pub const CAP_O: SingleDisplayData = [
    0b00011000, 0b00100100, 0b01000010, 0b01000010, 0b01000010, 0b01000010, 0b00100100, 0b00011000,
];
/// Capital letter P mapping.
pub const CAP_P: SingleDisplayData = [
    0b01111000, 0b01000100, 0b01000100, 0b01000100, 0b01111000, 0b01000000, 0b01000000, 0b01000000,
];
/// Capital letter Q mapping.
pub const CAP_Q: SingleDisplayData = [0; 8]; // TODO
/// Capital letter R mapping.
pub const CAP_R: SingleDisplayData = [
    0b01111000, 0b01000100, 0b01000100, 0b01111000, 0b01100000, 0b01010000, 0b01001000, 0b01000100,
];
/// Capital letter S mapping.
pub const CAP_S: SingleDisplayData = [
    0b00011100, 0b00100000, 0b01000000, 0b00110000, 0b00001000, 0b00000100, 0b00000100, 0b01111000,
];
/// Capital letter T mapping.
pub const CAP_T: SingleDisplayData = [
    0b11111110, 0b00010000, 0b00010000, 0b00010000, 0b00010000, 0b00010000, 0b00010000, 0b00010000,
];
/// Capital letter U mapping.
pub const CAP_U: SingleDisplayData = [
    0b01000010, 0b01000010, 0b01000010, 0b01000010, 0b01000010, 0b01000010, 0b01000010, 0b00111100,
];
/// Capital letter V mapping.
pub const CAP_V: SingleDisplayData = [0; 8]; // TODO
/// Capital letter W mapping.
pub const CAP_W: SingleDisplayData = [0; 8]; // TODO
/// Capital letter X mapping.
pub const CAP_X: SingleDisplayData = [0; 8]; // TODO
/// Capital letter Y mapping.
pub const CAP_Y: SingleDisplayData = [0; 8]; // TODO
/// Capital letter Z mapping.
pub const CAP_Z: SingleDisplayData = [
    0b01111110, 0b00000010, 0b00000100, 0b00001000, 0b00010000, 0b00100000, 0b01000000, 0b01111110,
];
/// Number 0 mapping.
pub const ZERO: SingleDisplayData = [
    0b00111000, 0b01000100, 0b01000100, 0b01000100, 0b01000100, 0b01000100, 0b01000100, 0b00111000,
];
/// Number 1 mapping.
pub const ONE: SingleDisplayData = [
    0b00000100, 0b00011100, 0b00000100, 0b00000100, 0b00000100, 0b00000100, 0b00000100, 0b00000100,
];
/// " " character mapping.
pub const SPACE: SingleDisplayData = [0; 8];
/// "." character mapping.
pub const DOT: SingleDisplayData = [0, 0, 0, 0, 0, 0, 0, 0b00010000];
/// "!" character mapping.
pub const EXCLAMATION_MARK: SingleDisplayData = [
    0b00010000, 0b00010000, 0b00010000, 0b00010000, 0b00010000, 0b00010000, 0, 0b00010000,
];
