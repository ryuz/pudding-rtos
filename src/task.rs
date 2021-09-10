use core::marker::PhantomData;
use core::ptr;

use super::context::*;

type Priority = i32;

pub struct TaskControlBlock<'a> {
    context: ContextControlBlock,
    queue: *mut TaskQueue<'a>,
    next: *mut TaskControlBlock<'a>,
    priority: Priority,
    task: fn(isize),
    exinf: isize,
    marker: PhantomData<&'a TaskControlBlock<'a>>,
}

pub struct TaskQueue<'a> {
    tail: *mut TaskControlBlock<'a>,
}

extern "C" fn task_entry(exinf: isize)
{
    let task_ptr = exinf as *mut TaskControlBlock;
    let task = unsafe { &mut *task_ptr };
    (task.task)(task.exinf);
}

impl<'a> TaskControlBlock<'a> {
    pub fn new(exinf: isize, task: fn(isize), priority: Priority, stack: &mut [isize]) -> Self {
        let mut task = TaskControlBlock {
            context: ContextControlBlock::new(),
            queue: ptr::null_mut(),
            next: ptr::null_mut(),
            priority: priority,
            task: task,
            exinf: exinf,
            marker: PhantomData,
        };
        let task_ptr = &mut task as *mut TaskControlBlock;
        task.context.create(stack, task_entry, task_ptr as isize);
        task
    }

    pub fn get_priority(&self) -> Priority
    {
        self.priority
    }
}


impl<'a> TaskQueue<'a> {
    pub fn new() -> Self {
        TaskQueue {
            tail: ptr::null_mut(),
        }
    }

    /// 優先度順で追加
    pub fn insert_priority_order(&mut self, task: &'a mut TaskControlBlock<'a>) {
        // タスクに所属キューを設定
        task.queue = self as *mut TaskQueue<'a>;

        // 生ポインタ化
        let task_ptr: *mut TaskControlBlock<'a> = task as *mut TaskControlBlock<'a>;

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
                    self.tail = task as *mut TaskControlBlock;
                    break;
                }
            }

            // 挿入
            prev_task.next = task as *mut TaskControlBlock;
            task.next = next_ptr;
        }
    }

    /// FIFO順で追加
    pub fn push_back(&mut self, task: &'a mut TaskControlBlock<'a>) {
        // 生ポインタ化
        let task_ptr = task as *mut TaskControlBlock;

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
    pub fn front(&mut self) -> Option<&'a mut TaskControlBlock<'a>> {
        if self.tail == ptr::null_mut() {
            None
        } else {
            let task_tail = unsafe { &mut *self.tail };
            Some(unsafe { &mut *task_tail.next })
        }
    }

    /// 先頭を取り出し
    pub fn pop_front(&mut self) -> Option<&'a mut TaskControlBlock<'a>> {
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

    #[test]
    fn test_task_queue() {
        unsafe {
            let mut que: TaskQueue = TaskQueue::new();
            static mut STACK0: [isize; 256] = [0; 256];
            static mut STACK1: [isize; 256] = [0; 256];
            let mut task0: TaskControlBlock = TaskControlBlock::new(0, task0,0, &mut STACK0);
            let mut task1: TaskControlBlock = TaskControlBlock::new(0, task1,1, &mut STACK1);
            que.push_back(&mut task0);
            que.push_back(&mut task1);
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
