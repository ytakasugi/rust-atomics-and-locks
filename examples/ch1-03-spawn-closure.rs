use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    let arr = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = arr.len();
        let sum = arr.iter().sum::<usize>();

        sum / len
    });

    let avg = t.join().unwrap();
    println!("Average: {avg}");
}
