# Lifetimes for Structs

Note the generics annotation syntax is now being used for lifetimes

```rust
#[derive(Debug)]
// The struct containing the reference has to live at least as long as the reference
struct MenuItem<'item> {
     name: &'item str,
     price: f32,
}

// We're just referencing name, so an anonymous lifetime is sufficient
impl MenuItem<'_> {
     fn display(&self) {
         println!("Enjoy {} for only ${:.2}", self.name, self.price);
     }
}

fn main() {
     let items = vec![String::from("Steak"), String::from("Hot Dog")];
     let steak = MenuItem { name: &items[0], price: 5.0E1 };

     steak.display();
}
```

```rust
#[derive(Debug)]
struct MenuItem<'item> {
     name: &'item str,
     price: f32,
}

impl<'item> MenuItem<'item> {
     fn display(&self) {
         println!("Enjoy {} for only ${:.2}", self.name, self.price);
     }

     // This uses lifetime coercion to say lifetime 'b is at least as long as 'a 
     fn substitute<'b: 'item>(& mut self, new_item: &'b str) {
        self.name = new_item;
     }
}

fn nice_try<'a>(item: &'a mut MenuItem) {
     // let fast_food = String::from("Fast food");
     let fast_food = "Fast food";
     item.substitute(&fast_food);
}

fn main() {
     let items = vec![String::from("Steak"), String::from("Hot Dog")];
     let mut steak = MenuItem { name: &items[0], price: 5.0E1 };

     nice_try(&mut steak);
     
     steak.substitute(&items[1]);
     steak.display();
}
```
