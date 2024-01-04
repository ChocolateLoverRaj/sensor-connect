use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{IOPin, PinDriver, Pull},
    peripherals::Peripherals,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Starting 0-input\nThis application is a basic blinky program that turns an LED on and off every 1 second.\n");

    // Get all the peripherals
    let peripherals = Peripherals::take().unwrap();
    // Initialize Pin 8 as an output to drive the LED
    // let mut btn_pin = PinDriver::input(peripherals.pins.gpio4.downgrade()).unwrap();
    // btn_pin.set_pull(Pull::Down).unwrap();

    let pins = peripherals.pins;
    let mut led_pin = PinDriver::output(pins.gpio8).unwrap();
    let mut inputs = vec![
        pins.gpio4.downgrade(),
        pins.gpio3.downgrade(),
        // pins.gpio2.downgrade(), // See https://www.espressif.com/sites/default/files/documentation/esp32-c3_datasheet_en.pdf - 2.3.3 Restrictions for GPIOs
        pins.gpio1.downgrade(),
        pins.gpio0.downgrade(),
        pins.gpio5.downgrade(),
        pins.gpio6.downgrade(),
        pins.gpio7.downgrade(),
        // pins.gpio8.downgrade(), // See https://www.espressif.com/sites/default/files/documentation/esp32-c3_datasheet_en.pdf - 2.3.3 Restrictions for GPIOs
        // pins.gpio9.downgrade(), // See https://www.espressif.com/sites/default/files/documentation/esp32-c3_datasheet_en.pdf - 2.3.3 Restrictions for GPIOs
        pins.gpio10.downgrade(),
        pins.gpio20.downgrade(),
        pins.gpio21.downgrade(),
    ]
    .into_iter()
    .map(|input| PinDriver::input(input).unwrap())
    .collect::<Vec<_>>();

    for input in &mut inputs {
        input.set_pull(Pull::Down).unwrap();
    }
    // Loop forever blinking the LED on/off every 500ms
    loop {
        let one_input_on = inputs.iter().any(|input| input.is_high());

        if one_input_on {
            led_pin.set_low().unwrap();
        } else {
            led_pin.set_high().unwrap();
        }

        println!(
            "{:?}",
            inputs
                .iter()
                .map(|input| input.is_high())
                .collect::<Vec<_>>()
        );
        // FreeRtos::delay_ms(1000);
    }
}
