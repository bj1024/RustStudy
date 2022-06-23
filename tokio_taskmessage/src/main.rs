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
            let val = rx1.recv().await.unwrap();
            println!("task1 received.{:?}", val);
        }
    });

    let task2 = tokio::spawn(async move {
        loop {
            let val = rx2.recv().await.unwrap();
            println!("task2 received.{:?}", val);
        }
        // assert_eq!(rx2.recv().await.unwrap(), 10);
        // assert_eq!(rx2.recv().await.unwrap(), 20);
        // thread::sleep(Duration::from_millis(2000));
    });

    // println!("tx.send(10)",);
    // tx.send(10).unwrap();
    // thread::sleep(Duration::from_millis(1000));
    // println!("tx.send(20)",);
    // tx.send(20).unwrap();
    // println!("tx.send(30)",);
    // tx.send(30).unwrap();
    for i in 0..10 {
        println!("tx.send({})", i);
        tx.send(i).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
    let _ = join!(task1, task2);
}
