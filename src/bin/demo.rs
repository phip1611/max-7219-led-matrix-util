//! Demo for the 4-display device by AzDelivery. This is the device in the gif in the README.md.
//! (https://www.az-delivery.de/products/4-x-64er-led-matrix-display).

use max_7219_led_matrix_util::setup::setup;
use max_7219_led_matrix_util::shop_moving_text_in_loop;

const NUM_DISPLAYS: usize = 4;

fn main() {
    println!("Demo for the 4-display device by AzDelivery. This is the device in the gif in the README.md.");
    println!();
    println!("Provide 3 pins (gpio pin nums) please and connect all to the device: <data> <cs> <clk>");
    println!("for example: '12 16 21'");
    println!();

    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 4, "Provide three args!");

    let data_pin = args[1].parse::<u32>().unwrap();
    let cs_pin = args[2].parse::<u32>().unwrap();
    let clk_pin = args[3].parse::<u32>().unwrap();

    println!("data={}, cs={}, clk={}", data_pin, cs_pin, clk_pin);

    let mut display = setup("/dev/gpiochip0", NUM_DISPLAYS, data_pin, cs_pin, clk_pin);
    shop_moving_text_in_loop(&mut display, "HELLO 01 ABCDEF", NUM_DISPLAYS, 50, 0x0F);
}