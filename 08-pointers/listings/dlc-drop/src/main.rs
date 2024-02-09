use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::mem;
use std::fmt;

// ANCHOR: drop
struct Token<T: std::fmt::Display> {
    token: Option<T>,
}

impl<T: std::fmt::Display> Token<T> {
    fn new(token: Option<T>) -> Self {
        if let Some(ref t) = token {
            println!("NEW token {}", t);
        }
        Self { token }
    }
}

impl<T: std::fmt::Display> Drop for Token<T> {
    fn drop(&mut self) {
        match &self.token {
            Some(t) => {
                println!("DROP token {}", t);
            }
            None => {
                println!("DROP ROOT!");
            }
        }
    }
}

struct DLNode<T: std::fmt::Display> {
    next: Option<StrongLink<T>>,
    prev: WeakLink<T>,
    val: Option<T>,
    token: Token<T>,
}
// ANCHOR_END: drop

type StrongLink<T> = Rc<RefCell<DLNode<T>>>;
type WeakLink<T> = Weak<RefCell<DLNode<T>>>;

impl<T: std::fmt::Display + Clone> DLNode<T> {
    fn new(val: T, next: Option<StrongLink<T>>, prev: WeakLink<T>) -> RefCell<DLNode<T>> {
        RefCell::new(DLNode { next, prev,
                              val: Some(val.clone()),
                              token: Token::new(Some(val))
        })
    }

    fn empty() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            RefCell::new(DLNode {
                next: None,
                prev: me.clone(),
                val: None,
                token: Token::new(None),
            })
        })
    }
}

struct DList<T: std::fmt::Display> {
    root: StrongLink<T>,
}

// ANCHOR: display
impl<T: std::fmt::Display> fmt::Display for DList<T> {
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
// ANCHOR_END: display

impl<T: std::fmt::Display + std::clone::Clone> DList<T> {
    fn new() -> DList<T> {
        DList {
            root: {
                let node = DLNode::<T>::empty();
                node.borrow_mut().next = Some(Rc::clone(&node));
                node
            }
        }
    }

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

    fn pop(&mut self) -> T {
        // node = root->next
        let node = mem::take(&mut self.root.borrow_mut().next).unwrap();

        // val has to be copied out of node because it is about to be dropped
        let val = node.borrow().val.as_ref().unwrap().clone();

        // next = node->next
        let next_owner = Rc::clone(&node.borrow().next.as_ref().unwrap());

        // next->prev = node->prev (= root)
        mem::swap(&mut next_owner.borrow_mut().prev, &mut node.borrow_mut().prev);
        // root->next = node->next
        self.root.borrow_mut().next = Some(next_owner);

        val
    }
}

fn main() {
    let mut stack = DList::new();

    println!("empty stack {}", stack);
    stack.push(10);
    println!("first stack {}", stack);
    stack.push(2);
    println!("second stack {}", stack);
    stack.push(42);
    println!("last stack {}", stack);

    let mut v = stack.pop();
    println!("first pop {}, stack {}", v, stack);
    v = stack.pop();
    println!("next pop {}, stack {}", v, stack);
}
