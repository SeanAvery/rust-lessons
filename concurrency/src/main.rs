// use std::thread;
// use std::time::Duration;
// use std::sync::{Arc, Mutex};
//
// fn main() {
//     let data = Arc::new(Mutex::new(vec![1, 2, 3]));
//
//     for i in 0..3 {
//         let data = data.clone();
//         thread::spawn(move || {
//             let mut data = data.lock().unwrap();
//             data[0] += i;
//         });
//     }
//
//     thread::sleep(Duration::from_millis(50));
// }

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }
}
