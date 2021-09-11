use core::ptr;

use super::context::*;
use super::*;

type Priority = i32;

pub struct Task {
    pub context: Context,
    pub queue: *mut TaskQueue,
    pub next: *mut Task,
    pub priority: Priority,
    pub task: Option<fn(isize)>,
    pub exinf: isize,
}

#[macro_export]
macro_rules! task_default {
    () => {
        Task {
            context: context_default!(),
            queue: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
            priority: 0,
            task: None,
            exinf: 0,
        }
    };
}

pub struct TaskQueue {
    tail: *mut Task,
}

/*
fn dummy(_: isize) {}

static mut SYSTEM_TASK: Task = Task {
    context : Context { sp: 0 },
    queue: ptr::null_mut(),
    next: ptr::null_mut(),
    priority: 127,
    task: dummy,
    exinf: 0,
    marker: PhantomData,
};
*/

static mut CURRENT_TASK: *mut Task = ptr::null_mut();
static mut READY_QUEUE: TaskQueue = TaskQueue {
    tail: ptr::null_mut(),
};

/*
fn current_task() -> &'static mut Task {
    unsafe { &mut *CURRENT_TASK }
}

fn task_eq(task0: &Task, task1: &Task) -> bool {
    let ptr0 = task0 as *const Task;
    let ptr1 = task1 as *const Task;
    ptr0 == ptr1
}
*/

pub unsafe fn task_switch() {
    let head = READY_QUEUE.front();
    match head {
        None => {
            CURRENT_TASK = ptr::null_mut();
            context_switch_system();
        }
        Some(task) => {
            task.switch();
        }
    };
}

impl Task {
    /// タスク生成
    /*
    pub fn new(exinf: isize, task: fn(isize), priority: Priority, stack: &mut [isize]) -> Self {
        let mut task = Task {
            context: Context::new(),
            queue: ptr::null_mut(),
            next: ptr::null_mut(),
            priority: priority,
            task: task,
            exinf: exinf,
        };

        let task_ptr = &mut task as *mut Task;
        unsafe {
            cpu_lock();
        }
        task.context.create(stack, task_entry, task_ptr as isize);
        unsafe {
            cpu_unlock();
        }
        task
    }*/

    pub fn create(
        &mut self,
        exinf: isize,
        task: fn(isize),
        priority: Priority,
        stack: &mut [isize],
    ) {
        extern "C" fn task_entry(exinf: isize) {
            let task_ptr = exinf as *mut Task;
            let task = unsafe { &mut *task_ptr };
            (task.task.unwrap())(task.exinf);
        }

        self.exinf = exinf;
        self.task = Some(task);
        self.priority = priority;

        let task_ptr = self as *mut Task;
        self.context.create(stack, task_entry, task_ptr as isize);
    }

    pub fn is_current(&self) -> bool {
        self.context.is_current()
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

    pub fn activate(&'static mut self) {
        unsafe {
            cpu_lock();
            READY_QUEUE.insert_priority_order(self);
            task_switch();
            cpu_unlock();
        }
    }


//    let task_tail = unsafe { &mut *self.tail };


}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            tail: ptr::null_mut(),
        }
    }

    /// 優先度順で追加
    pub fn insert_priority_order(&mut self, task: &'static mut Task) {
        // タスクに所属キューを設定
        task.queue = self as *mut TaskQueue;

        // 生ポインタ化
        let task_ptr: *mut Task = task as *mut Task;

        if self.tail == ptr::null_mut() {
            // キューにタスクが無ければ先頭に設定
            task.next = task_ptr;
            self.tail = task_ptr;
        } else {
            // キューが空でないなら挿入位置を探索
            // タスク優先度を取得
            let task_pri = task.priority;

            // 先頭から探索
            let mut prev_ptr = self.tail;
            let mut prev_task = unsafe { &mut *prev_ptr };
            let mut next_ptr = prev_task.next;
            let mut next_task = unsafe { &mut *next_ptr };
            loop {
                // 優先度取り出し
                let next_pri = next_task.priority;

                if next_pri > task_pri {
                    break;
                }

                // 次を探す
                prev_ptr = next_ptr;
                prev_task = next_task;
                next_ptr = prev_task.next;
                next_task = unsafe { &mut *next_ptr };

                // 末尾なら抜ける
                if prev_ptr == self.tail {
                    self.tail = task as *mut Task;
                    break;
                }
            }

            // 挿入
            prev_task.next = task as *mut Task;
            task.next = next_ptr;
        }
    }

    /// FIFO順で追加
    pub fn push_back(&mut self, task: &'static mut Task) {
        // 生ポインタ化
        let task_ptr = task as *mut Task;

        // タスクに所属キューを設定
        task.queue = self as *mut TaskQueue;

        if self.tail == ptr::null_mut() {
            // キューにタスクが無ければ先頭に設定
            task.next = task_ptr;
        } else
        // キューが空でないなら末尾に追加
        {
            let tail_task = unsafe { &mut *self.tail };
            task.next = tail_task.next;
            tail_task.next = task_ptr;
        }
        self.tail = task_ptr;
    }

    /// 先頭を参照
    pub fn front(&mut self) -> Option<&'static mut Task> {
        if self.tail == ptr::null_mut() {
            None
        } else {
            let task_tail = unsafe { &mut *self.tail };
            Some(unsafe { &mut *task_tail.next })
        }
    }

    /// 先頭を取り出し
    pub fn pop_front(&mut self) -> Option<&'static mut Task> {
        if self.tail == ptr::null_mut() {
            None
        } else {
            let task_tail = unsafe { &mut *self.tail };
            let task_head = unsafe { &mut *task_tail.next };
            if self.tail == task_tail.next {
                self.tail = ptr::null_mut();
            } else {
                task_tail.next = task_head.next;
            }
            Some(task_head)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    #[test]
    fn test_task_queue() {
        unsafe {
            let mut que: TaskQueue = TaskQueue::new();
            static mut STACK0: [isize; 256] = [0; 256];
            static mut STACK1: [isize; 256] = [0; 256];
            static mut TASK0: Lazy<Task> =
                Lazy::new(|| Task::new(0, task0, 0, unsafe { &mut STACK0 }));
            static mut TASK1: Lazy<Task> =
                Lazy::new(|| Task::new(0, task1, 1, unsafe { &mut STACK1 }));
            que.push_back(&mut TASK0);
            que.push_back(&mut TASK1);
            let t0 = que.pop_front();
            let t1 = que.pop_front();
            let t2 = que.pop_front();
            assert_eq!(t0.unwrap().priority, 0);
            assert_eq!(t1.unwrap().priority, 1);
            assert_eq!(t2.is_some(), false);
        }
    }

    fn task0(_ext: isize) {}
    fn task1(_ext: isize) {}
}
