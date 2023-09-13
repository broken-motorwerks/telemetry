use std::net::TcpStream;
use std::time::Duration;

use anyhow::{Ok, Result};
use model::EngineData;
use postcard::from_bytes;
use serde;
use serde::{Deserialize, Serialize};
use serialport;

fn main() -> Result<()> {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    test_sonnerie().unwrap();

    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_millis(10))
        .open()?;

    let mut serial_buf: Vec<u8> = vec![0; 100];
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(num_bytes) => {
                if let Ok(engine_data) = from_bytes::<EngineData>(&serial_buf[..num_bytes]) {
                    println!("RPM: {}", engine_data.rpm);
                }
            }
            Err(_) => (),
        }
        let _ = port.write(&[1, 2, 3]);
    }
    Ok(())
}
