use rtrs::sync::{Mutex, RaceAction};

pub enum CallbackType {
    Systick,
    TriggerCrash,
}

struct Callbacks {
    systick: Option<fn()>,
    crash:   Option<fn()>,
}

impl Callbacks {
    const fn empty() -> Self {
        Self {
            systick: None,
            crash: None
        }
    }
}

static CALLBACKS: Mutex<Callbacks> = Mutex::new(Callbacks::empty(), RaceAction::Crash);

pub struct BoardInterface {}

impl BoardInterface {
    pub fn register_callback(t: CallbackType, f: fn()) {
        let mut cbs = CALLBACKS.lock_mut();

        match t {
            CallbackType::Systick => {
                (*cbs).systick = Some(f);
            }
            CallbackType::TriggerCrash => {
                (*cbs).crash = Some(f);
            }
        }
    }

    pub fn callback(t: CallbackType) {
        let cbs = CALLBACKS.lock_mut();

        match t {
            CallbackType::Systick => {
                if let Some(f) = (*cbs).systick {
                    f();
                }
            }
            CallbackType::TriggerCrash => {
                if let Some(f) = (*cbs).crash {
                    f();
                }
            }
        }
    }
}
