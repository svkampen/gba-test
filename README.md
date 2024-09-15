# Building
Running `cargo make build-all` will

- build the project
- `objcopy` the ELF binary into a standalone ROM
- `gbafix` the ROM.

The resulting ROM can be found in `target/gba-test.gba`.
