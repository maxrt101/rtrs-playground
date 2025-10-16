use core::alloc::Layout;
use core::fmt::Write;

use rtrs::log::meta::ModuleMetaManager;
use rtrs::object::STORAGE;
use rtrs::log::Severity;
use rtrs::shell::script::Runtime;
use rtrs::{
    println,
    command,
    shell,
    object_with,
    object_with_mut,
    bit_set,
    bit_if,
    log,
    logger,
    trace,
    info,
};

logger!("SHELL");

fn cmd_panic(_rt: &mut Runtime, args: &[&str]) -> i8 {
    panic!("{}", args.get(0).map_or("Manual panic", |v| v));
}

fn cmd_crash(_rt: &mut Runtime, _args: &[&str]) -> i8 {
    crate::board::BoardInterface::callback(crate::board::CallbackType::TriggerCrash);
    0
}

fn cmd_test(_rt: &mut Runtime, args: &[&str]) -> i8 {
    fn help() {
        println!("test [help|all|task|task-irq|task-nest|task-obj|logger|hexdump|box|heap]")
    }

    enum Test {
        Task,
        TaskIrq,
        TaskNest,
        TaskObj,
        TaskSched,
        TaskSchedThis,
        TaskSchedCancel,
        Logger,
        Hexdump,
        Box,
        Heap,
    }

    let mut tests: u32 = 0;

    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match *arg {
            "all"               => tests = 0xFF,
            "task"              => bit_set!(tests, Test::Task),
            "task-irq"          => bit_set!(tests, Test::TaskIrq),
            "task-nest"         => bit_set!(tests, Test::TaskNest),
            "task-obj"          => bit_set!(tests, Test::TaskObj),
            "task-sched"        => bit_set!(tests, Test::TaskSched),
            "task-sched-this"   => bit_set!(tests, Test::TaskSchedThis),
            "task-sched-cancel" => bit_set!(tests, Test::TaskSchedCancel),
            "logger"            => bit_set!(tests, Test::Logger),
            "hexdump"           => bit_set!(tests, Test::Hexdump),
            "box"               => bit_set!(tests, Test::Box),
            "heap"              => bit_set!(tests, Test::Heap),
            "help" => {
                help();
                return 0;
            },
            arg => {
                println!("Unknown subcommand: {}", arg);
                help();
                return 1;
            }
        }
    }

    bit_if!(tests, Test::Task, {
        trace!("Running Test::Task");
        crate::test_tasks()
    });

    bit_if!(tests, Test::TaskIrq, {
        trace!("Running Test::TaskIrq");
        crate::test_irq_tasks()
    });

    bit_if!(tests, Test::TaskNest, {
        trace!("Running Test::TaskNest");
        crate::test_nested_tasks()
    });

    bit_if!(tests, Test::TaskObj, {
        trace!("Running Test::TaskObj");
        crate::test_task_object()
    });

    bit_if!(tests, Test::TaskSched, {
        trace!("Running Test::TaskSched");
        crate::test_task_sched()
    });

    bit_if!(tests, Test::TaskSchedThis, {
        trace!("Running Test::TaskSchedThis");
        crate::test_task_sched_this()
    });

    bit_if!(tests, Test::TaskSchedCancel, {
        trace!("Running Test::TaskSchedCancel");
        crate::test_task_sched_cancel()
    });

    bit_if!(tests, Test::Logger, {
        trace!("Running Test::TestLogger");
        crate::test_logger()
    });

    bit_if!(tests, Test::Hexdump, {
        trace!("Running Test::Hexdump");
        crate::test_hexdump()
    });

    bit_if!(tests, Test::Box, {
        trace!("Running Test::Box");
        crate::test_box()
    });

    bit_if!(tests, Test::Heap, {
        trace!("Running Test::Heap");
        crate::test_heap()
    });

    0
}

fn cmd_obj(_rt: &mut Runtime, args: &[&str]) -> i8 {
    match args.get(0).map(|v| *v) {
        Some("list") | None => {
            let storage = STORAGE.lock();
            for key in storage.keys() {
                println!("  {}", key);
            }
        }
        Some(arg) => {
            println!("Unknown subcommand: {}", arg);
        }
    }

    0
}

