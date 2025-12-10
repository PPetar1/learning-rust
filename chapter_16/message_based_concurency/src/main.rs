use std::sync::mpsc;// mpsc is short for multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();// We make a second producer by cloning the tx value
    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();// We are sending the message via the channel and unwraping in order
                              // to panic if there is an error
        //println!("val is {val}");// This code won't execute because we transfered ownership of
                                   // val with our last line of code
        
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    let received = rx.recv().unwrap(); // This blocks until it receives the message; there is also
                                       // try_recv() which returns an Err if there are no messages
                                       // at the time
    println!("Got: {received}");

    for received in rx {
        println!("Got: {received}");
    }
}
