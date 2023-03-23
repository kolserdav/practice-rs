mod async_exp;
use async_exp::async_exp;
mod async_listener;
use async_listener::async_listener;
mod await_duration;
use await_duration::await_duration;

#[tokio::main]
async fn main() {
    await_duration().await;
}
