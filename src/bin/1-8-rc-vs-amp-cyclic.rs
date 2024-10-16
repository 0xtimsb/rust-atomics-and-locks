use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn create_cycle() -> (Rc<Node>, Rc<Node>) {
    let node1 = Rc::new(Node {
        value: 1,
        next: None,
    });
    let node2 = Rc::new(Node {
        value: 2,
        next: Some(Rc::clone(&node1)),
    });

    // Create node1 with a reference to node2, completing the cycle
    let node1 = Rc::new(Node {
        value: 1,
        next: Some(Rc::clone(&node2)),
    });

    (node1, node2)
}

fn main() {
    let (node1, node2) = create_cycle();

    // Access values
    println!("Node 1 value: {}", node1.value);
    println!("Node 2 value: {}", node2.value);
    println!(
        "Node 1's next value: {}",
        node1.next.as_ref().unwrap().value
    );
    println!(
        "Node 2's next value: {}",
        node2.next.as_ref().unwrap().value
    );
}
