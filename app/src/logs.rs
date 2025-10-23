use rtrs::log::{meta::{ModuleMetaManager}, Severity, LOGGER_META_OBJECT_NAME};
use rtrs::object_insert;

pub(crate) fn init_logs() {
    object_insert!(LOGGER_META_OBJECT_NAME, ModuleMetaManager::new());

    rtrs::log::register("test", Severity::Trace, 255);
    rtrs::log::register("shell", Severity::Trace, 255);
    rtrs::log::register("radio", Severity::Trace, 2);
    rtrs::log::register("alloc", Severity::Warn, 0);
}
