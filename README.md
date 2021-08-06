# PACs for nRF microcontrollers

![CI](https://github.com/nrf-rs/nrf-pacs/workflows/CI/badge.svg)

This repository contains Peripheral Access Crates (PACs) for Nordic's nRF series of Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust].

For a more user-friendly interface to the peripherals, the [`nrf-hal`] crates might be more appropriate.

Please refer to the [changelog] to see what changed in the last releases.

[changelog]: ./CHANGELOG.md
[`nrf-hal`]: https://github.com/nrf-rs/nrf-hal/
[svd2rust]: https://github.com/rust-embedded/svd2rust

## Crates

Every nRF chip has its own PAC, listed below:

| Crate | Docs | crates.io | target |
|-------|------|-----------|--------|
| `nrf51-pac` | [![docs.rs](https://docs.rs/nrf51-pac/badge.svg)](https://docs.rs/nrf51-pac) | [![crates.io](https://img.shields.io/crates/d/nrf51-pac.svg)](https://crates.io/crates/nrf51-pac) | `thumbv6m-none-eabi` |
| `nrf52805-pac` | [![docs.rs](https://docs.rs/nrf52805-pac/badge.svg)](https://docs.rs/nrf52805-pac) | [![crates.io](https://img.shields.io/crates/d/nrf52805-pac.svg)](https://crates.io/crates/nrf52805-pac) | `thumbv7em-none-eabi` |
| `nrf52810-pac` | [![docs.rs](https://docs.rs/nrf52810-pac/badge.svg)](https://docs.rs/nrf52810-pac) | [![crates.io](https://img.shields.io/crates/d/nrf52810-pac.svg)](https://crates.io/crates/nrf52810-pac) | `thumbv7em-none-eabi` |
| `nrf52811-pac` | [![docs.rs](https://docs.rs/nrf52811-pac/badge.svg)](https://docs.rs/nrf52811-pac) | [![crates.io](https://img.shields.io/crates/d/nrf52811-pac.svg)](https://crates.io/crates/nrf52811-pac) | `thumbv7em-none-eabi` |
| `nrf52820-pac` | [![docs.rs](https://docs.rs/nrf52820-pac/badge.svg)](https://docs.rs/nrf52820-pac) | [![crates.io](https://img.shields.io/crates/d/nrf52820-pac.svg)](https://crates.io/crates/nrf52820-pac) | `thumbv7em-none-eabi` |
| `nrf52832-pac` | [![docs.rs](https://docs.rs/nrf52832-pac/badge.svg)](https://docs.rs/nrf52832-pac) | [![crates.io](https://img.shields.io/crates/d/nrf52832-pac.svg)](https://crates.io/crates/nrf52832-pac) | `thumbv7em-none-eabihf` |
| `nrf52833-pac` | [![docs.rs](https://docs.rs/nrf52833-pac/badge.svg)](https://docs.rs/nrf52833-pac) | [![crates.io](https://img.shields.io/crates/d/nrf52833-pac.svg)](https://crates.io/crates/nrf52833-pac) | `thumbv7em-none-eabihf` |
| `nrf52840-pac` | [![docs.rs](https://docs.rs/nrf52840-pac/badge.svg)](https://docs.rs/nrf52840-pac) | [![crates.io](https://img.shields.io/crates/d/nrf52840-pac.svg)](https://crates.io/crates/nrf52840-pac) | `thumbv7em-none-eabihf` |
| `nrf5340-app-pac` | [![docs.rs](https://docs.rs/nrf5340-app-pac/badge.svg)](https://docs.rs/nrf5340-app-pac) | [![crates.io](https://img.shields.io/crates/d/nrf5340-app-pac.svg)](https://crates.io/crates/nrf5340-app-pac) | `thumbv8m.main-none-eabihf` |
| `nrf5340-net-pac` | [![docs.rs](https://docs.rs/nrf5340-net-pac/badge.svg)](https://docs.rs/nrf5340-net-pac) | [![crates.io](https://img.shields.io/crates/d/nrf5340-net-pac.svg)](https://crates.io/crates/nrf5340-net-pac) | `thumbv8m.main-none-eabihf` |
| `nrf9160-pac` | [![docs.rs](https://docs.rs/nrf9160-pac/badge.svg)](https://docs.rs/nrf9160-pac) | [![crates.io](https://img.shields.io/crates/d/nrf9160-pac.svg)](https://crates.io/crates/nrf9160-pac) | `thumbv8m.main-none-eabihf` |

## Device Reference Manuals from Nordic

| Device | Product Specification | DK Reference Guide |
|-------|------|-----------|
| [`nRF52805`](https://www.nordicsemi.com/Products/nrf52805) | [`v1.2`](https://infocenter.nordicsemi.com/pdf/nRF52805_PS_v1.2.pdf) | [`v1.3.1*`](https://infocenter.nordicsemi.com/pdf/nRF52_DK_User_Guide_v1.3.1.pdf) |
| [`nRF52810`](https://www.nordicsemi.com/products/nrf52810) | [`v1.3`](https://infocenter.nordicsemi.com/pdf/nRF52810_PS_v1.3.pdf) | [`v1.3.1*`](https://infocenter.nordicsemi.com/pdf/nRF52_DK_User_Guide_v1.3.1.pdf) |
| [`nRF52811`](https://www.nordicsemi.com/products/nrf52811) | [`v1.0`](https://infocenter.nordicsemi.com/pdf/nRF52811_PS_v1.0.pdf) | [`v1.3.1*`](https://infocenter.nordicsemi.com/pdf/nRF52_DK_User_Guide_v1.3.1.pdf) |
| [`nRF52820`](https://www.nordicsemi.com/Products/nRF52820) | [`v1.0`](https://infocenter.nordicsemi.com/pdf/nRF52820_PS_v1.0.pdf) | [`v1.0.1`](http://infocenter.nordicsemi.com/pdf/nRF52833_DK_User_Guide_v1.0.1.pdf) |
| [`nRF52832`](https://www.nordicsemi.com/products/nrf52832) | [`v1.4`](https://infocenter.nordicsemi.com/pdf/nRF52832_PS_v1.4.pdf) | [`v1.3.1*`](https://infocenter.nordicsemi.com/pdf/nRF52_DK_User_Guide_v1.3.1.pdf) |
| [`nRF52833`](https://www.nordicsemi.com/products/nrf52833) | [`v1.3`](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.3.pdf) | [`v1.0.1`](http://infocenter.nordicsemi.com/pdf/nRF52833_DK_User_Guide_v1.0.1.pdf) |
| [`nRF52840`](https://www.nordicsemi.com/Products/nRF52840) | [`v1.1`](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf) | [`v1.2`](https://infocenter.nordicsemi.com/pdf/nRF52840_DK_User_Guide_v1.2.pdf) |
| [`nRF5340`](https://www.nordicsemi.com/Products/nRF5340) | [`v1.1`](https://infocenter.nordicsemi.com/pdf/nRF5340_PS_v1.1.pdf) | [`v1.0.0`](https://infocenter.nordicsemi.com/pdf/nRF5340_DK_User_Guide_20210304.pdf) |
| [`nRF9160`](https://www.nordicsemi.com/Products/nrf9160) | [`v2.0`](https://infocenter.nordicsemi.com/pdf/nRF9160_PS_v2.0.pdf) | [`v0.9.3`](https://infocenter.nordicsemi.com/pdf/nRF9160_DK_HW_User_Guide_v0.9.3.pdf) |

\* These devices do not have a separate development kit and share the [NRF52 DK](https://www.nordicsemi.com/Software-and-tools/Development-Kits/nRF52-DK)
