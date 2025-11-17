// This is free and unencumbered software released into the public domain.

pub fn main() {
    println!("Enclosures:");
    for enclosure in egpu::usb::list_enclosures().unwrap() {
        println!("- {:#?}", enclosure);
    }

    println!("Devices:");
    for device in egpu::pci::list_devices().unwrap() {
        println!("- {:#?}", device);
    }
}
