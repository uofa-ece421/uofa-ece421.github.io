use std::ptr;
use std::fmt;

// ANCHOR: drop
struct Token<T: std::fmt::Display> {
    token: Option<T>,
}

impl<T: std::fmt::Display> Token<T> {
    fn new(token: Option<T>) -> Self {
        match &token { 
            Some(t) => println!("NEW token {}", t),
            None => println!("NEW token root"),
        }
        Self { token }
    }
}

impl<T: std::fmt::Display> Drop for Token<T> {
    fn drop(&mut self) {
        match &self.token {
            Some(t) => println!("DROP token {}", t),
            None => println!("DROP root!"),
        }
    }
}
// ANCHOR_END: drop

// ANCHOR: here
struct Node<T: std::fmt::Display> {
    next: Link<T>,
    prev: Link<T>,
    elem: T,
    token: Token<T>,
}

type Link<T> = *mut Node<T>;

impl<T: Default + Clone + std::fmt::Display> Node<T> {
    fn new(elem: T, next: Link<T>, prev: Link<T>) -> Self {
        Self { next, prev, elem: elem.clone(), token: Token::new(Some(elem)) }
    }

    fn empty() -> Self {
        Self {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            elem: Default::default(),
            token: Token::new(None),
        }
    }
}

struct DLCList<T: std::fmt::Display> {
    root: Link<T>,
}

impl<T: Clone + Default + std::fmt::Display> DLCList<T> {
    fn new() -> Self {
        Self {
            root: {
                let me = Box::into_raw(Box::new(Node::<T>::empty()));
                unsafe {
                    (*me).next = me;
                    (*me).prev = me;
                    me as Link<T>
                }
            }
        }
    }
// ANCHOR_END: here

// ANCHOR: pushpop
    fn push(&mut self, val: T) {
        let next = unsafe { (*self.root).next };
        let node = Box::into_raw(Box::new(
            Node::new(val, next, self.root))
        );

        unsafe {
            (*next).prev = node; 
            (*self.root).next = node;
        }
    }

    fn pop(&mut self) -> Option<T> {
        let node = unsafe { (*self.root).next };

        if node == self.root {
            return None;
        }

        let val = unsafe { (*node).elem.clone() };
        unsafe {
            let next = (*node).next;
            (*next).prev = self.root;
            (*self.root).next = next;
            drop(Box::from_raw(node));
        }
        Some(val)
    }
// ANCHOR_END: pushpop
}

impl<T: std::fmt::Display> fmt::Display for DLCList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut curr = unsafe { (*self.root).next };
        loop {
            if curr == self.root {
                break;
            }

            let val;
            unsafe {
                val = &(*curr).elem;
                curr = (*curr).next;
            }
            write!(f, "{} ", val)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let mut stack = DLCList::new();

    println!("empty stack {}", stack);
    stack.push(10);
    stack.push(2);
    stack.push(42);
    println!("full stack {}", stack);

    while let Some(val) = stack.pop() {
        println!("popped {}, stack now {}", val, stack);
    }
    println!("empty stack {}", stack);
}
