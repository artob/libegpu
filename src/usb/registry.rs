// This is free and unencumbered software released into the public domain.

pub static KNOWN_VENDORS: [u16; 1] = [
    0x1532, // Razer
];

pub static KNOWN_ENCLOSURES: [UsbDevice; 5] = [
    // Razer Core (2016)
    UsbDevice {
        vid: 0x1532,
        pid: 0x0215,
    },
    // Razer Core V2 (2017)
    UsbDevice {
        vid: 0x1532,
        pid: 0x0000, // TODO
    },
    // Razer Core X (2018)
    UsbDevice {
        vid: 0x1532,
        pid: 0x0000, // TODO
    },
    // Razer Core X Chroma (2019)
    UsbDevice {
        vid: 0x1532,
        pid: 0x0F1A,
    },
    // Razer Core X V2 (2025)
    UsbDevice {
        vid: 0x1532,
        pid: 0x0F51,
    },
];

pub struct UsbDevice {
    pub vid: u16,
    pub pid: u16,
}
