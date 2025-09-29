//! List DVB devices available to the system

use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

/// A DVB adapter currently attached to the system.
#[derive(Debug)]
pub struct Adapter {
    adapter_id: String,
    manufacturer: String,
    product: String,
    id_vendor: String,
    id_product: String,
    serial: String,
    frontend_count: usize,
    demux_count: usize,
    dvr_count: usize,
    net_count: usize,
}

impl Adapter {
    /// Returns the manufacturer string of the device
    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    /// Returns a description of the device
    pub fn product(&self) -> &str {
        &self.product
    }

    /// Returns the Vendor ID of the device
    pub fn id_vendor(&self) -> &str {
        &self.id_vendor
    }

    /// Returns the Product ID of the device
    pub fn id_product(&self) -> &str {
        &self.id_product
    }

    /// Returns a Serial number for this device. Useful to disambiguate multiple identical adapters.
    pub fn serial(&self) -> &str {
        &self.serial
    }

    /// Returns a path to the first frontend of this adapter.
    pub fn get_first_frontend(&self) -> PathBuf {
        if self.frontend_count < 1 {
            panic!("dvb adapter does not have even 1 frontend. How is this possible ?")
        }

        format_dev_adapter(&self.adapter_id).join("frontend0")
    }

    /// Returns a path to the first demux of this adapter.
    pub fn get_first_demux(&self) -> PathBuf {
        if self.demux_count < 1 {
            panic!()
        }

        format_dev_adapter(&self.adapter_id).join("demux0")
    }

    pub fn get_first_dvr(&self) -> Option<PathBuf> {
        if self.dvr_count < 1 {
            return None;
        }

        Some(format_dev_adapter(&self.adapter_id).join("dvr0"))
    }

    pub fn get_first_net(&self) -> Option<PathBuf> {
        if self.net_count < 1 {
            return None;
        }

        Some(format_dev_adapter(&self.adapter_id).join("net0"))
    }
}

fn format_dev_adapter(adapter_id: &str) -> PathBuf {
    PathBuf::from("/")
        .join("dev")
        .join("dvb")
        .join(format!("adapter{}", adapter_id))
}

/// List all DVB adapters recognized by the system.
pub fn list_all_adapters() -> Vec<Adapter> {
    // TODO: Terrible code but oh well it seems to work. Could use /dev/dvb/ instead

    let base_path = PathBuf::from("/sys/class/dvb");

    let mut adapters: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for entry in read_dir(base_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let thing = path.to_str().unwrap();
        let (device, elm) = thing.split_once('.').unwrap();
        adapters
            .entry(device.to_string())
            .or_default()
            .push((thing.to_string(), elm.to_string()));
    }

    let mut better: Vec<Adapter> = Vec::new();
    for (key, value) in adapters.iter() {
        let path = PathBuf::from(value[0].0.clone());

        let device_dir = path.join("device");

        // Read info about adapter
        let manufacturer = read_to_string(device_dir.join("manufacturer"))
            .unwrap()
            .trim()
            .to_string();
        let product = read_to_string(device_dir.join("product"))
            .unwrap()
            .trim()
            .to_string();
        let id_vendor = read_to_string(device_dir.join("idVendor"))
            .unwrap()
            .trim()
            .to_string();
        let id_product = read_to_string(device_dir.join("idProduct"))
            .unwrap()
            .trim()
            .to_string();
        let serial = read_to_string(device_dir.join("serial"))
            .unwrap()
            .trim()
            .to_string();

        // Count sub-devices
        let mut frontend_count = 0;
        let mut demux_count = 0;
        let mut dvr_count = 0;
        let mut net_count = 0;
        for (_, d) in value {
            if d.starts_with("frontend") {
                frontend_count += 1
            } else if d.starts_with("demux") {
                demux_count += 1
            } else if d.starts_with("dvr") {
                dvr_count += 1
            } else if d.starts_with("net") {
                net_count += 1
            }
        }

        better.push(Adapter {
            // Keep only the number part
            adapter_id: key["/sys/class/dvb/dvb".len()..].to_string(),
            manufacturer,
            product,
            id_vendor,
            id_product,
            serial,
            frontend_count,
            demux_count,
            dvr_count,
            net_count,
        });
    }

    better
}
