mod kvstore;
mod workload;

use std::{sync::Arc, thread, env};
use std::time::{Duration, Instant};
use bytes::{Buf, Bytes};

use crate::kvstore::KvStore;
use crate::workload::{Workload, Operation, Ratios};



fn put_key(
    store: Arc<KvStore>,
    key: String,
    body: Bytes,
) -> Result<String, String> {
    if body.remaining() == 0 {
        Err(String::from("Missing Body"))
    } else {
        match store.set(key.clone(), body.to_vec()) {
            Some(kv_element) => {
                if kv_element.locked {
                    Ok(String::from("Locked"))
                } else {
                    Ok(String::from("Updated!"))
                }
            }
            None => Ok(String::from("Created!"))
        }
    }
}

fn get_key(store: Arc<KvStore>, key: String) -> Result<String, String> {
    match store.get(key) {
        Some(value) => Ok(String::from_utf8(value.data).unwrap()),
        None => Err(String::from("KeyNotFound"))
    }
}

fn find_key(store: Arc<KvStore>, key: String) -> Result<String, String> {
    match store.get(key) {
        Some(value) => Ok(String::from("KeyFound")),
        None => Err(String::from("KeyNotFound"))
    }
}

fn delete_key(store: Arc<KvStore>, key: String) -> Result<String, String> {
    match store.get(key.clone()) {
        Some(_) => {
            (*store).drop(key);
            Ok(String::from("KeyDeleted"))
        }
        None => Err(String::from("KeyNotFound"))
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let thread_num:i64 = args[1].parse().unwrap();
    let put_ratio:f32 = args[2].parse().unwrap();
    let get_ratio:f32 = args[3].parse().unwrap();
    let delete_ratio:f32 = args[4].parse().unwrap();
    let op_total_cnt:i64 = args[5].parse().unwrap();
    let body_len:i64 = args[6].parse().unwrap();
    let resource_cnt:i64 = args[7].parse().unwrap();
    let mconfig = Arc::new(Ratios{put: put_ratio, get: get_ratio, delete: delete_ratio});
    let store = Arc::new(KvStore::new());
    let mut handles = vec![];

    let start = Instant::now();

    for i in 0..thread_num {
        let t_store = Arc::clone(&store);
        let t_config = Arc::clone(&mconfig);
        let handle = thread::spawn(move || {
            let mut success_cnt : i64= 0;
            let mut fail_cnt : i64= 0;
            let thread_op_cnt = op_total_cnt/thread_num;
            let mut workload = Workload::new(*t_config, thread_op_cnt, resource_cnt, body_len);
            for j in 0..thread_op_cnt {
                let op = workload.get_next_op();
                let result : Result<String, String> = {
                    if op.category == 0 {
                        put_key(Arc::clone(&t_store), op.key, Bytes::copy_from_slice(op.body.as_bytes()))
                    } else if op.category == 1 {
                        get_key(Arc::clone(&t_store), op.key)
                    } else {
                        delete_key(Arc::clone(&t_store), op.key)
                    }
                };
                match result {
                    Ok(res) => {
                        success_cnt += 1;
                    }
                    Err(res) => {
                        fail_cnt += 1;
                    }
                }
            }
            println!("thread {}, success_cnt: {}, fail_cnt: {}", i, success_cnt, fail_cnt);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Time elapsed in lucid is: {:?}", duration);

}
