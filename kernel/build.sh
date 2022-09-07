#!/bin/bash

cd "${0%/*}" || exit 1
rm -rf dist ||:
mkdir dist

# needs the armv7a-none-eabi Rust toolchain and arm-none-eabi-binutils:
# rustup target add armv7a-none-eabi
# pacman -S arm-none-eabi-binutils

cargo rustc -- -C link_arg=--script=./linker.ld
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/kernel dist/kernel7.img

curl -o dist/fixup.dat https://raw.githubusercontent.com/raspberrypi/firmware/master/boot/fixup.dat
curl -o dist/start.elf https://raw.githubusercontent.com/raspberrypi/firmware/master/boot/start.elf
curl -o dist/bootcode.bin https://raw.githubusercontent.com/raspberrypi/firmware/master/boot/bootcode.bin