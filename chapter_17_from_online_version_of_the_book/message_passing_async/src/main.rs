use std::time::Duration;

fn main() {
    trpl::block_on(async {
         let (tx, mut rx) = trpl::channel();

         let tx1 = tx.clone();
         let tx_fut1= async move {
             let vals = vec![
                 String::from("hi"),
                 String::from("from"),
                 String::from("the"),
                 String::from("future"),
             ];
             
             for val in vals {
                 tx1.send(val).unwrap();
                 trpl::sleep(Duration::from_millis(500)).await;
             }
         };


         let rx_fut = async move {
             while let Some(value) = rx.recv().await {
                println!("received '{value}'");
             } 
         };
         
         let tx_fut2 = async move {
             let vals = vec![
                 String::from("more"),
                 String::from("messages"),
                 String::from("for"),
                 String::from("you"),
             ];
             
             for val in vals {
                 tx.send(val).unwrap();
                 trpl::sleep(Duration::from_millis(1500)).await;
             }
         };

         trpl::join!(rx_fut, tx_fut1, tx_fut2); // Join macro awaits on multiple futures
    });
}
