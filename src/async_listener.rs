use std::{
    thread::{sleep, spawn},
    time::Duration,
};
use tokio;

async fn loop_fn() {
    println!("Start loop");
    spawn(|| loop {
        sleep(Duration::from_secs(3));
        println!("loop");
    });
}

pub async fn async_listener() {
    loop_fn().await;
    println!("to be");
    loop {}
}
