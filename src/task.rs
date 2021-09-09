


use core::ptr;
use core::marker::PhantomData;


pub struct TaskControlBlock<'a> {
    queue: *mut TaskQueue<'a>,
    next: *mut TaskControlBlock<'a>,
    priority: i32,
    marker: PhantomData<&'a TaskControlBlock<'a>>,
}

pub struct TaskQueue<'a> {
    tail: *mut TaskControlBlock<'a>,
}

impl<'a> TaskControlBlock<'a> {
    pub fn new() -> Self {
        TaskControlBlock {
            queue: ptr::null_mut(), 
            next: ptr::null_mut(),
            priority: 0,
            marker: PhantomData,
        }
    }
}

impl<'a> TaskQueue<'a> {
    pub fn new() -> Self {
        TaskQueue {
            tail: ptr::null_mut(),
        }
    }

    /// 優先度順で追加
    pub fn insert_priority_order(&mut self, task: &'a mut TaskControlBlock<'a>)
    {
        // タスクに所属キューを設定
        task.queue = self as *mut TaskQueue<'a>;

        // 生ポインタ化
        let task_ptr:*mut TaskControlBlock<'a> = task as *mut TaskControlBlock<'a>;

        if self.tail == ptr::null_mut() { // キューにタスクが無ければ先頭に設定
            task.next = task_ptr;
            self.tail = task_ptr;
        }
        else { // キューが空でないなら挿入位置を探索
            // タスク優先度を取得
            let task_pri = task.priority;
            
            // 先頭から探索
            let mut prev_ptr  = self.tail;
            let mut prev_task = unsafe{ &mut *prev_ptr };
            let mut next_ptr  = prev_task.next;
            let mut next_task = unsafe{ &mut *next_ptr };
            loop {
                // 優先度取り出し
                let next_pri = next_task.priority;
                
                if next_pri > task_pri {
                    break;
                }
                
                // 次を探す
                prev_ptr  = next_ptr;
                prev_task = next_task;
                next_ptr  = prev_task.next;
                next_task = unsafe{ &mut *next_ptr };
                
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

        if self.tail == ptr::null_mut() { // キューにタスクが無ければ先頭に設定
            task.next = task_ptr;
        }
        else // キューが空でないなら末尾に追加
        {
            let tail_task = unsafe{ &mut *self.tail };
            task.next = tail_task.next;
            tail_task.next = task_ptr;
        }
        self.tail = task_ptr;
    }

    pub fn front(&mut self) -> Option<&'a mut TaskControlBlock<'a>>
    {
        if self.tail == ptr::null_mut() {
            None
        }
        else {
            let task_tail = unsafe{&mut *self.tail};
            Some(unsafe{&mut *task_tail.next})
        }
    }

    pub fn pop_front(&mut self) -> Option<&'a mut TaskControlBlock<'a>>
    {
        if self.tail == ptr::null_mut() {
            None
        }
        else {
            let task_tail = unsafe{&mut *self.tail};
            let task_head = unsafe{&mut *task_tail.next};
            if self.tail == task_tail.next {
                self.tail = ptr::null_mut();
            }
            else {
                task_tail.next = task_head.next;
            }
            Some(task_head)
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    static mut QUE:TaskQueue = TaskQueue { tail:ptr::null_mut() };
    static mut TASK0:TaskControlBlock = TaskControlBlock { queue:ptr::null_mut(), next:ptr::null_mut(), priority:0, marker:PhantomData, };
    static mut TASK1:TaskControlBlock = TaskControlBlock { queue:ptr::null_mut(), next:ptr::null_mut(), priority:1, marker:PhantomData, };

    #[test]
    fn task_queue_test1() {
        let value = 10;
        assert_eq!(10, value);

        unsafe {
            QUE.push_back(&mut TASK0);
            QUE.push_back(&mut TASK1);
            let t0 = QUE.pop_front();
            let t1 = QUE.pop_front();
            assert_eq!(t0.unwrap().priority, 0);
            assert_eq!(t1.unwrap().priority, 1);
        }
    }

    #[test]
    fn task_queue_test2() {
        let mut que:TaskQueue = TaskQueue { tail:ptr::null_mut() };
        let mut task0:TaskControlBlock = TaskControlBlock { queue:ptr::null_mut(), next:ptr::null_mut(), priority:0, marker:PhantomData };
        let mut task1:TaskControlBlock = TaskControlBlock { queue:ptr::null_mut(), next:ptr::null_mut(), priority:1, marker:PhantomData };
        que.push_back(&mut task0);
        que.push_back(&mut task1);
        let t0 = que.pop_front().unwrap();
        let t1 = que.pop_front().unwrap();
        assert_eq!(t0.priority, 0);
        assert_eq!(t1.priority, 1);
    }
}

