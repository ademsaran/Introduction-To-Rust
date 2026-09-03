// A generic function that can find the largest item in a list of any type T. Must implement the PartialOrd trait to compare items. 
// i32 and char implement PartialOrd, so we can use this function with those types.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// A generic struct that can hold any type T, both fields must be of the same type T
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Implementing methods on a generic struct. 
// The method x() returns a reference to the x field of the Point struct.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[derive(Debug)]
struct PointDifferentTypes<X1, Y1> {
    x: X1,
    y: Y1,
}

// Implementing methods on a generic struct with different types for each field.
impl<X1, Y1> PointDifferentTypes<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointDifferentTypes<X2, Y2>) -> PointDifferentTypes<X1, Y2> {
        PointDifferentTypes {
            x: self.x,
            y: other.y,
        }
    }
}

// Lifetime annotations in function signatures. The longest function takes 
// two string slices and returns the longest of the two. The lifetime 'a is 
// used to indicate that the returned reference will live at least as long 
// as the shortest of the input references.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// A struct that holds a reference to a string slice with a lifetime 'a.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // First elision rule
    fn level(&self) -> i32 {
        3
    }
    // Third elision rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// Combining Generic Type Parameters, Trait Bounds, and Lifetimes
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // Generic functions
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];

        // Generic function.
        let result = largest(&char_list);
        println!("The largest char is {result}");

        // Generic struct.
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };

        // Generic struct with different types for each field.
        let both_integer = PointDifferentTypes { x: 5, y: 10 };
        let both_float = PointDifferentTypes { x: 1.0, y: 4.0 };
        let integer_and_float = PointDifferentTypes { x: 5, y: 4.0 };

        dbg!("integer: {:?}", integer);
        dbg!("float: {:?}", float);
        dbg!("both_integer: {:?}", both_integer);
        dbg!("both_float: {:?}", both_float);
        dbg!("integer_and_float: {:?}", integer_and_float);

        // Using the method x() on the Point struct.
        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());

        // Using generic types that are different from its struct’s definition
        let p1 = PointDifferentTypes { x: 5, y: 10.4 };
        let p2 = PointDifferentTypes { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    // Lifetimes
    {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {result}"); // Code will compile.
        }

        // The main function here creates an instance of the ImportantExcerpt struct 
        // that holds a reference to the first sentence of the String owned by the 
        // variable novel. The data in novel exists before the ImportantExcerpt instance
        //  is created. In addition, novel doesn’t go out of scope until after the 
        // ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt 
        // instance is valid.
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        // Static lifetime. A reference with a 'static lifetime can live for the entire 
        // duration of the program. All string literals have a 'static lifetime, 
        // which means that they are stored in the binary of the program and
        //  are available for the entire duration of the program.
        let s: &'static str = "I have a static lifetime.";
    }

    // Combining Generic Type Parameters, Trait Bounds, and Lifetimes
    {
        let string1 = String::from("long string is long");
        let string2 = String::from("xyz");

        let result = longest_with_an_announcement(
            string1.as_str(),
            string2.as_str(),
            "This is an announcement.",
        );
        println!("The longest string is {result}");
    }
}