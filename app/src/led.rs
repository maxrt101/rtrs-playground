use rtrs::led_pattern;

#[allow(dead_code)]
pub static BLINK_PATTERN: rtrs::led::Pattern = led_pattern!(
    rtrs::led::Action::On(500),
    rtrs::led::Action::Off(500),
    rtrs::led::Action::On(500),
    rtrs::led::Action::Off(500),
    rtrs::led::Action::On(500),
    rtrs::led::Action::Off(500),
);
