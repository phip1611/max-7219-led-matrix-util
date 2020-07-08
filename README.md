# Util library for MAX7219-powered LED matrix displays written in Rust for Linux (Raspberry Pi).

This is a util library on top of `MAX7219`-crate that allows you to display
text on dot matrix displays. **The main purpose of this lib is educational.
There aren't mappings for all chars yet!** Feel free to contribute on github!

This definitely work's on Raspberry Pi. This is not `no-std` and won't work
on Arduino because it uses `gpio_cdev`-crate which uses linux character device driver
to access GPIO. Should work on other Linux powered devices with GPIO hardware too.

Feel free to learn from the code or to contribute!

![demo](demo.gif)

## Usage example
```
use max_7219_led_matrix_util::setup::setup;
use max_7219_led_matrix_util::{shop_moving_text_in_loop, prepare_display};

const NUM_DISPLAYS: usize = 4;

fn main() {
    // provide three args for the three pins
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 4, "Provide three args!");

    let data_pin = args[1].parse::<u32>().unwrap();
    let cs_pin = args[2].parse::<u32>().unwrap();
    let clk_pin = args[3].parse::<u32>().unwrap();

    println!("data={}, cs={}, clk={}", data_pin, cs_pin, clk_pin);

    let mut display = setup("/dev/gpiochip0", NUM_DISPLAYS, data_pin, cs_pin, clk_pin);
    prepare_display(&mut display, NUM_DISPLAYS, 0x0F);
    shop_moving_text_in_loop(&mut display, "HELLO 01 ABCDEF    ", NUM_DISPLAYS, 50);
}
```

`max_7219_led_matrix_util::setup::setup` returns an instance of a struct defined by crate `max7219` (the actual device driver).
Therefore you can work directly on this driver too without the need of the utility functions. Like `display.write_raw()`.
