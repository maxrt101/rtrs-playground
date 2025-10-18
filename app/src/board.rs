use rtrs::sync::RwLock;
use rtrs::time::TickProvider;

use alloc::boxed::Box;

pub enum CallbackType {
    Systick(fn()),
    TriggerCrash(fn()),
    MicrosecondDelay(fn(u32)),
    MicrosecondTickProvider(fn(&mut Box<dyn TickProvider<Tick = u32>>)),
}

pub enum Callback<'a> {
    Systick,
    TriggerCrash,
    MicrosecondDelay(u32),
    MicrosecondTickProvider(&'a mut Box<dyn TickProvider<Tick = u32>>),
}

struct Callbacks {
    systick:                   Option<fn()>,
    crash:                     Option<fn()>,
    microsecond_delay:         Option<fn(u32)>,
    microsecond_tick_provider: Option<fn(&mut Box<dyn TickProvider<Tick = u32>>)>
}

impl Callbacks {
    const fn empty() -> Self {
        Self {
            systick:                   None,
            crash:                     None,
            microsecond_delay:         None,
            microsecond_tick_provider: None
        }
    }
}

static CALLBACKS: RwLock<Callbacks> = RwLock::new(Callbacks::empty());

pub struct BoardInterface {}

impl BoardInterface {
    pub fn register_callback(cb: CallbackType) {
        let mut cbs = CALLBACKS.lock_mut();

        match cb {
            CallbackType::Systick(f) => {
                (*cbs).systick = Some(f);
            }
            CallbackType::TriggerCrash(f) => {
                (*cbs).crash = Some(f);
            }
            CallbackType::MicrosecondDelay(f) => {
                (*cbs).microsecond_delay = Some(f);
            }
            CallbackType::MicrosecondTickProvider(f) => {
                (*cbs).microsecond_tick_provider = Some(f);
            }
        }
    }

    pub fn callback(cb: Callback) {
        let cbs = CALLBACKS.lock();

        match cb {
            Callback::Systick => {
                if let Some(f) = (*cbs).systick {
                    f();
                }
            }
            Callback::TriggerCrash => {
                if let Some(f) = (*cbs).crash {
                    f();
                }
            }
            Callback::MicrosecondDelay(us) => {
                if let Some(f) = (*cbs).microsecond_delay {
                    f(us);
                }
            }
            Callback::MicrosecondTickProvider(provider) => {
                if let Some(f) = (*cbs).microsecond_tick_provider {
                    f(provider);
                }
            }
        }

    }
}
