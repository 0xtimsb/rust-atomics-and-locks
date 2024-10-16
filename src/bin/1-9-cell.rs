use std::cell::Cell;

fn main() {
    let a = Cell::new(1);
    let b = &a;

    // here a is not mutable, but we can change its inner value
    // a is mere container, not the value itself
    // hence, changing value of b is equivalent to changing value of a
    a.set(2);
    b.set(3);

    // cell get set method can be used when type is Copy

    println!("a: {}", a.get());
    println!("b: {}", b.get());
}
