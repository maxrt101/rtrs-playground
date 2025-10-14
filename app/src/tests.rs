use core::fmt::Write;
use core::alloc::Layout;
use core::any::Any;

use rtrs::object::Object;
use rtrs::time::TimeProvider;
use rtrs::task::{Event, ExecutionContext, Task};
use rtrs::log::console::CONSOLE_OBJECT_NAME;

use rtrs::{
    object_insert,
    object_remove,
    object_with,
    object_with_mut,
    print,
    println,
    task_sleep,
    task_yield,
    tasks_await,
    tasks_run,
    tasks_run_with_ctx
};

use rtrs::{
    logger,
    trace,
    info,
    warn,
    error,
    fatal
};

extern crate alloc;

use alloc::boxed::Box;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::task::Poll;

logger!("TEST");

async fn task1() {
    for i in 0..2 {
        task_yield!();
        println!("Task1:1 {}", i);
        task_sleep!(10);
        println!("Task1:2 {}", i);
    }
}

async fn task2() {
    for i in 0..2 {
        println!("Task2:1 {}", i);
        task_sleep!(10);
        println!("Task2:2 {}", i);
    }
}

async fn task3() {
    for i in 0..2 {
        println!("Task3:1 {}", i);
        task_sleep!(10);
        println!("Task3:2 {}", i);
    }
}

pub(crate) fn test_tasks() {
    let mut t1 = Task::new(task1());
    let mut t2 = Task::new(task2());
    let mut t3 = Task::new(task3());

    tasks_run!(t1, t2, t3);
}

pub(crate) static SYSTICK_EVENT: Event = Event::new();

async fn task_irq1(_ctx: &ExecutionContext) {
    let mut wakeups: u32 = 0;

    loop {
        wakeups += 1;

        if wakeups % 1000 == 0 {
            println!("task1: {}", wakeups);
        }

        (&SYSTICK_EVENT).await;
    }
}

async fn task_irq2(_ctx: &ExecutionContext) {
    let mut last_time: u32 = rtrs::time::global_tick();

    loop {
        if rtrs::time::global_tick() - last_time > 1000 {
            last_time = rtrs::time::global_tick();

            println!("task2: {}", rtrs::time::global_tick());
        }

        task_sleep!(1);
    }
}

async fn task_monitor(ctx: &ExecutionContext) {
    loop {
        if let Some(_) = object_with!(CONSOLE_OBJECT_NAME, rtrs::tty::Tty, tty, tty.read()) {
            ctx.set_should_run(false);
        }

        task_sleep!(1);
    }
}

pub(crate) fn test_irq_tasks() {
    let ctx = ExecutionContext::new();

    let mut t1 = Task::new(task_irq1(&ctx));
    let mut t2 = Task::new(task_irq2(&ctx));
    let mut t3 = Task::new(task_monitor(&ctx));

    info!("Press any key to stop");

    tasks_run_with_ctx!(ctx, t1, t2, t3);

    SYSTICK_EVENT.clear();
}

async fn worker(name: &'static str, max_cycles: u32) {
    let mut last_time: u32 = rtrs::time::global_tick();
    let mut cycles: u32 = 0;

    loop {
        if rtrs::time::global_tick() - last_time > 500 {
            cycles += 1;

            last_time = rtrs::time::global_tick();

            println!("{}: {} ({}/{})", name, rtrs::time::global_tick(), cycles, max_cycles);
        }

        if cycles >= max_cycles {
            break;
        }

        task_sleep!(1);
    }
}

async fn task_nested() {
    let mut worker1 = Task::new(worker("worker1", 5));
    let mut worker2 = Task::new(worker("worker2", 5));

    tasks_await!(worker1, worker2);
}

pub(crate) fn test_nested_tasks() {
    let mut task1 = Task::new(worker("task1  ", 8));
    let mut task2 = Task::new(task_nested());

    tasks_run!(task1, task2);
}

pub(crate) fn test_logger() {
    let a = 42;

    // info!(logger: ALT_LOGGER, "Test");
    // info!(level: 4, "Test");

    trace!(level: 1, "Test trace");
    info!("Test info {}", 1);
    warn!("Test warn");
    error!("Test error {}", a);
    fatal!("Test fatal");
}

pub(crate) fn test_hexdump() {
    println!("{}", rtrs::colored!(rtrs::ANSI_COLOR_FG_GREEN, "Test123"));
    println!("{}", rtrs::colored_fmt!(rtrs::ANSI_TEXT_INVERSE, "Test {} {}", 42, 69));

    // println!("{}", rtrs::multicolored!((rtrs::ANSI_COLOR_FG_GREEN, rtrs::ANSI_TEXT_INVERSE), "Test123"));

    let arr = [0, 1, '!' as u8, 3, 'c' as u8, 5, 6, 'a' as u8, 8, 9, 'f' as u8, 'u' as u8, 'c' as u8, 'k' as u8, 0, 42, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0xff, 0xaa, 0x55];

    print!("{}", rtrs::util::Hexdump::from(&arr).default_color());

}

trait ParentTrait {
    fn beep(&self);
}

struct ChildObject1 {
    id: usize,
}

