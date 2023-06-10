use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    // `LOCKED`がfalseの場合、つまり`compare_exchange`がOk(bool)`を返した場合、DATAに`!`をpushする
    // 現在の値はfalseなので、比較は成功し、LOCKEDをtrueに変更する
    if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
        unsafe { DATA.push('!') }
        // `LOCKED`をfalseに変更
        LOCKED.store(false, Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });

    dbg!(unsafe { DATA.len() }); // [examples/ch3-08-lock.rs:22] unsafe { DATA.len() } = 100
    assert!(unsafe { DATA.len() } > 0);
    assert!(unsafe { DATA.chars().all(|c| c == '!') });
}
