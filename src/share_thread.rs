use std::rc::Rc;
use std::thread;

pub fn func_d() {
    let mut handles = Vec::new();
    let mut data = Rc::new(vec![1; 10]);

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            data_ref[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);
}
