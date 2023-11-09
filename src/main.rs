use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

fn main() {
    play7();
}
