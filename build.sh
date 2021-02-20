rm -rf out || true
mkdir out || true

# Rust compilation
rustc --target arm-unknown-linux-gnueabihf -o out/kernel.o -O --emit=obj src/kernel.rs
arm-none-eabi-ld -T src/boot/linker.ld out/kernel.o -o out/kernel.elf
arm-none-eabi-objdump -D out/kernel.o > out/kernel.list
arm-none-eabi-objdump -d -s out/kernel.elf > out/kernel.elf.list
arm-none-eabi-nm out/kernel.o > out/kernel.dump
arm-none-eabi-nm out/kernel.o > out/kernel.elf.dump


# Final image generation
arm-none-eabi-objcopy out/kernel.elf -O binary out/boot.bin
cat src/boot/toc.bin src/boot/header.bin out/boot.bin > out/rom.img