impl ChildObject1 {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

impl ParentTrait for ChildObject1 {
    fn beep(&self) {
        println!("ChildObject1::beep {}", self.id);
    }
}

pub(crate) fn test_box() {
    // Test Box with casting to dyn Trait (T: Trait)
    let b = Box::new(ChildObject1::new(42));
    (*b).beep();

    let b: Box<dyn ParentTrait> = b;
    (*b).beep();


    // Another test with traits, but with both down and up casting
    let b2 = Box::new(TimeProvider::new());

    for _ in 0..42 {
        (*b2).increment();
    }

    println!("Box<TimeProvider>::now {}", (*b2).now());

    let mut b2: Box<dyn Object> = b2;

    println!("(Box<dyn Object> as TimeProvider)::now {}", unsafe { &*(b2.deref_mut() as *mut dyn Object as *mut TimeProvider) }.now());

    let b2: Box<dyn Any> = b2;
    let b2: Box<TimeProvider> = b2.downcast().unwrap();

    println!("Box<TimeProvider>::now {}", (*b2).now());


    // Maybe uninit test
    let mut b3 = Box::<u32>::new_uninit();
    // TODO: What happens if uninit is accessed?
    b3.write(42);

    println!("Box<MaybeUninit<<T>> -> Box<T>: {}", unsafe { b3.assume_init_ref() });


    // Macros test
    let b4 = Box::new(TimeProvider::new());

    for _ in 0..69 {
        (*b4).increment();
    }

    println!("Box<TimeProvider>::now {}", (*b4).now());

    let mut b4: Box<dyn Object> = b4;

    println!("(Box<dyn Object> as TimeProvider)::now {}", unsafe { &*(b4.deref_mut() as *mut dyn Object as *mut TimeProvider) }.now());

    let b4: Box<dyn Any> = b4;
    let b4: Box<TimeProvider> = b4.downcast().unwrap();

    println!("Box<TimeProvider>::now {}", (*b4).now());
}

pub(crate) fn test_heap() {
    unsafe {
        println!("GLOBAL_HEAP: {:?} buf={:?}", &crate::GLOBAL_HEAP as *const _, crate::GLOBAL_HEAP.buffer().as_ptr());
    }

    crate::GLOBAL_HEAP.dump();

    println!();

    let ptr1 = crate::GLOBAL_HEAP.allocate(unsafe { Layout::from_size_align_unchecked(10, 4) });

    crate::GLOBAL_HEAP.dump();

    let ptr2 = crate::GLOBAL_HEAP.allocate(unsafe { Layout::from_size_align_unchecked(24, 4) });

    println!("ptr1: {:?}, ptr2: {:?}", ptr1, ptr2);

    crate::GLOBAL_HEAP.dump();

    unsafe {
        *(ptr1 as *mut u32) = 0xcafebabe;
        *(ptr2 as *mut u32) = 0xdeadbeef;

        println!("ptr1: {:?} u32={:x}", ptr1, *(ptr1 as *mut u32));
        println!("ptr2: {:?} u32={:x}", ptr2, *(ptr2 as *mut u32));
    }

    crate::GLOBAL_HEAP.free(ptr1);
    crate::GLOBAL_HEAP.free(ptr2);

    crate::GLOBAL_HEAP.dump();

    let ptr3 = crate::GLOBAL_HEAP.allocate(unsafe { Layout::from_size_align_unchecked(14, 4) });

    crate::GLOBAL_HEAP.dump();

    unsafe {
        *(ptr3 as *mut u32) = 0xcafebabe;

        println!("ptr3: {:?} u32={:x}", ptr3, *(ptr3 as *mut u32));
    }

    crate::GLOBAL_HEAP.free(ptr3);

    crate::GLOBAL_HEAP.dump();
}


trait TaskObject: Object {
    fn poll(&mut self) -> Poll<u8>;
}

struct Task1 {
    task: Task<'static, u8>,
}

impl Task1 {
    fn new(task: Task<'static, u8>) -> Self {
        Self { task }
    }
}

impl Object for Task1 {}

impl TaskObject for Task1 {
    fn poll(&mut self) -> Poll<u8> {
        self.task.poll()
    }
}

static TEST_OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

async fn test_object_task1() -> u8 {
    let mut counter = usize::MAX;

    while counter > 0 {
        counter = TEST_OBJECT_COUNTER.load(Ordering::Acquire);
        trace!("task1: {}", counter);
        TEST_OBJECT_COUNTER.store(counter - 1, Ordering::SeqCst);
        task_yield!();
    }

    0
}

pub(crate) fn test_task_object() {
    fn print_objects() {
        print!("Objects: ");
        for obj in rtrs::object::STORAGE.lock().keys() {
            print!("{} ", obj);
        }
        println!();
    }

    TEST_OBJECT_COUNTER.store(5, Ordering::Release);
    object_insert!("task1", Task1::new(Task::new(test_object_task1())));

    print_objects();

    loop {
        match object_with_mut!("task1", Task1, t, t.poll()) {
            Poll::Ready(res) => {
                info!("task1 finished with result: {}", res);
                break;
            }
            Poll::Pending => {
                trace!("task1 running...");
            }
        }
    }

    object_remove!("task1");

    print_objects();
}

pub(crate) fn test_task_sched() {
    let mut sched = rtrs::task::sched::Scheduler::new();

    sched.attach(Task::new(worker("worker1", 4)));
    sched.attach(Task::new(worker("worker2", 5)));
    sched.attach(Task::new(worker("worker3", 3)));

    sched.run_to_completion();
}


