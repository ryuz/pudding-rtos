

use crate::system::*;
use crate::task::*;

pub type SemCount = i32;


// Task control block
// static初期化の為に泣く泣くすべてpubにする
pub struct Semaphore {
    pub queue: TaskQueue,
    pub count: SemCount,
}

// static初期化時に中身を知らなくてよいようにマクロで補助
#[macro_export]
macro_rules! semaphore_default {
    ($x:expr) => {
        Semaphore {
            queue: task_queue_default!(),
            count: $x,
        }
    };
    () => {
        Semaphore {
            queue: task_queue_default!(),
            count: 0,
        }
    };
}

impl Semaphore {
    pub fn wait(&mut self) {
        unsafe {
            let _sc = SystemCall::new();
            if self.count > 0 {
                self.count -= 1;
            }
            else {
                let task = detach_ready_queue().unwrap();
                self.queue.insert_priority_order(task);
                set_dispatch_reserve_flag();
            }
        }
    }

    pub fn signal(&mut self) {
        unsafe {
            let _sc = SystemCall::new();
            let head = self.queue.pop_front();
            match head {
                None => {
                    self.count += 1;
                }
                Some(task) => {
                    task.attach_ready_queue();
                    set_dispatch_reserve_flag();
                }
            };
        }
    }
}

