use crate::system::*;
use crate::*;



#[derive(Clone, Copy)]
pub enum WaitFlagMode {
    AndWait,
    OrWait,
}


// EventFlag control block
pub struct Eventflag {
    queue: TaskQueue,
    flg_ptn: FlagPattern,
    wait_ptn: FlagPattern,
    wait_mode: WaitFlagMode,
}


impl Eventflag {
    pub const fn new(init_pattern: FlagPattern) -> Self {
        Eventflag {
            queue: TaskQueue::new(),
            flg_ptn: init_pattern,
            wait_ptn: 0,
            wait_mode: WaitFlagMode::AndWait,
        }
    }

    fn is_match_pattern(&self) -> bool {
        match self.wait_mode {
            WaitFlagMode::AndWait => { (self.flg_ptn & self.wait_ptn) == self.wait_ptn },
            WaitFlagMode::OrWait => { (self.flg_ptn & self.wait_ptn) != 0 },
        }
    }

    pub fn set_flag(&mut self, pattern: FlagPattern) {
        let _sc = SystemCall::new();
        self.flg_ptn |= pattern;
        let front = self.queue.front();
        match front {
            None => {},
            Some(task) => {
                if self.is_match_pattern() {
                    task.detach_from_queue();
                    task.detach_from_timeout();
                    task.attach_to_ready_queue();
                    set_dispatch_reserve_flag();
                }
            }
        }
    }

    pub fn clear_flag(&mut self, pattern: FlagPattern) {
        let _sc = SystemCall::new();
        self.flg_ptn &= pattern;
    }

    pub fn wait(&mut self, wait_pattern: FlagPattern, wait_mode: WaitFlagMode) {
        let _sc = SystemCall::new();
        assert!(self.queue.is_empty());   // TA_WSGL のみ

        self.wait_ptn = wait_pattern;
        self.wait_mode = wait_mode;
        if !self.is_match_pattern() {
            let task = detach_current_task().unwrap();
            task.attach_to_queue(&mut self.queue, Order::Fifo);
            set_dispatch_reserve_flag();
        }
    }

    
    pub fn polling(&mut self, wait_pattern: FlagPattern, wait_mode: WaitFlagMode) -> Result<(), Error>
    {
        let _sc = SystemCall::new();
        assert!(self.queue.is_empty());   // TA_WSGL のみ
        self.wait_ptn = wait_pattern;
        self.wait_mode = wait_mode;
        if self.is_match_pattern() {Ok(())} else {Err(Error::Timeout)}

    }

    pub fn wait_with_timeout(&mut self, wait_pattern: FlagPattern, wait_mode: WaitFlagMode, time: RelativeTime) -> Result<(), Error>
    {
        let task = current_task().unwrap();
        {
            let _sc = SystemCall::new();
            assert!(self.queue.is_empty());   // TA_WSGL のみ

            self.wait_ptn = wait_pattern;
            self.wait_mode = wait_mode;
            if self.is_match_pattern() {
                task.set_result(Ok(()));
            }
            else {
                task.detach_from_queue();
                task.attach_to_queue(&mut self.queue, Order::Fifo);
                task.attach_to_timeout(time);
                set_dispatch_reserve_flag();
            }
        }
        task.result()
    }    
}

