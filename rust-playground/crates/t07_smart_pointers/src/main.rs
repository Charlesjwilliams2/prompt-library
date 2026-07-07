// TODO(reader): every smart pointer here trades something for something.
// As you read each one, ask: what would this code look like with a plain
// owned value instead, and what stops that from working?

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            children: Vec::new(),
        }))
    }
}

fn sum_tree(node: &Rc<RefCell<TreeNode>>) -> i32 {
    let node = node.borrow();
    node.value + node.children.iter().map(sum_tree).sum::<i32>()
}

// A single-threaded counter. `Rc<RefCell<T>>` is the right tool here:
// multiple owners (`Rc`) need to mutate shared state (`RefCell`), but
// everything happens on one thread, so there's no need for atomics/locks.
fn single_threaded_counter() {
    let root = TreeNode::new(1);
    let child_a = TreeNode::new(2);
    let child_b = TreeNode::new(3);
    root.borrow_mut().children.push(Rc::clone(&child_a));
    root.borrow_mut().children.push(Rc::clone(&child_b));

    println!("tree sum: {}", sum_tree(&root));
    println!("child_a strong_count: {}", Rc::strong_count(&child_a));

    // TODO(reader): uncomment this to see `RefCell`'s runtime borrow check
    // panic instead of a compile error — `RefCell` moves borrow-checking
    // from compile time to run time.
    // let _first_borrow = root.borrow_mut();
    // let _second_borrow = root.borrow_mut(); // panics: already borrowed
}

// A value genuinely shared across threads needs `Arc` (atomic reference
// counting) instead of `Rc`, and `Mutex` instead of `RefCell` for the
// interior mutability, because `RefCell`'s borrow checks aren't thread-safe.
fn multi_threaded_counter() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            let mut guard = counter.lock().unwrap();
            *guard += 1;
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("threaded counter: {}", *counter.lock().unwrap());
}

fn main() {
    single_threaded_counter();
    multi_threaded_counter();

    // A plain `Box<T>` — no sharing, no shared mutability, just a
    // heap-allocated owned value. Useful for recursive types or when a
    // value is too large to move around on the stack.
    let boxed: Box<i32> = Box::new(42);
    println!("boxed: {boxed}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_tree_adds_all_children() {
        let root = TreeNode::new(1);
        let child = TreeNode::new(2);
        root.borrow_mut().children.push(Rc::clone(&child));
        assert_eq!(sum_tree(&root), 3);
    }

    #[test]
    fn rc_clone_increases_strong_count() {
        let node = TreeNode::new(1);
        assert_eq!(Rc::strong_count(&node), 1);
        let _also_node = Rc::clone(&node);
        assert_eq!(Rc::strong_count(&node), 2);
    }
}
