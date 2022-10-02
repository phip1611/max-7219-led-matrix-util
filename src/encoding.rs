//! Encoding utility functions that helps you to print symbols easily on a
//! MAX7219-powered LED matrix.

use crate::mappings::*;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Encodes a char to its bit-representation on a single display. This means a 8x8 bit matrix.
/// Currently only a very limited alphabet is available and only capital letters. Unknown chars
/// are mapped to SPACE (empty).
#[allow(clippy::wildcard_in_or_patterns)]
pub const fn encode_char(c: char) -> SingleDisplayData {
    // currently we only support cap chars
    let c = c.to_ascii_uppercase();
    match c {
        'A' => CAP_A,
        'B' => CAP_B,
        'C' => CAP_C,
        'D' => CAP_D,
        'E' => CAP_E,
        'F' => CAP_F,
        'G' => CAP_G,
        'H' => CAP_H,
        'I' => CAP_I,
        'J' => CAP_J,
        'K' => CAP_K,
        'L' => CAP_L,
        'M' => CAP_M,
        'N' => CAP_N,
        'O' => CAP_O,
        'P' => CAP_P,
        'Q' => CAP_Q,
        'R' => CAP_R,
        'S' => CAP_S,
        'T' => CAP_T,
        'U' => CAP_U,
        'V' => CAP_V,
        'W' => CAP_W,
        'X' => CAP_X,
        'Y' => CAP_Y,
        'Z' => CAP_Z,

        '0' => ZERO,
        '1' => ONE,

        '.' => DOT,
        '!' => EXCLAMATION_MARK,

        ' ' | _ => SPACE,
    }
}

/// Encodes each char of a string to its bit-representation. The resulting vector should be understood
/// as the data to be displayed on a display chain that is as long as the coupled/chained displays
/// that are powered by the MAX7219.
/// So if you have 4 displays and 10 chars then you could only display indices zero to three and
/// bit shift all rows one to the left per iteration. This way you get a smooth transition/animation.
/// Don't forget about the sleep-timeout per iteration!
pub fn encode_string(s: &str) -> Vec<SingleDisplayData> {
    s.chars().into_iter().map(encode_char).collect()
}
