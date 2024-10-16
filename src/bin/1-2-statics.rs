use std::thread;

fn main() {
    // owned by program
    static X: [i32; 3] = [1, 2, 3];

    // this is only read only. mutability of statics is explored later.
    // todo: add mutability to statics file name here.
    let handle1 = thread::spawn(|| println!("{:?}", &X));
    let handle2 = thread::spawn(|| println!("{:?}", &X));

    handle1.join().unwrap();
    handle2.join().unwrap();
}
