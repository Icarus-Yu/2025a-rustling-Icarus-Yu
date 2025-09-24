//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// build.rs

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 1. 获取当前的 Unix 时间戳（秒）
    let now = SystemTime::now();
    let seconds_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 2. 使用 "cargo:rustc-env" 命令来设置环境变量
    // 这会让 TEST_FOO 变量在编译时对 tests7.rs 可见
    println!(
        "cargo:rustc-env=TEST_FOO={}",
        seconds_since_epoch
    );
     // --- 这是本题 tests8.rs 新增的代码 ---
    // 这个命令会告诉 Cargo 启用一个名为 "pass" 的 feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}