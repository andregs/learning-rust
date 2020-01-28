#[allow(unused_imports)]

use std::rc::Rc;
use std::sync::{{Arc, Mutex}};
use std::thread;

fn main() {
    how_lock_works();
    share_value_with_mutex();
}

fn how_lock_works() {
    let m = Mutex::new(5);
    {
        // "lock" blocks thread until it's our turn to have the lock
        let mut num = m.lock().unwrap();
        *num = 6;
    } // num is dropped and the lock released
    println!("\nm = {:?}", m);
}

// does not compile because 'counter' cannot have multiple owners
// fn share_value_with_mutex() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     println!("\nLet's have 10 threads incrementing a counter from 0 to 10");
//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             // we're moving ownershipt of "counter" from main thread to this thread
//             // but we're creating threads in a loop!
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap(); // make sure all threads finish
//     }

//     // acquire the lock to print the results
//     println!("Result: {}", *counter.lock().unwrap());
// }

// When wrapped in Rc, our Mutex can now have multiple owners, but it
// does not compile because Rc is not thread-safe.
// fn share_value_with_mutex() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     println!("\nLet's have 10 threads incrementing a counter from 0 to 10");
//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap(); // make sure all threads finish
//     }

//     // acquire the lock to print the results
//     println!("Result: {}", *counter.lock().unwrap());
// }

// Arc is the thread-safe version of Rc.
// That's why Mutex is often used with Arc (multi-threads),
// and RefCell is often used with Rc (single thread).
fn share_value_with_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    println!("\nLet's have 10 threads incrementing a counter from 0 to 10");
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // make sure all threads finish
    }

    // acquire the lock to print the results
    println!("Result: {}", *counter.lock().unwrap());
}
