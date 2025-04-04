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

// // 作用域结束 → `MutexGuard` 被 Drop → 自动解锁
// lock() 返回 Result‌
// Mutex::lock() 返回的是 Result<MutexGuard<T>, PoisonError<T>>，因为：
// ‌正常情况‌：获取锁成功 → 返回 Ok(MutexGuard)。
// ‌线程崩溃‌：如果持有锁的线程在锁定时崩溃（panic），锁会进入“中毒”（Poisoned）状态 → 返回 Err(PoisonError)。
// ‌unwrap() 的作用‌
// 直接解包 Result，若锁中毒则触发 panic（简化代码，生产环境建议用 ? 或 unwrap_or_else 处理错误）

fn main() {
    // Wrap JobStatus in a Mutex and then in an Arc for thread-safe shared access
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // Lock the Mutex before updating the shared value
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        
    }
    let final_status = status.lock().unwrap();
    println!("jobs completed {}", final_status.jobs_completed);
   
}
