// This is free and unencumbered software released into the public domain.

/// Known eGPU enclosure vendors.
pub static KNOWN_VENDORS: [u16; 1] = [
    0x1532, // Razer
];

/// Known eGPU enclosures on the USB4/Thunderbolt bus.
pub static KNOWN_ENCLOSURES: [UsbDevice; 6] = [
    // Razer Core (2016)
    UsbDevice(0x1532, 0x0215),
    // Razer Core V2 (2017)
    UsbDevice(0x1532, 0x0000), // TODO
    // Razer Core X (2018)
    UsbDevice(0x1532, 0x0000), // TODO
    // Razer Core X Chroma (2019)
    UsbDevice(0x1532, 0x0F1A),
    // Razer Core X V2 (2025)
    UsbDevice(0x1532, 0x0F51),
    // ASMedia ASM2464PD (2023
    UsbDevice(0x174C, 0x2461),
];

pub struct UsbDevice(pub u16, pub u16);
