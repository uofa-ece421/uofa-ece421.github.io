// ANCHOR: test
#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}
// ANCHOR_END: test

// ANCHOR: swap
fn main() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("before swap: &test1 {:p}, size {}", &test1, std::mem::size_of::<Test>());

    println!("test1: a: {} {:p}, b: {} {:p}", test1.a(), &test1.a, test1.b(), test1.b);
    println!("test2: a: {} {:p}, b: {} {:p}", test2.a(), &test2.a, test2.b(), test2.b);
    
    std::mem::swap(&mut test1, &mut test2);

    println!("after swap &test2 {:p}, size {}", &test2, std::mem::size_of::<Test>());

    //test1.a = "Surprise!".to_string();
    println!("test1: a: {} {:p}, b: {} {:p}", test1.a(), &test1.a, test1.b(), test1.b);
    println!("test2: a: {} {:p}, b: {} {:p}", test2.a(), &test2.a, test2.b(), test2.b);
}
// ANCHOR_END: swap