fn cmd_mem(_rt: &mut Runtime, args: &[&str]) -> i8 {
    fn help() {
        println!("mem [help|info|alloc|free]");
    }

    match args.get(0).map(|v| *v) {
        Some("info") => {
            crate::GLOBAL_HEAP.dump();
        }
        Some("alloc") => {
            let size = match args.get(1) {
                Some(arg) => arg.parse::<usize>().unwrap_or(0),
                None => 0
            };
            let ptr = crate::GLOBAL_HEAP.allocate(unsafe { Layout::from_size_align_unchecked(size, 4) });
            println!("alloc({}): {:?}", size, ptr);
        }
        Some("free") => {
            let ptr = match args.get(1) {
                Some(arg) => usize::from_str_radix(arg, 16).unwrap_or(0),
                None => 0
            };

            if ptr == 0 {
                println!("Invalid value for pointer");
                return 1;
            }

            crate::GLOBAL_HEAP.free(ptr as *mut u8);
        }
        Some("help") | Some(_) | None => {
            help();
        }
    }

    0
}

fn cmd_log(_rt: &mut Runtime, args: &[&str]) -> i8 {
    match args.get(0).map(|v| *v) {
        Some("help") => {
            println!("log help - Shows this message");
            println!("log / log list - Shows all modules");
            println!("log severity|s MOD VAL - Set max severity for module");
            println!("log level|l MOD VAL - Set max level for module");
            println!("log MOD SEV LVL - Set max severity & level for module");
            println!("log print|p SEV LVL ... - Print log message");
        }
        Some("list") | None => {
            object_with!(log::LOGGER_META_OBJECT_NAME, ModuleMetaManager, meta, {
                for (name, meta) in meta.iter() {
                    println!("{}: >{} <{}", name, meta.severity, meta.level);
                }
            });
        }
        Some("severity") | Some("s") => {
            let module = args.get(1).map_or("", |v| v);
            let value = args.get(2).map_or("", |v| v);

            object_with_mut!(log::LOGGER_META_OBJECT_NAME, ModuleMetaManager, meta, {
                meta.set_severity(module, value.into());
            });
        }
        Some("level") | Some("l") => {
            let module = args.get(1).map_or("", |v| v);
            let value = args.get(2).map_or("", |v| v);

            object_with_mut!(log::LOGGER_META_OBJECT_NAME, ModuleMetaManager, meta, {
                meta.set_level(module, value.parse().unwrap_or(0));
            });
        }
        Some("print") | Some("p") => {
            let severity: Severity = args.get(1).map_or("", |v| v).into();
            let level = args.get(2).map_or("", |v| v).parse().unwrap_or(0);

            let mut buf = alloc::string::String::new();

            for arg in args[1..].iter() {
                write!(buf, "{} ", arg).unwrap();
            }

            log!(severity, level, "{}", buf);
        }
        Some(module) => {
            let sev = args.get(1).map_or("", |v| v);
            let lev = args.get(2).map_or("", |v| v);

            if sev.is_empty() && lev.is_empty() {
                // TODO: Get
            } else {
                object_with_mut!(log::LOGGER_META_OBJECT_NAME, ModuleMetaManager, meta, {
                    meta.set_severity(module, sev.into());
                    meta.set_level(module, lev.parse().unwrap_or(0));
                });
            }
        }
    }

    0
}

fn cmd_time(_rt: &mut Runtime, _args: &[&str]) -> i8 {
    info!("tick: {}", rtrs::time::global_tick());
    0
}

pub fn create_shell() -> rtrs::shell::Shell {
    shell!(
        // Builtin commands
        command!("help",     "Prints help",     shell::builtins::cmd_help),
        command!("echo",     "Echo args",       shell::builtins::cmd_echo),
        command!("env",      "Prints env vars", shell::builtins::cmd_env),
        command!("set",      "Set env var val", shell::builtins::cmd_set),
        // Custom commands
        command!("panic",   "Trigger a panic",  cmd_panic),
        command!("crash",   "Trigger a crash",  cmd_crash),
        command!("test",    "Run Tests",        cmd_test),
        command!("obj",     "Object storage",   cmd_obj),
        command!("mem",     "Memory control",   cmd_mem),
        command!("log",     "Logging control",  cmd_log),
        command!("time",    "Get tick",         cmd_time),
    )
}
