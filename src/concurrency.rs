use std::sync::{mpsc, Mutex};
use std::thread;
use std::thread::current;
use std::time::Duration;

use crate::log;

pub fn run_concurrency_examples() {
    log::begin_topic("concurrency");
    simple_thread();
    thread_with_join();
    thread_with_move();
    thread_with_channel();
    thread_with_channel_multiple_msgs();
    multi_producer_single_consumer();
    mutex_example();
    using_thread_builder();
    using_park();
    log::end_topic("concurrency");
}


fn simple_thread() {
    log::start_example("concurrency basic");
    thread::spawn(|| {
        //println!("inside thread with id: {:?}",thread::current().id().as_u64());  //unstable

        for i in 1..10 {
            println!("no in thread :{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..10 {
        println!("no from main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    log::end_example("concurrency basic");
}

fn thread_with_join() {
    log::start_example("concurrency join");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("no in thread :{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap(); //this will force to wait all the related threads to exit
    for i in 1..10 {
        println!("no from main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    log::end_example("concurrency join");
}

fn thread_with_move() {
    log::start_example("concurrency move");

    let s = String::from("main");
    let handle = thread::spawn(move || {  //moves ownership of s to the thread
        for i in 1..10 {
            println!("{} : {} ", s, i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();


    log::end_example("concurrency move");
}

fn thread_with_channel() {
    log::start_example("concurrency channel");
    let (transmitter, receiver) = mpsc::channel(); //creates a transmitter and receiver

    thread::spawn(move || {
        let val = String::from("hi");
        transmitter.send(val).unwrap(); //this will move the ownership as well
    });

    let received = receiver.recv().unwrap();
    println!("got : {}", received);
    log::end_example("concurrency channel");
}

fn thread_with_channel_multiple_msgs() {
    log::start_example("concurrency channel_multiple_msgs");
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![String::from("a1"), String::from("a2"), String::from("a3"), String::from("a4"), String::from("a5")];

        for val in vals {
            transmitter.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver {
        println!("got : {}", received);
    }


    log::end_example("concurrency channel_multiple_msgs");
}

fn multi_producer_single_consumer() {
    log::start_example("concurrency multi_producer_single_consumer");
    let (transmitter1, receiver) = mpsc::channel();

    let transmitter2 = transmitter1.clone(); //creating a clone of the transmitter

    thread::spawn(move || {
        let vals = vec![String::from("a1"), String::from("a2"), String::from("a3"), String::from("a4"), String::from("a5")];

        for val in vals {
            transmitter1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![String::from("b1"), String::from("b2"), String::from("b3"), String::from("b4"), String::from("b5")];

        for val in vals {
            transmitter2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver {
        println!("got : {}", received);
    }


    log::end_example("concurrency multi_producer_single_consumer");
}

fn mutex_example() {
    log::start_example("concurrency mutex");
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m: {:?}", m);
    log::end_example("concurrency mutex");
}

fn using_thread_builder() {
    log::start_example("concurrency thread builder");
    let builder = thread::Builder::new()
        .name("test-thread".into())
        .stack_size(32 * 1024);

    let handler = builder.spawn(|| {
        println!("inside named thread");
        println!("inside {:?}", thread::current().name())
    }).unwrap();

    handler.join().unwrap();
    log::end_example("concurrency builder");
}

fn using_park() {
    log::start_example("concurrency parking");
    let parked_thread = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();

// Let some time pass for the thread to be spawned.
    thread::sleep(Duration::from_millis(10));

    println!("Unpark the thread");
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
    log::end_example("concurrency parking");
}