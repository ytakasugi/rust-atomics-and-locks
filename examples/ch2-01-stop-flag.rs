use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn some_work() {
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);
    // 作業スレッドを生成
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });
    // メインスレッドを使用して、ユーザーの入力を待ちます。
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }
    // バックグラウンドスレッドに停止する必要があることを通知します。
    STOP.store(true, Relaxed);
    // バックグラウンドスレッドが終了するまで待ちます。
    background_thread.join().unwrap();
}
