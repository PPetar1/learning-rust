use std::time::Duration;

fn main() {
    trpl::block_on(async {
      // let future = trpl::spawn_task(async {
      //      for i in 1..10 {
      //          println!("hi number {i} from the first task!");
      //          trpl::sleep(Duration::from_millis(500)).await;
      //      }
      //  });

      //  for i in 1..5 {
      //      println!("hi number {i} from the second task!");
      //      trpl::sleep(Duration::from_millis(500)).await;
      //  }

      //  future.await.unwrap();// Here we have concurency without spawning a new os thread

        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first loop!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second loop!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;// This joins both futures into one that
                                                       // will finish when both futures finish
                                                       // Also join here is fair - it tries to
                                                       // split the execution fairly between two
                                                       // futures
    });
}
