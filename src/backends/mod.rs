use crate::{
    AudioDevice, AudioDriver, AudioInputDevice, AudioOutputCallback, AudioOutputDevice, DeviceType,
};

#[cfg(os_alsa)]
pub mod alsa;

#[cfg(os_wasapi)]
pub mod wasapi;

pub fn default_driver() -> impl AudioDriver {
    #[cfg(os_alsa)]
    return alsa::AlsaDriver;
    #[cfg(os_wasapi)]
    return wasapi::WasapiDriver;
}

pub fn default_input_device_from<Driver: AudioDriver>(driver: &Driver) -> Driver::Device
where
    Driver::Device: Clone + AudioInputDevice,
{
    driver
        .default_device(DeviceType::Input)
        .expect("Audio driver error")
        .expect(
            "No \
    default device found",
        )
        .clone()
}

pub fn default_input_device() -> impl AudioInputDevice {
    #[cfg(os_alsa)]
    return default_input_device_from(&alsa::AlsaDriver);
    #[cfg(os_wasapi)]
    return default_input_device_from(&wasapi::WasapiDriver);
}

pub fn default_output_device_from<Driver: AudioDriver>(driver: &Driver) -> Driver::Device
where
    Driver::Device: Clone + AudioOutputDevice,
{
    driver
        .default_device(DeviceType::Output)
        .expect("Audio driver error")
        .expect("No default device found")
        .clone()
}

pub fn default_output_device() -> impl AudioOutputDevice {
    #[cfg(os_alsa)]
    return default_output_device_from(&alsa::AlsaDriver);
    #[cfg(os_wasapi)]
    return default_output_device_from(&wasapi::WasapiDriver);
}
