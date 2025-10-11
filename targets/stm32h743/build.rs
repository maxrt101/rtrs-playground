
const ANSI_RED_FG: &str = "\x1b[31m";
const ANSI_RESET_TEXT: &str = "\x1b[0m";

fn main() {
    println!("cargo::warning={}! ! ! stm32h743 is broken and wont run{}", ANSI_RED_FG, ANSI_RESET_TEXT);

    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Ltargets/stm32h743/");
    println!("cargo:rustc-link-arg=--gc-sections");
}
