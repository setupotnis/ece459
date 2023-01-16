use std::sync::mpsc;
use std::thread;
/*
Modify the above example so that the created thread takes
two ints from main(), adds them, and passes the sum back
along the channel.
 */
fn main() {
    let num_1 = 5;
    let num_2 = 10;
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let sum = num_1 + num_2;
        tx.send(sum).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("The sum is: {}", received);
}
