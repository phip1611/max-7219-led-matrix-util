# Util library for MAX7219-powered LED matrix displays written in Rust for Linux (Raspberry Pi).

This is a `no_std` utility library on top of `max7219`-crate that allows you to easily display
text on dot matrix displays. **The main purpose of this lib is educational. There aren't mappings for all chars yet!**
Feel free to contribute on [Github](https://github.com/phip1611/max-7219-led-matrix-util)!

## `no_std` support vs usage on Raspberry Pi
By default, this crate requires `std` and provide easy setup functions using `gpio_cdev`-crate on Raspberry Pi for example.
If you need `no_std`, disable the default features.
#### Cargo.toml
```toml
[dependencies]
max-7219-led-matrix-util = "<latest-version>"
# or if you need `no_std`
max-7219-led-matrix-util = { version = "<latest-version>", default-features = false }
```

![demo](demo.gif)

## Usage example (`std`)
```rust
use max_7219_led_matrix_util::setup_adapter;
use max_7219_led_matrix_util::{prepare_display, show_moving_text_in_loop};

const NUM_DISPLAYS: usize = 4;

fn main() {
    println!("Demo for the 4-display device by AzDelivery. This is the device in the gif in the README.md.");
    println!();
    println!(
        "Provide 3 pins (gpio pin nums) please and connect all to the device: <data> <cs> <clk>"
    );
    println!("for example: '12 16 21'");
    println!();

    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 4, "Provide three args!");

    let data_pin = args[1].parse::<u32>().unwrap();
    let cs_pin = args[2].parse::<u32>().unwrap();
    let clk_pin = args[3].parse::<u32>().unwrap();

    println!("data={}, cs={}, clk={}", data_pin, cs_pin, clk_pin);

    // display adapter (std-feature, doesn't work in no_std)
    let mut display = setup_adapter("/dev/gpiochip0", NUM_DISPLAYS, data_pin, cs_pin, clk_pin);
    prepare_display(&mut display, NUM_DISPLAYS, 0x0F);
    show_moving_text_in_loop(
        &mut display,
        "HELLO 01 ABCDEF MAPA   ",
        NUM_DISPLAYS,
        // ms for each animation step
        50,
        // max_gap_width
        2
    );
}
```

### MSRV
The MSRV is `1.56.1`.

### Thanks to
Thanks to the creators of the `max7219`-crate!
