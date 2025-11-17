// This is free and unencumbered software released into the public domain.

pub fn main() {
    #[cfg(feature = "usb")]
    {
        println!("# eGPU Enclosures:");
        for enclosure in egpu::list_enclosures().unwrap() {
            println!("- {:#?}", enclosure);
        }
    }

    #[cfg(feature = "pci")]
    {
        println!("# eGPU Devices:");
        for device in egpu::list_devices().unwrap() {
            println!("- {:#?}", device);
        }
    }
}
