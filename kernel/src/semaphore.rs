use crate::system::*;
use crate::*;

// Semephore control block
pub struct Semaphore {
    queue: TaskQueue,
    order: Order,
    count: SemCount,
}

impl Semaphore {
    pub const fn new(init_count: SemCount, order: Order) -> Self {
        Semaphore {
            queue: TaskQueue::new(),
            order: order,
            count: init_count,
        }
    }

    pub fn signal(&mut self) {
        let _sc = SystemCall::new();
        let head = self.queue.pop_front();
        match head {
            None => {
                self.count += 1;
            }
            Some(task) => {
                task.detach_from_timeout();
                task.attach_to_ready_queue();
                set_dispatch_reserve_flag();
            }
        };
    }

    pub fn wait(&mut self) {
        let _sc = SystemCall::new();
        if self.count > 0 {
            self.count -= 1;
        } else {
            let task = detach_current_task().unwrap();
            task.attach_to_queue(&mut self.queue, self.order);
            set_dispatch_reserve_flag();
        }
    }

    pub fn wait_with_timeout(&mut self, time: RelTime) -> Result<(), Error> {
        let _sc = SystemCall::new();
        if self.count > 0 {
            self.count -= 1;
            Ok(())
        } else {
            let task = detach_current_task().unwrap();
            task.attach_to_queue(&mut self.queue, self.order);
            task.attach_to_timeout(time);
            set_dispatch_reserve_flag();
            task.result()
        }
    }
}
