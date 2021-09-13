use bitflags::bitflags;

bitflags! {
    pub struct ContextStatus: u8 {
        const TASK = 0x00;
        const NON_TASK = 0x01;
        const CPU_LOCK = 0x02;
        const DISABLE_DSP = 0x04;
    }
}


pub struct SystemControlBlock {
    status: ContextStatus,
    dispatch_pending: bool,
}

static mut SYSCB: SystemControlBlock = SystemControlBlock {
    status: ContextStatus::TASK,
    dispatch_pending: false,
};

pub fn set_non_task_state() {
    unsafe {
        SYSCB.status = SYSCB.status | ContextStatus::NON_TASK;
    }
}

pub fn is_dispatch_pending_state() -> bool {
    unsafe { SYSCB.dispatch_pending }
}
