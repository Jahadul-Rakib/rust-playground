use std::ops::Add;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn test() {
    let thread1 = thread::current();
    println!("{}", thread1.name().unwrap());

    let thread_one_name = "test-1";
    let thread_two_name = "test-2";
    scope_thread_example(&thread_one_name);
    swpan_thread_example();

    // channel example
    let (sender, receiver) = channel();

    thread::spawn(move || {
        send_channel_example(sender);
    });

    loop {
        if receiver.recv().is_ok() {
            let result = receiver.recv().unwrap();
            println!("received: {}", result)
        } else {
            break;
        }
    }
}

fn swpan_thread_example() {
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawn thread {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    handle.join();
}

fn scope_thread_example(thread_one_name: &&str) {
    thread::scope(|scope| {
        scope.spawn(|| {
            for data in 0..5 {
                println!("scope thread {} {}", &thread_one_name, data)
            }
        });
    });
}
fn send_channel_example(x: Sender<&str>) {
    for i in 0..10 {
        thread::sleep(Duration::from_secs(1));
        x.send("send-by-loop").unwrap();
    }
}
