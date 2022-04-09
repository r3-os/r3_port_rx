use std::env;

fn main() {
    // Use the linker script `memory.ld` at the crate root
    println!(
        "cargo:rustc-link-search={}",
        env::current_dir().unwrap().display()
    );
}
