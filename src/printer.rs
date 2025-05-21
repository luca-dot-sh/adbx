use crate::global_flags::VERBOSE;

pub fn print_dbg(msg: &str) {
    if VERBOSE.load(std::sync::atomic::Ordering::Relaxed) {
        print!("[DEBUG] {}", msg);
    }
}

pub fn println_debug(msg: &str) {
    if VERBOSE.load(std::sync::atomic::Ordering::Relaxed) {
        println!("[DEBUG] {}", msg);
    }
}
