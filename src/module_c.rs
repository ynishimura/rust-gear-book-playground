use std::thread;

pub fn func_c() {
    let handle = thread::spawn(|| {
        println!("Hello World!");
    });
    dbg!(handle.join());
}
