# Libegpu

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/egpu.svg)](https://crates.io/crates/egpu)
[![Package on PyPI](https://img.shields.io/pypi/v/egpu.svg)](https://pypi.org/project/egpu)

**Libegpu** is a library for enumerating external GPU (eGPU) devices & enclosures.

<p align="center"><img src="https://github.com/artob/libegpu/raw/master/.img/workbench.jpeg" alt="NVIDIA GeForce RTX 5060 Ti attached to the ADT-Link ADT-UT3G dock" style="max-width: 100%;"></p>

## ✨ Features

- Enumerates PCIe-tunneled eGPU devices from AMD, NVIDIA, and Intel
- Enumerates USB-attached eGPU enclosures from Razer (e.g. the Razer Core X V2)
- Includes an `lsegpu` CLI tool for enumerating eGPU devices & enclosures
- Includes easy-to-use Python bindings to the Rust library (a work in progress)
- 100% free and unencumbered public domain software

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation from PyPI

```bash
pip install -U egpu
```

### Installation from Source Code

```bash
cargo install egpu
```

## 👉 Examples

### Enumerating eGPU Devices

#### Python

```python
import egpu

for device in egpu.devices():
    print(device)
```

#### Rust

```rust
for device in egpu::list_devices().unwrap() {
    println!("{:#?}", device);
}
```

### Enumerating eGPU Controllers

#### Python

```python
import egpu

for controller in egpu.controllers():
    print(controller)
```

#### Rust

```rust
for controller in egpu::list_controllers().unwrap() {
    println!("{:#?}", controller);
}
```

### Enumerating eGPU Enclosures

#### Python

```python
import egpu

for enclosure in egpu.enclosures():
    print(enclosure)
```

#### Rust

```rust
for enclosure in egpu::list_enclosures().unwrap() {
    println!("{:#?}", enclosure);
}
```

## 📚 Reference

[docs.rs/egpu](https://docs.rs/egpu/)

### Supported Hardware

Some of the hardware specifically detected by this library include:

#### eGPU Enclosures

Vendor | Model | Year
:----- | :---- | :---
Razer | Core | 2016
Razer | Core V2 | 2017
Razer | Core X | 2018
Razer | Core X Chroma | 2019
Razer | Core X V2 | 2025

#### eGPU Controllers

Vendor | Model | Year
:----- | :---- | :---
ASMedia | ASM2464PD | 2023
Intel | JHL6540 | 2016
Intel | JHL7440 | 2018
Intel | JHL9480 | 2024

### Resources

#### macOS

- [Modernize PCI and SCSI drivers with DriverKit](https://developer.apple.com/videos/play/wwdc2020/10210/) (WWDC20)
- [System Extensions and DriverKit](https://developer.apple.com/videos/play/wwdc2019/702/) (WWDC19)
- [System Extensions and DriverKit](https://developer.apple.com/system-extensions/)
- [PCIDriverKit Framework](https://developer.apple.com/documentation/pcidriverkit)

## 👨‍💻 Development

```bash
git clone https://github.com/artob/libegpu.git
```

### Developing the Python Bindings

```console
cd sdk/python
maturin develop
source .venv/bin/activate
python3
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/artob/libegpu&text=Libegpu)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/artob/libegpu&title=Libegpu)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/artob/libegpu&t=Libegpu)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/artob/libegpu)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/artob/libegpu)

[Rust]: https://rust-lang.org
