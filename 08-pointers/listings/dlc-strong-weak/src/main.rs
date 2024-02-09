use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::mem;
use std::fmt;

// ANCHOR: here
struct DLNode<T> {
    next: Option<StrongLink<T>>,
    prev: WeakLink<T>,
    val: Option<T>,
}

type StrongLink<T> = Rc<RefCell<DLNode<T>>>;
type WeakLink<T> = Weak<RefCell<DLNode<T>>>;

impl<T: std::default::Default> DLNode<T> {
    fn new(val: T, next: Option<StrongLink<T>>, prev: WeakLink<T>) -> RefCell<DLNode<T>> {
        RefCell::new(DLNode { next, prev, val: Some(val) })
    }

    fn empty() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            RefCell::new(DLNode {
                next: None,
                prev: me.clone(),
                val: None,
            })
        })
    }
}

struct DList<T> {
    root: StrongLink<T>,
}
// ANCHOR_END: here

impl<T: std::fmt::Display + std::fmt::Debug> fmt::Display for DList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut curr = Rc::clone(self.root.borrow().next.as_ref().unwrap());
        loop {     
            let tmp = curr;
            match &tmp.borrow().val {
                Some(val) => {
                    write!(f, "{} ", val)?;
                    // This is to check that we haven't done excessive cloning
                    let count = Rc::strong_count(&tmp);
                    if count > 2 {
                        write!(f, "strong count {}! ", count)?;
                    }
                },
                None => {
                    break;
                },
            };
            curr = Rc::clone(&tmp.borrow().next.as_ref().unwrap());
        }
        write!(f, "]")
    }
}  

impl<T: std::default::Default + std::clone::Clone> DList<T> {
    fn new() -> DList<T> {
        DList {
            root: {
                let node = DLNode::<T>::empty();
                node.borrow_mut().next = Some(Rc::clone(&node));
                node
            }
        }
    }

// ANCHOR: pushpop
    fn push(&mut self, val: T) {
        // create the new node; node->prev = root
        let node = Rc::new(DLNode::new(val, None, Rc::downgrade(&self.root)));

        let node_clone = Rc::clone(&node);
        // next = root->next; root->next = node; 
        let next = mem::replace(&mut self.root.borrow_mut().next, Some(node));

        // next->prev = node
        next.as_ref().unwrap().borrow_mut().prev = Rc::downgrade(&node_clone);

        // node->next = next
        node_clone.borrow_mut().next = next;
    }

    fn pop(&mut self) -> Option<T> {
        // if root->next->val == None -> empty list!
        if self.root.borrow().next.as_ref().unwrap().borrow().val.is_none() {
            return None
        }

        // node = root->next
        let node = mem::take(&mut self.root.borrow_mut().next).unwrap();

        // val has to be copied out of node because it is about to be dropped
        let val = node.borrow().val.clone();

        // next = node->next
        let next_owner = Rc::clone(&node.borrow().next.as_ref().unwrap());

        // next->prev = node->prev (= root)
        mem::swap(&mut next_owner.borrow_mut().prev, &mut node.borrow_mut().prev);
        // root->next = node->next
        self.root.borrow_mut().next = Some(next_owner);

        val
    }
// ANCHOR_END: pushpop
}

fn main() {
    let mut stack = DList::new();

    println!("size {} ", mem::size_of::<DLNode<i32>>());
    println!("empty stack {}", stack);
    stack.push(10);
    println!("first stack {}", stack);
    stack.push(2);
    println!("second stack {}", stack);
    stack.push(42);
    println!("last stack {}", stack);

    let mut v = stack.pop();
    println!("first pop {}, stack {}", v.unwrap(), stack);
    v = stack.pop();
    println!("next pop {}, stack {}", v.unwrap(), stack);
}
