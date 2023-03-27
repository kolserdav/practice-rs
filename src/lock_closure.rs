use once_cell::sync::Lazy;
use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

struct Clos {
    start: String,
}

impl Clos {
    fn with_closure<F>(&self, cb: F)
    where
        F: Fn(String) + Send + 'static,
    {
        spawn(move || loop {
            sleep(Duration::from_secs(2));
            cb("call closure".to_string());
        });
    }
}

static CONN: Lazy<Arc<Mutex<Clos>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Clos {
        start: String::from("start"),
    }))
});

pub fn lock_closure() {
    let conn = CONN.lock().unwrap();

    conn.with_closure(|m| {
        let conn = CONN.clone();
        let conn = conn.lock().unwrap();
        println!("{}: {}", &conn.start, m);
    });
    drop(conn);
    loop {}
}
