use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};
use tokio::sync::mpsc::channel;

async fn wait_fn() {
    println!("Wait forever");
    sleep(Duration::from_secs(5));
    ()
}

async fn thread_job() {
    loop {
        let waited = Arc::new(Mutex::new(false));
        let waited_m = waited.clone();
        spawn(move || {
            let waited = waited.clone();
            sleep(Duration::from_secs(1));
            let waited = waited.lock().unwrap();
            if *waited == false {
                println!("End duration: {:?}", waited);
            }
        });

        wait_fn().await;

        let mut waited = waited_m.lock().unwrap();
        *waited = true;
        drop(waited);
    }
}

use futures::pin_mut;

pub async fn await_duration() {
    // let v = tokio::time::timeout(Duration::from_secs(1), wait_fn()).await;
    runner().await;
    println!("Need restart:");
}

async fn delay() {
    for _ in 0..6 {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        println!("Ping!");
    }
}

async fn runner() {
    let delayer = delay();

    if let Err(_) = tokio::time::timeout(std::time::Duration::from_secs(2), delayer).await {
        println!("Taking more than two seconds");
    }
}
