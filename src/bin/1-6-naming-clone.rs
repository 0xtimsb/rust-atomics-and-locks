use std::{sync::Arc, thread};

fn main() {
    let a = Arc::new([1, 2, 3]);

    let handle1 = thread::spawn({
        // shadowing is done here.
        let a = a.clone();
        // now move is used.
        move || {
            println!("{:?}", a);
        }
    });

    println!("main: {:?}", a);

    handle1.join().unwrap();
}

// fn main() {
//     let a = Arc::new([1, 2, 3]);
//     let b = a.clone();

//     // here you have to define new variable b, to pass it to thread.
//     let handle1 = thread::spawn(move || println!("{:?}", b));

//     // now you can access a here
//     println!("main: {:?}", a);

//     handle1.join().unwrap();
// }
