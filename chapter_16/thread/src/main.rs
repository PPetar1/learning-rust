use std::thread;
use std::time::Duration;

fn main() {
    // We create a new thread with thread::spawn() which takes a closure that the thread will run
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hello {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();// Here we block the main thread until handled thread finishes

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {// move tells rust that the thread will take ownership of
                                        // the environment variables instead of borrowing them;
                                        // this code woudn't work without it because rust cannot
                                        // know that the thread won't outlive vector v
        println!("Here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}
