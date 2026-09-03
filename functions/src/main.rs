fn main() {
    println!("Hello, world!");

    another_function(5, 'm');
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The unit label is: {}", unit_label);
}