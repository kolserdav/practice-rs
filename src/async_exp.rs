use futures::executor::block_on;
use tokio::runtime::Runtime;

async fn test_as() -> String {
    println!("async");
    return String::from("test");
}

pub fn async_exp_tokio() {
    let rt = Runtime::new().unwrap();
    let future = test_as();
    let r = rt.block_on(future);
    println!("{r}");
}

pub fn async_exp() {
    let v = block_on(test_as());
    println!("{v}");
}
