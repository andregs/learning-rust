use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    main_waits_spawned_to_finish();
    thread_captures_only_moved_data();
    thread_messaging();
    listening_to_messages();
    multiple_producers();
    thread_dies_with_main();
}

fn thread_dies_with_main() {
    println!("\nNote that spawned thread dies when main thread dies");

    thread::spawn(|| {
        for i in 1..100 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..4 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }

    // spawned thread should live longer than main thread, but gets killed here
}

fn main_waits_spawned_to_finish() {
    println!("\nMain thread waits spawned thread to finish");

    let spawned = thread::spawn(|| {
        for i in 1..7 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..4 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }

    spawned.join().unwrap();
}

fn thread_captures_only_moved_data() {
    let v = vec![1, 2, 3, 5, 7];
    
    let handle = thread::spawn(move || { // doesn't compile without 'move'
        println!("\nHere's some vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn thread_messaging() {
    println!("\nSpawned thread sends 'hi' to main thead");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // bad idea to read a value that was sent to another thread
        // but Rust doesn't compile this code 😊
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap(); // blocks untill msg arrives
    println!("Got: {}", received);
}

fn listening_to_messages() {
    println!("\nMain reacts whenever a message arrives");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = "I like this thing".split(" ");

        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx { // loops untill channel is closed
        println!("Got: {}", received);
    }
}

fn multiple_producers() {
    println!("\nMessages coming from multiple producers");

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let messages = "I like this thing".split(" ");

        for msg in messages {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let messages = "This is fun too".split(" ");

        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
