
#[macro_export]
macro_rules! print_reg {
    ($name:expr, $value:expr) => {
        println!("{}{}{}\t0x{:08x}{}", rtrs::ANSI_TEXT_BOLD, $name, rtrs::ANSI_COLOR_FG_MAGENTA, $value, rtrs::ANSI_TEXT_RESET);
    };
}

#[macro_export]
macro_rules! print_regs {
    ( $( { $name:expr, $value:expr } ), * $(,)?) => {
        $( print_reg!($name, $value); )*
    };
}
