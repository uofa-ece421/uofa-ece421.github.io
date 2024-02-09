# Interior Mutability

Rust memory safety is based on two rules:
* An object `T` may have multiple _immutable_ references (`&T`)
* *OR* an object may have exactly one _mutable_ reference (`&mut T)

Sometimes you just need mutiple references that allow the object to be mutated.
This is not necessarily bad, it is just difficult for the compiler to *prove*
memory safety.

Shareable mutable containers exist to permit controlled mutability, even in the
presence of multiple references.

* Single-threaded
  - `std::cell:Cell<T>`, [std::cell::Refcell](https://doc.rust-lang.org/std/cell/struct.RefCell.html), and `OnceCell<T>`
* Muti-threaded
  - `Mutex<T>`, `RwLock<T>`, and `OnceLock<T>`

These [std::cell](https://doc.rust-lang.org/std/cell/index.html) types provide
_interior mutability_ (mutable via `&T`), compared to most Rust types that
exhibit _inherited mutability_ (mutable only via `&mut T`).

### Cell

Implements interior mutability by moving values into and out of the cell,
i.e. you can `get` a copy of the object or you can `set` or `replace` the object
with a new one.

```rust
use std::cell::Cell;

struct Widget {
    model: String,
    version: (u8, u8, u8),
    in_stock: Cell<bool>,
}

fn main() {
    let product = Widget {  // Product is declared immutable here
        model: "Super Thing".to_string(),
        version: (0, 9, 123),
        in_stock: Cell::new(true),
    };

    // Finally sold it!
    product.in_stock.set(false); // Tee-hee! We changed a field anyway!
}
```

### RefCell

`Box<T>` represents single ownership and inheritance mutability that is enforced
at _compile_ time. `RefCell<T>` supports interior mutability that is checked
at _runtime_.

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct EnterExitCounts { enter: u32, exit: u32 }

impl EnterExitCounts {
    fn new() -> Self {
        Self { enter: 0, exit: 0 }
    }
}

struct House {
    doors: [EnterExitCounts; 2] // Front and back doors in this house
}

fn main() {
    // Immutable house_visitors owns the House on the heap
   let house_visitors = Rc::new(RefCell::new(
        House { doors: [EnterExitCounts::new(), EnterExitCounts::new()] }
   ));

   // The outside watcher needs to record what they see
   let outside_watcher = Rc::clone(&house_visitors);

   outside_watcher.borrow_mut().doors[0].enter = 1; // See a person enter the front
   outside_watcher.borrow_mut().doors[1].exit = 1; // and leave by the back

   let visitors = house_visitors.borrow();
   let inside = visitors.doors[0].enter - visitors.doors[0].exit
              + visitors.doors[1].enter - visitors.doors[1].exit;
   println!("{} people currently inside", inside);
}
```       
