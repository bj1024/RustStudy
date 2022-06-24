use std::{thread, time::Duration};
use tokio::{join, sync::broadcast};

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    let task1 = tokio::spawn(async move {
        // assert_eq!(rx1.recv().await.unwrap(), 10);
        // assert_eq!(rx1.recv().await.unwrap(), 20);
        // thread::sleep(Duration::from_millis(3000));
        loop {
            let val = match rx1.recv().await {
                Ok(val) => {
                    println!("task1 received.{:?}", val);
                }
                Err(_) => break,
            };

            //     Ok(Some(val)) => {
            //         println!("task1 received.{:?}", val);
            //     }
            //     None => break,
            // };
        }
    });

    let task2 = tokio::spawn(async move {
        loop {
            let val = match rx2.recv().await {
                Ok(val) => {
                    println!("task2 received.{:?}", val);
                }
                Err(_) => break,
            };
        }
    });

    // println!("tx.send(10)",);
    // tx.send(10).unwrap();
    // thread::sleep(Duration::from_millis(1000));
    // println!("tx.send(20)",);
    // tx.send(20).unwrap();
    // println!("tx.send(30)",);
    // tx.send(30).unwrap();
    for i in 0..3 {
        println!("tx.send({})", i);
        tx.send(i).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
    drop(tx);
    let _ = join!(task1, task2);
    println!("end.");
}
