use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    // mutable variables are not allowed yet.
    // if you try to use mut and use mutatate that variable in any of the threads, it will throw an error.
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}
