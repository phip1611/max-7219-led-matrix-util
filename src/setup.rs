use embedded_hal::digital::v2::OutputPin;
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use max7219::connectors::PinConnector;
use max7219::MAX7219;

/// The type of the driver.
pub type Max7219 = MAX7219<PinConnector<LHandle, LHandle, LHandle>>;

// setup glue structure between crates "embedded_hal", "gpio_cdev" and "max7219 "
/// Wrapper around [`gpio_cdev::LineHandle`] that implements [`embedded_hal::digital::v2::OutputPin`].
#[derive(Debug)]
pub struct LHandle(LineHandle);
impl OutputPin for LHandle {
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0.set_value(0).map_err(|_| ())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0.set_value(1).map_err(|_| ())
    }
}
// setup glue structure done

/// Set's up the MAX7219-driver using "gpio_cdev". This only works on Linux. It works for example
/// on Raspberry. The communication uses SPI protocol. This does not require the "SPI Module" on
/// Raspberry Pi to be activated. This works via regular GPIO pins.
///
/// * `gpio_dev` the gpio device. Probably "/dev/gpiochip0"
/// * `num_displays` number of displays
/// * `data_pin` number of GPIO pin used as data pin
/// * `clk_pin` number of GPIO pin used as clock pin
/// * `cs_pin` number of GPIO pin used as cs (chip select) pin
pub fn setup(
    gpio_dev: &str,
    num_displays: usize,
    data_pin: u32,
    cs_pin: u32,
    clk_pin: u32,
) -> Max7219 {
    let mut gpio = Chip::new(gpio_dev).unwrap();
    let data_pin = gpio
        .get_line(data_pin)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "spi-data-pin")
        .unwrap();
    let cs_pin = gpio
        .get_line(cs_pin)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "spi-cs-pin")
        .unwrap();
    let clk_pin = gpio
        .get_line(clk_pin)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "spi-clk-pin")
        .unwrap();

    let data_pin = LHandle(data_pin);
    let clk_pin = LHandle(clk_pin);
    let cs_pin = LHandle(cs_pin);

    MAX7219::from_pins(num_displays, data_pin, cs_pin, clk_pin).unwrap()
}
