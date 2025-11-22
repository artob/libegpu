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
- Includes Python bindings to the Rust library (a work in progress)
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

```rust
for device in egpu::list_devices().unwrap() {
    println!("{:#?}", device);
}
```

### Enumerating eGPU Enclosures

```rust
for enclosure in egpu::list_enclosures().unwrap() {
    println!("{:#?}", enclosure);
}
```

## 📚 Reference

[docs.rs/egpu](https://docs.rs/egpu/)

### Supported Hardware

#### eGPU Enclosures

Vendor | Model | Year |
:------ | :------ | ----: |
Razer | Core X V2 | 2025 |
Razer | Core X Chroma | 2019 |
Razer | Core X | 2018 |
Razer | Core V2 | 2017 |
Razer | Core | 2016 |

## 👨‍💻 Development

```bash
git clone https://github.com/artob/libegpu.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/artob/libegpu&text=Libegpu)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/artob/libegpu&title=Libegpu)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/artob/libegpu&t=Libegpu)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/artob/libegpu)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/artob/libegpu)

[Rust]: https://rust-lang.org
