// This is free and unencumbered software released into the public domain.

/// Known eGPU chip/controller vendors.
pub static KNOWN_VENDORS: [u16; 4] = [
    0x1002, // AMD/ATI
    0x10DE, // NVIDIA
    0x1B21, // ASMedia
    0x8086, // Intel
];

/// Known eGPU controllers on the PCIe bus.
pub static KNOWN_CONTROLLERS: [PciDevice; 1] = [
    // ASMedia ASM2464PD (2023), used in ADT-Link ADT-UT3G
    PciDevice(0x1B21, 0x2461),
    // TODO: Intel JHL6540 (2016), used in Razer Core X
    // TODO: Intel JHL7440 (2018)
    // TODO: Intel JHL9480 (2024), used in Razer Core X V2
];

/// Known eGPU devices on the PCIe bus.
pub static KNOWN_DEVICES: [PciDevice; 2] = [
    // AMD Phoenix3 (2023)
    PciDevice(0x1002, 0x1900),
    // NVIDIA RTX 5060 Ti (2025)
    PciDevice(0x10DE, 0x2D04),
];

pub struct PciDevice(pub u16, pub u16);
