use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implementing the Deref trait for MyBox<T>
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

                use std::rc::Rc;
        use crate::List::{Cons, Nil};

fn main() {
    // Using Box<T> to store data on the heap
    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    // Using my own smart pointer MyBox<T> to store data on the heap
    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    // Deref Coercion: Rust automatically converts references to types that implement the Deref trait
    {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
    // Reference counting with Rc<T>: Rc<T> is a reference-counted smart pointer that enables multiple ownership of the same data
    {

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let _b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let _c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}
