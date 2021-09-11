//#![no_std]
//#![no_main]
//#![feature(asm)]

#[macro_use]
extern crate kernel;

use kernel::context::*;
use kernel::task::*;
use core::ptr;

//use once_cell::sync::Lazy;
//use lazy_static::lazy_static;



static mut STACK0: [isize; 256] = [0; 256];
//static mut TASK0: Lazy<Task> = Lazy::new(|| Task::new(0, task0, 0, unsafe { &mut STACK0 }));
static mut TASK0: Task = task_default!();

static mut STACK1: [isize; 256] = [0; 256];
static mut TASK1: Task = task_default!();


/*
lazy_static! {
    static ref STACK0: [isize; 256] = [0; 256];
//    static ref  TASK0: Task = Task::new(0, task0, 0, unsafe { &mut STACK0 });
    
    static ref STACK1: [isize; 256] = [0; 256];
//    static ref  TASK1: Task = Task::new(0, task1, 1, unsafe { &mut STACK1 });
}
*/

fn task0(_ext: isize) {
    println!("Task0");
}

fn task1(_ext: isize) {
    println!("Task1");
}

fn main() {
    println!("Start");
    unsafe {
        
        kernel::initialize();
        TASK0.create(0, task0, 0, &mut STACK0);
        TASK1.create(0, task1, 0, &mut STACK1);

        let pri = TASK0.get_priority();
        println!("{}", pri);

//        let task1: Task = Task::new(0, task1, 1, &mut STACK1);

        TASK0.activate();
        TASK1.activate();
//      TASK0.activate();
//        TASK1.activate();
    }
    println!("End");
}

/*
fn main() {
    unsafe {
        println!("Hello!");

        static mut QUE: Lazy<TaskQueue> = Lazy::new(|| TaskQueue::new());
        static mut STACK0: [isize; 256] = [0; 256];
        static mut STACK1: [isize; 256] = [0; 256];
        static mut TASK0: Lazy<Task> = Lazy::new(|| Task::new(0, task0, 0, unsafe { &mut STACK0 }));
        static mut TASK1: Lazy<Task> = Lazy::new(|| Task::new(0, task1, 1, unsafe { &mut STACK1 }));
        QUE.push_back(&mut TASK0);
        QUE.push_back(&mut TASK1);
        let t0 = QUE.pop_front();
        let t1 = QUE.pop_front();
        let t2 = QUE.pop_front();
        assert_eq!(t0.unwrap().get_priority(), 0);
        assert_eq!(t1.unwrap().get_priority(), 1);
        assert_eq!(t2.is_some(), false);

        TASK0.activate();
    }
}
*/
