use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

pub static SHUTDOWN: AtomicBool = AtomicBool::new(false);

unsafe extern "C" {
    fn signal(signum: i32, handler: usize) -> usize;
}

extern "C" fn handle_shutdown(_: i32) {
    SHUTDOWN.store(true, SeqCst);
}

pub fn setup_signal_handler() {
    unsafe {
        signal(2, handle_shutdown as *const () as usize);
        signal(15, handle_shutdown as *const () as usize);
    }
}
