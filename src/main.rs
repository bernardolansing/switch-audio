use pulsectl::controllers::{DeviceControl, SinkController};

fn main() {
    let mut handler = SinkController::create().expect("Error while creating sink handler");
    let devices = handler.list_devices().expect("Error while fetching devices");

    let default_device_index = handler.get_default_device()
        .expect("Error fetching default device")
        .index;

    // be aware that device.index is not necessarily the same of the iteration index.
    for (index, device) in devices.iter().enumerate() {
        if device.index == default_device_index {
            let device_to_set_as_default_index: usize =
                if index + 1 == devices.len() { 0 }
                else { index + 1};
            let new_device_name = &devices[device_to_set_as_default_index].name
                .as_ref()
                .expect("Fail while retrieving new default device name");
            let _ = handler.set_default_device(new_device_name);
            break;
        }
    }
}
