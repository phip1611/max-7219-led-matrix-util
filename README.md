# Util library for MAX7219-powered LED matrix displays written in Rust for Linux (Raspberry Pi).

This is a util library on top of `MAX7219`-crate that allows you to display
text on dot matrix displays. **The main purpose of this lib is educational.
There aren't mappings for all chars yet!** Feel free to contribute on github!

This definitely work's on Raspberry Pi. This is not `no-std` and won't work
on Arduino because it uses `gpio_cdev`-crate which uses linux character device driver
to access GPIO. Should work on other Linux powered devices with GPIO hardware too.

Feel free to learn from the code or to contribute!

![demo](demo.gif)