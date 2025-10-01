# nrf-pac

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for Nordic Semiconductor nRF microcontrollers.

This crate has been automatically generated from the SVD files in [nrfx](https://github.com/NordicSemiconductor/nrfx), using [chiptool](https://github.com/embassy-rs/chiptool/). Fixes are added to the SVD file to make the
crate more amenable to writing HALs with, such as converting sets of identical registers/fields to arrays, merging identical registers and enums, etc.

This crate is used for the [`embassy-nrf`](https://github.com/embassy-rs/embassy/tree/main/embassy-nrf) Rust Hardware Abstraction Layer (HAL) for the nRF microcontrollers.

## Supported chips


- nRF51xx
- nRF52805
- nRF52810
- nRF52811
- nRF52820
- nRF52832
- nRF52833
- nRF52840
- nRF5340 appplication core
- nRF5340 network core
- nRF54L15 appplication core
- nRF9120 (SoC used in nRF9131, nRF9161 and nRF9151)
- nRF9160

## License

The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file, which is licensed by Nordic Semiconductor under a BSD-3-Clause license.
