use core::sync::atomic::{AtomicI8, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn print_alternate(
    val: &str,
    pair: Arc<(Mutex<AtomicI8>, Condvar)>,
    expected_num: i8,
    new_num: i8,
) {
    loop {
        let (lock, cvar) = &*pair;
        let mut current_val = lock.lock().unwrap();
        while current_val.load(Ordering::Acquire) != expected_num {
            current_val = cvar.wait(current_val).unwrap();
        }
        current_val.store(new_num, Ordering::Release);
        println!("Result: {}", val);

        cvar.notify_all();
    }
}

fn main() {
    let pair1: Arc<(Mutex<AtomicI8>, Condvar)> =
        Arc::new((Mutex::new(AtomicI8::new(1)), Condvar::new()));
    let pair2 = Arc::clone(&pair1);
    let pair3 = Arc::clone(&pair1);

    let handle1 = thread::spawn(move || {
        print_alternate("One", pair1, 1, 2);
    });

    let handle2 = thread::spawn(move || {
        print_alternate("Two", pair2, 2, 3);
    });

    let handle3 = thread::spawn(move || {
        print_alternate("Three", pair3, 3, 1);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}
