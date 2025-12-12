use std::time::Duration;
use std::thread;

fn slow(name: &str, ms: u64) {// This will simulate a slow function for us
    thread::sleep(Duration::from_millis(ms));// We call thread::sleep because it is blocking
    println!("'{name}' ran for {ms}ms");
}

fn main() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started");
            slow("a", 30);
            trpl::yield_now().await;// yield_now gives the controll back to the runtime
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished");
        };

        let b = async {
            println!("'b' started");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished");
        };

        trpl::select(a,b).await;
    });
}
