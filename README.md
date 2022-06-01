# [Renesas RX][3] port for [the R3-OS original kernel][1] (WIP)

## Prerequisites

- rustup
- [`rustc_codegen_gcc`](https://github.com/rust-lang/rustc_codegen_gcc) (experimental GCC codegen for rustc)
    - binutils configured with `--target=rx-none-elf`
    - [A patched version of libgccjit](https://github.com/rust-lang/rustc_codegen_gcc/blob/14c33f592ae9ecd65c5f7f2436350e8489972a60/Readme.md#building), configured with `--target rx-none-elf --enable-languages=c,jit --enable-host-shared --disable-werror --disable-multilib --disable-libssp`
    - [A custom fork](https://github.com/yvt/rustc_codegen_gcc/tree/rx/v8) of `rustc_codegen_gcc` for `rx-none-elf` target, built by `./build.sh --release-sysroot`

### Nix

Using [Nix][5], you can start a shell with the prerequisites by the following command: `nix shell `[`github:yvt/nix-rustc_codegen_gcc`][4]`#rx-embedded-gcc-rustenv nixpkgs#{pkgsCross.rx-embedded.buildPackages.binutils,rustup}`. (note: You need Nix version 2.9 or later.)

The example programs can be built by `cd examples/$name; rx-embedded-gcc-cargo build --release -Zbuild-std`.

[1]: https://crates.io/crates/r3_kernel
[2]: https://github.com/rust-lang/rustc_codegen_gcc
[3]: https://en.wikipedia.org/w/index.php?title=RX_microcontroller_family
[4]: https://github.com/yvt/nix-rustc_codegen_gcc
[5]: https://nixos.org/

