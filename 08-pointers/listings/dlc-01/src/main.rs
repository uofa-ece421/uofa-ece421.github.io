use std::rc::{Rc, Weak};
use std::mem;
use std::fmt;

// ANCHOR: here
struct DLNode<T> {
    next: Weak<DLNode<T>>,
    prev: Weak<DLNode<T>>,
    val: T,
}

impl<T: std::default::Default> DLNode<T> {
    fn new(val: T, next: Weak<DLNode<T>>, prev: Weak<DLNode<T>>) -> DLNode<T> {
        DLNode { next, prev, val }
    }

    fn empty() -> Rc<Self> {
        Rc::new_cyclic(|me| {
            DLNode {
                next: me.clone(),
                prev: me.clone(),
                val: Default::default(),
            }
        })
    }
}

struct DList<T> {
    root: Rc<DLNode<T>>,
}
// ANCHOR_END: here

impl<T: std::fmt::Display> fmt::Display for DList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let p = &self.root.next.upgrade().unwrap();
        loop {
            if std::ptr::eq(p, &self.root) {
                break;
            }
            write!(f, "{} ", p.val)?;
            let p = &p.next.upgrade().unwrap();
        }
        write!(f, "]")
    }
}  

impl<T: std::default::Default> DList<T> {
    fn new() -> DList<T> {
        DList { root: DLNode::<T>::empty() }
    }

    fn push(&mut self, val: T) {
        let node = Rc::new(DLNode::new(val, self.root.next, Rc::downgrade(&self.root)));
        let root_next = mem::replace(&mut self.root.next, Rc::downgrade(&node));
        let rc_next = root_next.upgrade().unwrap();
        rc_next.prev = Rc::downgrade(&node);
    }
}

fn main() {
    let mut stack = DList::new();

    stack.push(10);
    stack.push(2);
    stack.push(42);
    println!("first stack {}", stack);
}
