# EFR32MG22 PAC

## How this was generated
SVD file were taken from https://www.keil.arm.com/devices/silicon-labs-efr32mg22c224f512im40/features/.

Boilerplate generated with

```bash
svd2rust -i EFR32MG22E224F512IM40.svd --reexport-core-peripherals --reexport-interrupt
form -i lib.rs -o src/ && rm lib.rs
```

Then, fixed some `unsafe` handling because `#[no_mangle]` requires `unsafe` now.
Apart from that the `svd2rust` code is umodified.

Created `Cargo.toml` based on <https://docs.rs/svd2rust/0.37.1/svd2rust/#peripheral-api>.

Formatted the code
```bash
cargo fmt
```

Additionally, a public constant `NVIC_PRIO_BITS` has been added as it is required for using [rtic](https://rtic.rs).

### Radio Interrupts
Although not included in the original System View Description, this PAC additionally defines all radio interrupts (e.g. RFSENSE, BUFC, MODEM, ...).

## Crate Documentation
Documentation can be generated with `cargo doc --open`.

## Example Usages
To be done.
