use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("length: {}", numbers.len());
    });

    // can't use numbers here because it's moved to the thread.
    // println!("numbers: {:?}", numbers);

    println!("Waiting for thread to finish...");
    handle.join().unwrap();
    println!("Thread finished!");
}
