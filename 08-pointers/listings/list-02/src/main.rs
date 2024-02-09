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

// ANCHOR: here
impl<T> SList<T> {
    fn new() -> SList<T> {
        SList { root: None }
    }

    fn push(&mut self, val: T) {
        self.root = Some(Box::new(SLNode::new(val, self.root)));
    }
}
// ANCHOR_END: here

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
    let stack = SList::<i32>::new();

    println!("our first list based stack: {}", stack);
}

