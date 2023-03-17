use std::{
    collections::HashMap,
    mem::drop,
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

fn hash_map_mutex() {
    // Duplicate data
    let key_vec = Arc::new(vec![5, 2, 6, 2, 5, 1]);
    let value_vec = Arc::new(vec![-0.1, 0.2, 0.0, 0.2, -0.1, -0.3]);

    // Atomic reference counter for mutex of HashMap
    // If you need sort map by key that use BTreeMap instead of HashMap
    let map = Arc::new(Mutex::new(HashMap::new()));

    // Create two threads
    for th in 0..2 {
        // Get references for every thread
        let key_vec = key_vec.clone();
        let value_vec = value_vec.clone();
        let map = map.clone();

        spawn(move || {
            for i in 0..key_vec.len() {
                if th == 0 && i % 2 == 0 {
                    // Lock map for other threads
                    let mut map = map.lock().unwrap();
                    map.insert(key_vec[i].clone(), value_vec[i].clone());
                    // Manually unlock mutex if needed
                    drop(map);
                } else if th == 1 && i % 2 != 0 {
                    let mut map = map.lock().unwrap();
                    map.insert(key_vec[i].clone(), value_vec[i].clone());
                    // drop(map);
                }
            }
        });
    }

    // Waiting result in main thread (must use std::mpsc::channel calls instead of sleep)
    sleep(Duration::from_secs(1));

    // Result
    println!("map: {:?}", &map);
}
