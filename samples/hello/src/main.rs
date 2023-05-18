use kernel::*;
use pudding_kernel as kernel;

static mut STACK0: [u8; 4096] = [0; 4096];
static mut STACK1: [u8; 4096] = [0; 4096];

static mut TASK0: Task = Task::new();
static mut TASK1: Task = Task::new();

static mut SEM0: Semaphore = Semaphore::new(0, Order::Priority);
static mut FLG0: Eventflag = Eventflag::new(0);

fn main() {
    println!("Start");
    unsafe {
        kernel::initialize();
        TASK0.create(0, task0, 2, &mut STACK0);
        TASK1.create(0, task1, 1, &mut STACK1);

        TASK0.activate();
        TASK1.activate();
    }
    for _ in 0..10 {
        println!("time_tick");
        kernel::supply_time_tick(1);
    }

    unsafe {
        println!("FLG0.set_flag(4)");
        FLG0.set_flag(4);

        println!("SEM0.signal()");
        SEM0.signal();
    }

    println!("End");
}

fn task0(_exinf: isize) {
    println!("Task0_start");
    kernel::sleep(5);
    let t = unsafe { &mut TASK1 };
    t.activate();

    unsafe {
        println!("SEM0.signal()");
        SEM0.signal();

        println!("FLG0.set_flag(1)");
        FLG0.set_flag(1);
    }

    println!("Task0_end");
}

fn task1(_exinf: isize) {
    println!("Task1_start");
    unsafe {
        println!("Task1: SEM0.wait() start");
        SEM0.wait();
        println!("Task1: SEM0.wait() end");

        println!("Task1: FLG0.wait() start");
        FLG0.wait(0x5, WaitFlagMode::AndWait);
        println!("Task1: FLG0.wait() end");
    }
    println!("Task1_end");
}
