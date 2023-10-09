use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let signature = std::env::args().next().expect("Signature not present");

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Signature {signature}");
    while running.load(Ordering::SeqCst) {}
    println!("Shutting down...");
}
