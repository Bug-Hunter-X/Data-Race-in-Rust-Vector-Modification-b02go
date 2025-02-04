use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));
    let v_clone = v.clone();

    let handle = std::thread::spawn(move || {
        let mut data = v_clone.lock().unwrap();
        *data.get_mut(0).unwrap() = 10; 
    });

    let mut data = v.lock().unwrap();
    println!("{:?}", *data);
    handle.join().unwrap();
    println!("{:?}", *v.lock().unwrap());
} 