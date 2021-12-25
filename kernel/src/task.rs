#![allow(dead_code)]

use core::ptr::NonNull;

use crate::priority_queue::*;
use crate::timeout_queue::*;
use crate::*;

pub(crate) type TaskQueue = PriorityQueue<Task, Priority>;
type TimeQueue = TimeoutQueue<Task, RelativeTime>;

// ---------------------------------
//  Ready Queue
// ---------------------------------

/*
static mut CURRENT_TASK: *mut Task = ptr::null_mut();

pub(crate) fn current_task() -> Option<&'static mut Task>  {
    unsafe {
        let task = CURRENT_TASK;
        if task == ptr::null_mut() {None} else {Some(&mut *task)}
    }
}
*/

pub(crate) fn current_task() -> Option<&'static mut Task> {
    ready_queue::front() // レディーキューの先頭を実行中タスクとする
}

pub(crate) fn detach_current_task() -> Option<&'static mut Task> {
    ready_queue::pop_front() // レディーキューの先頭を実行中タスクとする
}

mod ready_queue {
    use super::*;

    static mut READY_QUEUE: TaskQueue = TaskQueue::new();

    pub(crate) fn front() -> Option<&'static mut Task> {
        unsafe { READY_QUEUE.front() }
    }

    pub(crate) fn pop_front() -> Option<&'static mut Task> {
        unsafe { READY_QUEUE.pop_front() }
    }

    pub(crate) fn attach(task: &mut Task) {
        unsafe {
            READY_QUEUE.insert_priority_order(task);
        }
    }

    pub(crate) fn detach(task: &mut Task) {
        unsafe {
            READY_QUEUE.remove(task);
        }
    }

    pub(crate) fn is_attached(task: &Task) -> bool {
        //        if task.queue.is_none() {false} else {
        //        task.queue.unwrap().as_ptr() == unsafe{&mut READY_QUEUE as *mut TaskQueue}}

        task.queue == Some(unsafe { NonNull::new_unchecked(&mut READY_QUEUE as *mut TaskQueue) })
    }
}

// ---------------------------------
//  Timeout Queue
// ---------------------------------

mod timeout_queue {
    use super::*;

    static mut TIME_QUEUE: TimeQueue = TimeQueue::new();

    pub(crate) fn sig_tim(tick: RelativeTime) {
        unsafe {
            TIME_QUEUE.sig_tim(tick);
        }
    }

    pub(crate) fn attach(task: &mut Task, time: RelativeTime) {
        unsafe {
            TIME_QUEUE.add(task, time);
        }
    }

    pub(crate) fn detach(task: &mut Task) {
        unsafe {
            TIME_QUEUE.remove(task);
        }
    }

    pub(crate) fn is_attached(task: &Task) -> bool {
        !task.timeout.prev.is_none()
    }
}

pub fn supply_time_tick_for_timeout(tick: RelativeTime) {
    timeout_queue::sig_tim(tick);
}

// ---------------------------------
//  Task control block
// ---------------------------------

struct Timeout {
    difftim: RelativeTime,
    next: Option<NonNull<Task>>,
    prev: Option<NonNull<Task>>,
}

impl Timeout {
    const fn new() -> Self {
        Timeout {
            difftim: 0,
            next: None,
            prev: None,
        }
    }
}

// Task control block
pub struct Task {
    context: crate::context::Context,
    queue: Option<NonNull<TaskQueue>>,
    next: Option<NonNull<Task>>,
    priority: Priority,
    task: Option<fn(isize)>,
    exinf: isize,
    actcnt: ActivateCount,
    timeout: Timeout,
    result: Result<(), Error>,
}

impl Task {
    /// インスタンス生成
    pub const fn new() -> Self {
        Task {
            context: Context::new(),
            queue: None,
            next: None,
            priority: 0,
            task: None,
            exinf: 0,
            actcnt: 0,
            timeout: Timeout::new(),
            result: Ok(()),
        }
    }

