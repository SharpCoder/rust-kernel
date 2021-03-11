use crate::board::{timer, platform};

pub struct SystemContext {
    pub sysclock: timer::Timer,
}

static mut CONTEXT: SystemContext = SystemContext {
    sysclock: timer::Timer::new(platform::DMTIMER3),
};

pub fn configure_context(context: SystemContext) {
    unsafe {
        CONTEXT = context;
    }
}

pub fn get_context() -> &'static mut SystemContext {
    unsafe { return &mut CONTEXT };
}