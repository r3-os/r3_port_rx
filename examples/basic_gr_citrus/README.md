This directory contains an example R3 application for [GR-CITRUS](https://www.renesas.com/us/en/products/gadget-renesas/boards/gr-citrus).

Enter the USB mass storage loader mode by pressing the reset button on your board while it's connected to a host machine. Then do the following:

```shell
rustc_codegen_gcc/cargo.sh build --release
rx-none-elf-objcopy -O binary ../../target/rx-none-elf/release/r3_example_basic_gr_citrus /tmp/app.bin
cp /tmp/app.bin /run/media/$USER/GR-CITRUS_b
```
