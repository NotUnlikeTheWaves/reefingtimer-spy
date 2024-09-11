
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::Gpio0;
use esp_idf_svc::hal::spi;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::gpio::PinDriver;
// use embedded_graphics::*;
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    // application code
    // 128x160 pixels

    let peripherals = Peripherals::take().unwrap();
    let spi = peripherals.spi3;
    let sclk = peripherals.pins.gpio18;
    let sdo = peripherals.pins.gpio23;
    let sdi = Option::<Gpio0>::None;
    let cs = Option::<Gpio0>::None;
    let driver_config = Default::default();
    let spi_config = spi::SpiConfig::new().baudrate(1.MHz().into());
    let spi =
        spi::SpiDeviceDriver::new_single(spi, sclk, sdo, sdi, cs, &driver_config, &spi_config).unwrap();

    let rst = PinDriver::output(peripherals.pins.gpio17).unwrap();
    let dc = PinDriver::output(peripherals.pins.gpio5).unwrap();


    let mut display = st7735_lcd::ST7735::new(spi, dc, rst, false,false, 128, 160);
    let mut delay = FreeRtos;
    display.init(&mut delay);
    {
        let this = display.set_pixel(10, 10, 0x07e0);
        match this {
            Ok(t) => t,
            Err(e) => log::info!("Error writing pixel"),
        }
    };
    log::info!("1");
    display.set_pixel(10, 11, 0x07e0);
    log::info!("2");
    display.set_pixel(11, 10, 0x07e0);
    log::info!("3");
    display.set_pixel(11, 11, 0x07e0);
    log::info!("EOL");
}
