use crate::system::*;
use crate::*;

static mut SYSTEM_TIME: SysTime = 0;

pub fn supply_time_tick(tick: RelTime) {
    let _sc = SystemCall::new();
    task::supply_time_tick_for_timeout(tick);
    unsafe {
        SYSTEM_TIME = SYSTEM_TIME.wrapping_add(tick as SysTime);
    }
}

pub fn set_system_time(systime: SysTime) {
    let _sc = SystemCall::new();
    unsafe {
        SYSTEM_TIME = systime;
    }
}

pub fn system_time() -> SysTime {
    let _sc = SystemCall::new();
    unsafe { SYSTEM_TIME }
}

pub fn sleep(time: RelTime) {
    if time > 0 {
        let _sc = SystemCall::new();
        let task = current_task().unwrap();
        task.detach_from_queue(); // レディーキューから取り外す
        task.attach_to_timeout(time); // タイムアウトキューに繋ぐ
        set_dispatch_reserve_flag();
    }
}

pub struct Rate {
    next_time: SysTime,
    interval: RelTime,
}

impl Rate {
    pub fn new(interval: RelTime) -> Self {
        Rate {
            next_time: system_time(),
            interval: interval,
        }
    }

    // もし set_system_time した場合は必ず呼ぶこと
    pub fn reset(&mut self) {
        self.next_time = system_time();
    }

    pub fn sleep(&mut self) {
        let cur_time = system_time();
        loop {
            let sleep_time = self.next_time.wrapping_sub(cur_time);
            self.next_time = self.next_time.wrapping_add(self.interval as SysTime);
            if sleep_time > 0 && sleep_time <= self.interval as SysTime {
                sleep(sleep_time as RelTime);
                break;
            }
        }
    }
}
