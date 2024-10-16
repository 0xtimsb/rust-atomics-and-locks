use std::thread;

fn main() {
    // box lives forever without any owner.
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    // here we are only moving the reference to the x variable. x is not moved as x don't have owner now.
    // move keyword is still used.
    let handle1 = thread::spawn(move || println!("{:?}", x));
    let handle2 = thread::spawn(move || println!("{:?}", x));

    // downside of leaking that we never drop or deallcate it. should only be done when absolutely necessary and limited number of times.

    handle1.join().unwrap();
    handle2.join().unwrap();
}
