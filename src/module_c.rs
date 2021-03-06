use std::thread;

pub fn func_c() {
    let handle = thread::spawn(|| {
        println!("Hello World!");
    });
    dbg!(handle.join());
    thread10()
}

fn thread10() {
    let mut handles = Vec::new();
    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("Hello: {}", x);
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
}
