use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::sync::Arc;

fn play1() {
    thread::spawn(|| {
        for i in 0..9 {
            println!("Hello from spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("Hello from the main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn play2() {
    for i in 0..9 {
        thread::spawn(move || {
            println!("Hello from spawned thread number: {}", i);
            thread::sleep(Duration::from_millis(1));
        });
    }

    for i in 0..5 {
        println!("Hello from the main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn play3() {
    let handle = thread::spawn(|| {
        for i in 0..9 {
            println!("Hello from spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("Hello from the main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // ensure the spawned thread is finished before leaving the function
    handle.join().unwrap();
}

/// 
/// Message passing is a way of handling concurrency
/// Channels is like single ownership
/// Once the value is transferred it should not be used anymore
/// 

fn play4() {
    // Multiple Producers Single Consuner
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = "Hi".to_string();

        // succesful send moves the value, it can't be used after
        tx.send(message).unwrap();
    });

    // recv is blocking, try_recv is not
    let message = rx.recv().unwrap();
    println!("Received: {} from spawned thread", message);
}

fn play5() {
    let (tx, rx) = mpsc::channel();

    let message_list = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for message in message_list {
        let local_tx = tx.clone();

        thread::spawn(move || {
            local_tx.send(message).unwrap();
        });
    }

    // we don't call recv on rx but treat it as an iterator
    // Note: the program will never end, as it will wait forever
    // after the last message is sent
    for received in rx {
        println!("Received {}", received);
    }

}

fn play6() {
    let (tx, rx) = mpsc::channel();

    let message_list = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    // ensure all threads will be joined at the end of the scope
    // note: this allow threads to borrow non static datas
    thread::scope(|s| {
        for message in message_list {
            let local_tx = tx.clone();

            s.spawn(move || {
                local_tx.send(message).unwrap();
            });
        }
    });

    // we don't call recv on rx but treat it as an iterator
    // I thought this was correcting the forever running program
    // but no: I have to close the transmitter
    for received in rx {
        println!("Received {}", received);
    }

}

fn play7() {
    let (tx, rx) = mpsc::channel();

    let message_list = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for message in message_list {
        let local_tx = tx.clone();

        thread::spawn(move || {
            local_tx.send(message).unwrap();
        });
    }

    // I have to close the channel, and tx is still hanging around
    // local_txs have been closed by the scope
    drop(tx);

    // we don't call recv on rx but treat it as an iterator 
    // iteration will finish when the "channel is closed"   
    for received in rx {
        println!("Received {}", received);
    }
}

///
/// Memory sharing is another way of handling concurrency
/// Here it is more like multiple ownership:
/// multiple thread can access the same memory location at
/// the same time
/// deadlock empire, here we come !
/// (saw in embedded Rust OS project, even Rust compiler
/// can't prevent deadlocks to happen)
/// 


fn play8() {
    let counter = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            // thread scope avoid us the hassle of moving the mutex
            // as the thread lifetime will not exceed the scope
            // it seems it can borrow the mutex without any issues
            s.spawn(|| {
                let mut count = counter.lock().unwrap();

                *count += 1;
            });
        }
    });

    println!("Result: {}", *counter.lock().unwrap());
}

fn play9() {
    // without thread scope we have to move the Mutex
    // but a Rc can't be sent safely between threads
    // (so no Send trait)
    // it use due the fact that when changing the reference
    // count, it does not use thread safe primitives
    // So we go with Atomic Rc, which has the same API
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


fn main() {
    play9();
}
