# [Renesas RX][3] port for [the R3-OS original kernel][1] (WIP)

## Prerequisites

- rustup
- [`rustc_codegen_gcc`](https://github.com/rust-lang/rustc_codegen_gcc) (experimental GCC codegen for rustc)
    - binutils configured with `--target=rx-elf`
    - [A patched version of libgccjit](https://github.com/rust-lang/rustc_codegen_gcc/blob/14c33f592ae9ecd65c5f7f2436350e8489972a60/Readme.md#building), configured with `--target rx-elf --enable-languages=c,jit --enable-host-shared --disable-werror --disable-multilib --disable-libssp`
    - [A custom fork](https://github.com/yvt/rustc_codegen_gcc/tree/rx/v4) of `rustc_codegen_gcc` for `rx-none-elf` target, built by `./build.sh --release-sysroot`

[1]: https://crates.io/crates/r3_kernel
[2]: https://github.com/rust-lang/rustc_codegen_gcc
[3]: https://en.wikipedia.org/w/index.php?title=RX_microcontroller_family
