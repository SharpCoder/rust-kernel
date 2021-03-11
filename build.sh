rm -rf out || true
mkdir out || true


# Rust compilation
echo "compiling rust"
rustc -C lto --target arm-unknown-linux-gnueabihf -o out/kernel.o -O --emit=obj src/kernel.rs
echo "compiling assembly"
arm-none-eabi-gcc -c src/_start.S -o out/start.o
echo "linking"
arm-none-eabi-ld  -T src/linker.ld out/start.o out/kernel.o -o out/kernel.elf

echo "generating debug info"
arm-none-eabi-objdump -D out/kernel.o > out/kernel.list
arm-none-eabi-objdump -d -s out/kernel.elf > out/kernel.elf.list
arm-none-eabi-nm out/kernel.o > out/kernel.dump
arm-none-eabi-nm out/kernel.o > out/kernel.elf.dump

# Final image generation
echo "generating image file"
arm-none-eabi-objcopy out/kernel.elf -O binary out/boot.bin
cat src/boot/toc.bin src/boot/header.bin out/boot.bin > out/rom.img
du -sh out/rom.img
