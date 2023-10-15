use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let stop_process = Arc::new(AtomicBool::new(false));

    // Register a signal handler.
    let _ = signal_hook::flag::register(libc::SIGINT, Arc::clone(&stop_process));

    println!("Running ...");

    loop {
        // The process was requested to be stopped.
        if stop_process.load(Ordering::Relaxed) {
            println!("Got signal to stop");
            exit(0);
        }

        // Sleep a bit.
        thread::sleep(Duration::from_millis(1000));
    }
}
