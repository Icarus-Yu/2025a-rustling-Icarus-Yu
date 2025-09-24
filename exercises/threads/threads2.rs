// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 1. 使用 Arc<Mutex<T>> 来包裹共享状态
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 2. 在修改共享值之前，必须先获取锁
            let mut status_lock = status_shared.lock().unwrap();
            status_lock.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // 先等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 3. 所有工作完成后，再获取锁并打印最终结果
    // 这样只会输出一次最终的总数 "10"
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
}