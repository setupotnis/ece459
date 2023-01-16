use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let v0 = 0;
    let rb = Arc::new(Mutex::new(v0));
    let rb_clone = rb.clone();

    let handle = thread::spawn(move || {
        let mut v = rb_clone.lock().unwrap();
        println!("Got: {}", v);
        *v = 5;
    });

    handle.join().unwrap();
    println!("Got: {}", rb.lock().unwrap());
}
