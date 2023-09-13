use std::thread;
use std::time::Duration;

use anyhow::Result;
use esp_can_rx::messages::Messages;
use esp_can_rx::EngineData;
use esp_idf_hal::can;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::uart::*;
use esp_idf_hal::units::Hertz;
use esp_idf_sys as _;
use postcard::to_slice;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let uart_config = UartConfig::default().baudrate(Hertz(115_200));

    let uart = UartDriver::new(
        peripherals.uart1,
        peripherals.pins.gpio1,
        peripherals.pins.gpio3,
        Option::<Gpio0>::None, // had to provide turbo fish to qualify type
        Option::<Gpio1>::None,
        &uart_config,
    )?;

    // Configure CAN
    let timing = can::config::Timing::B1M;
    let config = can::config::Config::new().timing(timing);
    let mut can = can::CanDriver::new(
        peripherals.can,
        peripherals.pins.gpio5,
        peripherals.pins.gpio4,
        &config,
    )?;

    // Spawn a thread to read CAN messages
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || loop {
        match can.receive(500) {
            // TODO: likely want to record the time the data was received
            // TODO: does number of ticks to block matter here?
            Ok(msg) => tx.send(msg).unwrap_or(()),
            Err(_) => (),
        }
    });

    let mut output_pin = PinDriver::output(peripherals.pins.gpio7)?;

    loop {
        // Toggle LED to indicate we are alive
        output_pin.toggle()?;

        let mut msg_iter = rx.try_iter();
        while let Some(frame) = msg_iter.next() {
            // Convert raw [Frame] into can_message
            if let Ok(esp_can_rx::messages::Messages::Dme1(dme1)) =
                Messages::from_can_message(frame.identifier(), frame.data())
            {
                println!("Received DME1: {:?}", dme1);
                let engine_data = EngineData { rpm: dme1.rpm() };
                let mut buffer = [0u8; 100];
                let used = to_slice(&engine_data, &mut buffer[..])?;
                uart.write(used)?;
            } else {
                println!("Failed to parse frame {}", frame);
            }
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}
