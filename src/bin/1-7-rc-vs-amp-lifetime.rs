use std::rc::Rc;

struct Data {
    value: i32,
}

fn create_and_process() -> Rc<Data> {
    // This data is created here and would normally go out of scope at the end of the function
    let data = Data { value: 42 };

    // We wrap it in an Rc to extend its lifetime and allow shared ownership
    let rc_data = Rc::new(data);

    // We can return the Rc<Data>, allowing the caller to own it
    rc_data
}

// fn create_and_process() -> &Data {
//     // This won't compile
//     let data = Data { value: 42 };
//     &data // Error: `data` does not live long enough
// }

fn main() {
    // Get the data from the function
    let shared_data = create_and_process();

    // We can still use the data here, even though the original `data` variable
    // in create_and_process() has gone out of scope
    println!("The value is: {}", shared_data.value);
}
