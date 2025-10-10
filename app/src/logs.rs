use rtrs::log::{meta::{ModuleMetaManager}, Severity, LOGGER_META_OBJECT_NAME};
use rtrs::object_insert;

pub(crate) fn init_logs() {
    object_insert!(LOGGER_META_OBJECT_NAME,  ModuleMetaManager::new());

    rtrs::log::register("main", Severity::Trace, 255);
    rtrs::log::register("task", Severity::Trace, 255);

    rtrs::log::register("TEST", Severity::Trace, 255);
    rtrs::log::register("SHELL", Severity::Trace, 255);
}
