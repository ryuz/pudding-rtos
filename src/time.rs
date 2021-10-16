use crate::system::*;
use crate::*;

pub fn supply_time_tick(tick: RelTime) {
    let _sc = SystemCall::new();
    task::supply_time_tick_for_timeout(tick);
}

pub fn sleep(time: RelTime) {
    let _sc = SystemCall::new();
    let task = current_task().unwrap();
    task.detach_from_queue(); // レディーキューから取り外す
    task.attach_to_timeout(time); // タイムアウトキューに繋ぐ
    set_dispatch_reserve_flag();
}
