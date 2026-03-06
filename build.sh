#!/bin/bash
set -e

COM_PORT=/dev/cu.usbmodem1101


cargo build --release
cargo objcopy --release -- -O binary z_main.bin
uf2conv z_main.bin --base 0x27000 --family 0xADA52840 --output z_main.uf2