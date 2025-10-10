use core::alloc::Layout;
use core::str::SplitWhitespace;
use core::fmt::Write;

use rtrs::log::meta::ModuleMetaManager;
use rtrs::object::STORAGE;
use rtrs::{
    println,
    command,
    commands,
    object_with,
    object_with_mut,
    bit_set,
    bit_if,
    log,
    logger,
};
use rtrs::log::Severity;
// use crate::bsp;

logger!("SHELL");

fn cmd_panic(mut args: SplitWhitespace) -> i8 {
    panic!("{}", args.next().unwrap_or("Manual panic"));
}

fn cmd_crash(_args: SplitWhitespace) -> i8 {
    // bsp::Board::crash();
    0
}

fn cmd_test(mut args: SplitWhitespace) -> i8 {
    fn help() {
        println!("test [help|all|task|task-irq|task-nest|task-obj|logger|hexdump|box|heap]")
    }

    enum Test {
        Task,
        TaskIrq,
        TaskNest,
        TaskObj,
        Logger,
        Hexdump,
        Box,
        Heap,
    }

    let mut tests: u32 = 0;

    while let Some(arg) = args.next() {
        match arg {
            "all" => tests = 0xFF,
            "task" => bit_set!(tests, Test::Task),
            "task-irq" => bit_set!(tests, Test::TaskIrq),
            "task-nest" => bit_set!(tests, Test::TaskNest),
            "task-obj" => bit_set!(tests, Test::TaskObj),
            "logger" => bit_set!(tests, Test::Logger),
            "hexdump" => bit_set!(tests, Test::Hexdump),
            "box" => bit_set!(tests, Test::Box),
            "heap" => bit_set!(tests, Test::Heap),
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

    bit_if!(tests, Test::Task,     crate::test_tasks());
    bit_if!(tests, Test::TaskIrq,  crate::test_irq_tasks());
    bit_if!(tests, Test::TaskNest, crate::test_nested_tasks());
    bit_if!(tests, Test::TaskObj,  crate::test_task_object());
    bit_if!(tests, Test::Logger,   crate::test_logger());
    bit_if!(tests, Test::Hexdump,  crate::test_hexdump());
    bit_if!(tests, Test::Box,      crate::test_box());
    bit_if!(tests, Test::Heap,     crate::test_heap());

    0
}

fn cmd_obj(mut args: SplitWhitespace) -> i8 {
    match args.next() {
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

fn cmd_mem(mut args: SplitWhitespace) -> i8 {
    fn help() {
        println!("mem [help|info|alloc|free]");
    }

    match args.next() {
        Some("info") => {
            crate::GLOBAL_HEAP.dump();
        }
        Some("alloc") => {
            let size = match args.next() {
                Some(arg) => arg.parse::<usize>().unwrap_or(0),
                None => 0
            };
            let ptr = crate::GLOBAL_HEAP.allocate(unsafe { Layout::from_size_align_unchecked(size, 4) });
            println!("alloc({}): {:?}", size, ptr);
        }
        Some("free") => {
            let ptr = match args.next() {
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

fn cmd_log(mut args: SplitWhitespace) -> i8 {
    match args.next() {
        Some("help") => {
            println!("log help - Shows this message");
            println!("log / log list - Shows all modules");
            println!("log severity|s MOD VAL - Set max severity for module");
            println!("log level|l MOD VAL - Set max level for module");
            println!("log MOD SEV LVL - Set max severity & level for module");
            println!("log print|p SEV LVL ... - Print log message");
        }
        Some("list") | None => {
            object_with!(rtrs::log::LOGGER_META_OBJECT_NAME, ModuleMetaManager, meta, {
                for (name, meta) in meta.iter() {
                    println!("{}: >{} <{}", name, meta.severity, meta.level);
                }
            });
        }
        Some("severity") | Some("s") => {
            let module = args.next().unwrap_or("");
            let value = args.next().unwrap_or("");

            object_with_mut!(rtrs::log::LOGGER_META_OBJECT_NAME, ModuleMetaManager, meta, {
                meta.set_severity(module, value.into());
            });
        }
        Some("level") | Some("l") => {
            let module = args.next().unwrap_or("");
            let value = args.next().unwrap_or("");

            object_with_mut!(rtrs::log::LOGGER_META_OBJECT_NAME, ModuleMetaManager, meta, {
                meta.set_level(module, value.parse().unwrap_or(0));
            });
        }
        Some("print") | Some("p") => {
            let severity: Severity = args.next().unwrap_or("").into();
            let level = args.next().unwrap_or("").parse().unwrap_or(0);

            let mut buf = alloc::string::String::new();

            loop {
                match args.next() {
                    Some(arg) => {
                        write!(buf, "{} ", arg).unwrap();
                    }
                    None => break,
                }
            }

            log!(severity, level, "{}", buf);
        }
        Some(module) => {
            let sev = args.next().unwrap_or("");
            let lev = args.next().unwrap_or("");

            if sev.is_empty() && lev.is_empty() {
                // TODO: Get
            } else {
                object_with_mut!(rtrs::log::LOGGER_META_OBJECT_NAME, ModuleMetaManager, meta, {
                    meta.set_severity(module, sev.into());
                    meta.set_level(module, lev.parse().unwrap_or(0));
                });
            }
        }
    }

    0
}

pub fn create_shell() -> rtrs::shell::Shell {
    rtrs::shell::Shell::new(commands!(
        command!("panic",   "Trigger a panic", cmd_panic),
        command!("crash",   "Trigger a crash", cmd_crash),
        command!("test",    "Run Tests",       cmd_test),
        command!("obj",     "Object storage",  cmd_obj),
        command!("mem",     "Memory control",  cmd_mem),
        command!("log",     "Logging control", cmd_log),
    ))
}
