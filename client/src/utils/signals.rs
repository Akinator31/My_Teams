use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

pub static SHUTDOWN: AtomicBool = AtomicBool::new(false);

unsafe extern "C" {
    fn signal(signum: i32, handler: usize) -> usize;
}

extern "C" fn handle_sigint(_: i32) {
    SHUTDOWN.store(true, SeqCst);
}


pub fn setup_signal_handler() {
    unsafe {
        signal(2, handle_sigint as *const () as usize);
    }
}