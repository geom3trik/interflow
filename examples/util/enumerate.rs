use interflow::prelude::*;
use std::error::Error;

pub fn enumerate_devices<Driver: AudioDriver>(driver: Driver) -> Result<(), Box<dyn Error>>
where
    <Driver as AudioDriver>::Error: 'static,
{
    eprintln!("Driver name   : {}", Driver::DISPLAY_NAME);
    eprintln!("Driver version: {}", driver.version()?);
    eprintln!("Default device");
    for device_type in [DeviceType::Input, DeviceType::Output] {
        eprint!("\t{device_type:?}:\t");
        if let Some(device) = driver.default_device(device_type)? {
            eprintln!("{}", device.name());
        } else {
            eprintln!("None");
        }
    }

    eprintln!("All devices");
    for device in driver.list_devices()? {
        eprintln!("\t{}", device.name());
    }
    Ok(())
}

pub fn enumerate_duplex_devices<Driver: AudioDuplexDriver>(
    driver: Driver,
) -> Result<(), Box<dyn Error>>
where
    <Driver as AudioDriver>::Error: 'static,
{
    eprintln!("Driver name   : {}", Driver::DISPLAY_NAME);
    eprintln!("Driver version: {}", driver.version()?);
    if let Some(device) = driver.default_duplex_device()? {
        eprintln!("Default duplex device: {}", device.name());
    } else {
        eprintln!("No default duplex device");
    }

    eprintln!("All duplex devices");
    for device in driver.list_duplex_devices()? {
        eprintln!("\t{}", device.name());
    }
    Ok(())
}