    /// タスク生成
    pub fn create(&mut self, exinf: isize, task: fn(isize), priority: Priority, stack: &mut [u8]) {
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
                    task.detach_from_queue();
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

    pub(crate) fn result(&self) -> Result<(), Error> {
        self.result
    }

    pub(crate) fn set_result(&mut self, result: Result<(), Error>) {
        self.result = result;
    }

    pub(crate) fn is_attached_to_ready_queue(&self) -> bool {
        ready_queue::is_attached(self)
    }

    pub(crate) fn is_attached_to_timeout(&self) -> bool {
        timeout_queue::is_attached(self)
    }

    pub(crate) fn is_attached_to_any_queue(&self) -> bool {
        self.queue != None
    }

    pub(crate) fn is_attached_to_wait_queue(&self) -> bool {
        !self.is_attached_to_ready_queue() && self.is_attached_to_any_queue()
    }

    pub(crate) fn attach_to_ready_queue(&mut self) {
        debug_assert!(!self.is_attached_to_timeout());
        debug_assert!(!self.is_attached_to_any_queue());
        ready_queue::attach(self);
    }

    pub(crate) fn attach_to_queue(&mut self, que: &mut TaskQueue, order: Order) {
        debug_assert!(!self.is_attached_to_any_queue());
        match order {
            Order::Priority => {
                que.insert_priority_order(self);
            }
            Order::Fifo => {
                que.push_back(self);
            }
        }
    }

    pub(crate) fn detach_from_queue(&mut self) {
        match self.queue {
            Some(mut que) => unsafe {
                que.as_mut().remove(self);
            },
            _ => {}
        }
    }

    pub(crate) fn attach_to_timeout(&mut self, time: RelativeTime) {
        debug_assert_eq!(self.timeout.prev, None);
        timeout_queue::attach(self, time);
    }

    pub(crate) fn detach_from_timeout(&mut self) {
        if !self.timeout.prev.is_none() {
            timeout_queue::detach(self);
        }
    }

    // タスクスイッチ
    unsafe fn switch(&mut self) {
        self.context.switch();
    }

    pub fn priority(&self) -> Priority {
        self.priority
    }

    pub fn activate(&mut self) {
        let _sc = SystemCall::new();
        self.actcnt += 1;
        if self.queue == None {
            self.attach_to_ready_queue();
            set_dispatch_reserve_flag();
        }
    }
}

impl PriorityObject<Task, Priority> for Task {
    fn next(&self) -> Option<NonNull<Task>> {
        self.next
    }
    fn set_next(&mut self, next: Option<NonNull<Task>>) {
        self.next = next;
    }
    fn priority(&self) -> Priority {
        self.priority
    }
    fn queue(&self) -> Option<NonNull<TaskQueue>> {
        self.queue
    }

    fn set_queue(&mut self, que: Option<NonNull<TaskQueue>>) {
        self.queue = que;
    }

    fn queue_dropped(&mut self) {}
}

impl TimeoutObject<Task, RelativeTime> for Task {
    fn difftim(&self) -> RelativeTime {
        self.timeout.difftim
    }
    fn set_difftim(&mut self, difftim: RelativeTime) {
        self.timeout.difftim = difftim;
    }

    fn next(&self) -> Option<NonNull<Task>> {
        self.timeout.next
    }
    fn set_next(&mut self, next: Option<NonNull<Task>>) {
        self.timeout.next = next;
    }

    fn prev(&self) -> Option<NonNull<Task>> {
        self.timeout.prev
    }
    fn set_prev(&mut self, prev: Option<NonNull<Task>>) {
        self.timeout.prev = prev;
    }

    fn timeout(&mut self) {
        self.detach_from_queue();
        self.attach_to_ready_queue();
        self.result = Err(Error::Timeout);
        set_dispatch_reserve_flag();
    }

    fn queue_dropped(&mut self) {}
}

pub(crate) unsafe fn task_switch() {
    let head = ready_queue::front();
    match head {
        None => {
            // CURRENT_TASK = None;
            context_switch_to_system();
        }
        Some(task) => {
            task.switch();
        }
    };
}
