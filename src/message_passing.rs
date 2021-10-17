use std::sync::mpsc;
use std::thread;

pub fn func_e() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data);
    });
    let _ = tx.send("Hello, world!");
    let _ = handle.join();
}
