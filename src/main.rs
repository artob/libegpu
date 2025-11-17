// This is free and unencumbered software released into the public domain.

pub fn main() {
    #[cfg(feature = "usb")]
    {
        println!("Enclosures:");
        for enclosure in egpu::usb::list_enclosures().unwrap() {
            println!("- {:#?}", enclosure);
        }
    }

    #[cfg(feature = "pci")]
    {
        println!("Devices:");
        for device in egpu::pci::list_devices().unwrap() {
            println!("- {:#?}", device);
        }
    }
}
