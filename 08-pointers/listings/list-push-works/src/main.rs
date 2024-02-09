use std::mem;
use std::fmt;

struct SLNode<T> {
    next: Option<Box<SLNode<T>>>, // Option needed because the last element's next is None
    val: T,
}

impl<T> SLNode<T> {
    fn new(val: T, next: Option<Box<SLNode<T>>>) -> SLNode<T> {
        SLNode {
            next,
            val,
        }
    }
}   

struct SList<T> {
    root: Option<Box<SLNode<T>>>,
}

impl<T> SList<T> {
    fn new() -> SList<T> {
        SList { root: None }
    }

// ANCHOR: here
    fn push(&mut self, val: T) {
        let new_next = mem::replace(&mut self.root, None);
        self.root = Some(Box::new(SLNode::new(val, new_next)));
    }
// ANCHOR_END: here
}

impl<T: std::fmt::Display> fmt::Display for SList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut p = &self.root;
        while let Some(curr) = p {
            write!(f, "{} ", curr.val)?;
            p = &curr.next;
        }
        write!(f, "]")
    }
}  
    
fn main() {
    let mut stack = SList::new();

    stack.push(10);
    stack.push(2);
    stack.push(42);
    println!("our first list based stack: {}", stack);
}

