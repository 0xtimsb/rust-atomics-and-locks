use std::rc::Rc;

fn main() {
    let x = vec![1, 2, 3];

    // here when cloning, the reference count is incremented and data is not cloned.
    let y = Rc::new(x);
    let z = y.clone();

    // let mut y = Rc::new(vec![1, 2, 3]);
    // This is allowed because we're changing what `y` points to
    // y = Rc::new(vec![4, 5, 6]);

    // value in Rc is immutable by design. later discussed how to mutate it.

    println!("y: {:?}", y);
    println!("z: {:?}", z);

    println!("y: {:?}", y.as_ptr());
    println!("z: {:?}", z.as_ptr());

    // rc cannot be used for multi-threading.
    // why? if multiple threads are trying to modify rc counter at same time, it will cause weird bugs.

    // more on how to use it in threads later.
}
