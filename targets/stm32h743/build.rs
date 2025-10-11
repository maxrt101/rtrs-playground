
fn main() {
    println!("cargo::warning=stm32h743 is broken and wont run");

    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Ltargets/stm32h743/");
    println!("cargo:rustc-link-arg=--gc-sections");
}
