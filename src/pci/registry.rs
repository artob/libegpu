// This is free and unencumbered software released into the public domain.

pub static KNOWN_VENDORS: [u16; 3] = [
    0x10DE, // NVIDIA
    0x1002, // AMD/ATI
    0x8086, // Intel
];

pub static KNOWN_DEVICES: [PciDevice; 0] = [];

pub struct PciDevice {
    pub vid: u16,
    pub did: u16,
}
