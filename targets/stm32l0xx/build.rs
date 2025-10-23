
fn main() {
    #[cfg(feature = "mcu-stm32l073")]
    std::fs::copy("memory_l073.x", "memory.x").unwrap();
    #[cfg(feature = "mcu-stm32l051")]
    std::fs::copy("memory_l051.x", "memory.x").unwrap();

    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Ltargets/stm32l0xx/");
    println!("cargo:rustc-link-arg=--gc-sections");
}
