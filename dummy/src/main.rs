use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let signature = std::env::args()
        .skip(1)
        .next()
        .expect("Signature not present");

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("PID {}", std::process::id());
    println!("Signature {signature}");
    while running.load(Ordering::SeqCst) {}
    println!("Shutting down...");

    let path = std::env::current_dir().unwrap();
    let path = path.join("signature.txt");

    println!("Writing signature to {path:?}");

    std::fs::write(path, signature).expect("Unable to write file");
}
