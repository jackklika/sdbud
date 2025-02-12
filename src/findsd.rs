use std::fs;
use std::io;
use std::path::PathBuf;
use udev::Enumerator;
use crate::error::Error;
use tokio::spawn;
use std::string::String;

#[derive(Debug, Clone)]
pub struct Device {
    name: String,
    full_path: PathBuf,
}
impl Device {
    fn from_udev_device(udev_device: udev::Device) -> Result<Device, Error> {
        let sysname = udev_device
            .sysname()
            .to_str()
            .ok_or_else(|| Error::Generic("invalid sysname".to_string()))?;
        let devnode = udev_device
            .devnode()
            .ok_or_else(|| Error::Generic("no device path".to_string()))?;

        Ok(Device {
            name: sysname.to_string(),
            full_path: devnode.to_path_buf(),
        })
    }
}


pub fn list_sd_cards() -> crate::Result<Vec<Device>> {
    //let mut output = Vec::new();

    let mut enumerator = match Enumerator::new() {
        Ok(res) => res,
        Err(_) => return Err(Error::Generic("could not get udev enumerator".to_string())),
    };

    let mut valid_devices: Vec<Device> = Vec::new();
    for device in enumerator.scan_devices().expect("could not scan devices") {
        //println!("{:?}", device.syspath());
        if is_valid_device(&device) {
            println!("\t{:?}", device.syspath());
            let foo_device = Device::from_udev_device(device).expect("could not create foo device");
            println!("{:?}", list_root_dirs_for_device(&foo_device));
            valid_devices.push(foo_device);
        }
    }

    Ok(valid_devices)
}

fn is_valid_device(device: &udev::Device) -> bool {

    // Only consider block devices with a devnode.
    if device.devnode().is_none() || device.subsystem().map(|s| s != "block").unwrap_or(true) {
        return false;
    }

    // Identify flash-based devices.
    if let Some(val) = device.property_value("ID_DRIVE_FLASH_SD") {
        if val.to_str() == Some("1") {
            return true;
        }
    }
    if let Some(val) = device.property_value("ID_DRIVE_FLASH_USB") {
        if val.to_str() == Some("1") {
            return true;
        }
    }

    // Check bus type for USB or MMC (built-in SD readers).
    if let Some(bus) = device.property_value("ID_BUS") {
        let bus = bus.to_str().unwrap_or("").to_lowercase();
        if bus == "usb" || bus == "mmc" {
            return true;
        }
    }

    // Fallback: if the device path contains "usb", assume external.
    if device.devpath().to_string_lossy().contains("/usb") {
        return true;
    }

    false
}

fn list_root_dirs_for_device(device: &Device) -> io::Result<Vec<String>> {
    use std::io::BufRead;

    let dev_str = device
        .full_path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid device path"))?;

    let client = udisks2::Client::new();
    Ok(vec![String::from("Hello")])
}
