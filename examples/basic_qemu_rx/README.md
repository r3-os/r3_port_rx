This directory contains an example R3 application for QEMU.

```shell
rustc_codegen_gcc/cargo.sh build --release
rx-elf-objcopy -O binary ../../target/rx-none-elf/release/r3_example_basic_qemu_rx /tmp/app.bin
qemu-system-rx -machine gdbsim-r5f562n8 -bios /tmp/app.bin -nographic -s
# (C-a x to exit)
```
