use std::{sync::Arc, thread};

fn main() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone(); // same allocation, count is 2 now.

    // using arc make sure that the count increment is atomic.

    let handle1 = thread::spawn(move || println!("{:?}", a));
    let handle2 = thread::spawn(move || println!("{:?}", b));

    handle1.join().unwrap();
    handle2.join().unwrap();
}

// NOTE: WE STILL CAN'T MUTATE THE ARC OR RC YET.
