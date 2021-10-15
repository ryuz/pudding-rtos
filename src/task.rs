#![allow(dead_code)]

use core::ptr;

use crate::*;
use crate::cpu::*;
use crate::context::*;
use crate::priority_queue::*;
use crate::timeout_queue::*;
use crate::system::*;


pub type TaskQueue = PriorityQueue<Task, Priority>;
pub type TimeQueue = TimeoutQueue<Task, RelTime>;


static mut CURRENT_TASK: *mut Task = ptr::null_mut();
static mut READY_QUEUE: TaskQueue = TaskQueue::new();
static mut TIME_QUEUE:  TimeQueue = TimeQueue::new();


struct Timeout {
    difftim : RelTime,
    next: *mut Task,
    prev: *mut Task,
}

impl Timeout {
    const fn new() -> Self {
        Timeout{
            difftim:0,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
         }
    }
}

// Task control block
// static初期化の為に泣く泣くすべてpubにする
pub struct Task {
    context: crate::context::Context,
    queue: *mut TaskQueue,
    next: *mut Task,
    priority: Priority,
    task: Option<fn(isize)>,
    exinf: isize,
    actcnt: ActCount,
    timeout : Timeout
}


impl Task {
    /// インスタンス生成
    pub const fn new() -> Self {
        Task {
            context: Context::new(),
            queue: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
            priority: 0,
            task: None,
            exinf: 0,
            actcnt: 0,
            timeout: Timeout::new()
        }
    }

    /// タスク生成
    pub fn create(
        &mut self,
        exinf: isize,
        task: fn(isize),
        priority: Priority,
        stack: &mut [u8],
    ) {
        extern "C" fn task_entry(exinf: isize) {
            unsafe {
                let task_ptr = exinf as *mut Task;
                let task = &mut *task_ptr;
                loop {
                    while task.actcnt > 0 {
                        task.actcnt -= 1;
                        cpu_unlock();
                        (task.task.unwrap())(task.exinf);
                        cpu_lock();
                    }
                    task.remove_from_queue();
                    task_switch()
                }
            }
        }

        self.exinf = exinf;
        self.task = Some(task);
        self.priority = priority;

        let task_ptr = self as *mut Task;
        self.context.create(stack, task_entry, task_ptr as isize);
    }

    pub(crate) fn remove_from_queue(&mut self) {
        if self.queue != ptr::null_mut() {
            let que = unsafe { &mut *self.queue };
            que.remove(self);
            self.queue = ptr::null_mut();
        }
    }

    
    pub(crate) fn remove_from_timeout(&mut self) {
        if self.timeout.prev != ptr::null_mut() {
            unsafe{
                TIME_QUEUE.remove(self);
            }
        }
    }
    

    pub(crate) fn attach_ready_queue(&mut self) {
        unsafe {
            READY_QUEUE.insert_priority_order(self);
        }
    }

    /// タスクスイッチ
    unsafe fn switch(&mut self) {
        let task_ptr = self as *mut Task;
        CURRENT_TASK = task_ptr;
        //      CURRENT_TASK = self as *mut Task;
        self.context.switch();
    }

    pub fn get_priority(&self) -> Priority {
        self.priority
    }

    pub fn activate(&mut self) {
        unsafe {
            let _sc = SystemCall::new();
            self.actcnt += 1;
            if self.queue == ptr::null_mut() {
                //              READY_QUEUE.insert_priority_order(self);
                self.attach_ready_queue();
                set_dispatch_reserve_flag();
            }
        }
    }
}

impl PriorityObject<Task, Priority> for Task {
    fn next(&self) -> *mut Task {
        self.next
    }
    fn set_next(&mut self, next: *mut Task) {
        self.next = next;
    }
    fn priority(&self) -> Priority {
        self.priority
    }
    fn queue(&self) -> *mut TaskQueue {
        self.queue
    }

    fn set_queue(&mut self, que: *mut TaskQueue) {
        self.queue = que;
    }

    fn queue_dropped(&mut self) {}
}


impl TimeoutObject<Task, RelTime> for Task {
    fn difftim(&self) -> RelTime{
        self.timeout.difftim
    }
    fn set_difftim(&mut self, difftim: RelTime) {
        self.timeout.difftim = difftim;
    }

    fn next(&self) -> *mut Task {
        self.timeout.next
    }
    fn set_next(&mut self, next: *mut Task) {
        self.timeout.next = next;
    }

    fn prev(&self) -> *mut Task {
        self.timeout.prev
    }        
    fn set_prev(&mut self, prev: *mut Task) {
        self.timeout.prev = prev;
    }

    fn timeout(&mut self) {}

    fn queue_dropped(&mut self) {}
}


pub(crate) unsafe fn detach_ready_queue() -> Option<&'static mut Task> {
    set_dispatch_reserve_flag();
    READY_QUEUE.pop_front()
}

pub(crate) unsafe fn task_switch() {
    let head = READY_QUEUE.front();
    match head {
        None => {
            CURRENT_TASK = ptr::null_mut();
            context_switch_to_system();
        }
        Some(task) => {
            task.switch();
        }
    };
}

