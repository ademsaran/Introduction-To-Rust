#[derive(Debug)] // The #[derive(Debug)] attribute automatically creates an implementation of the Debug trait for the Rectangle struct, allowing us to print instances of Rectangle using the {:?} or {:#?} format specifiers.
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { //Method (that takes a reference to self as a parameter)
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool { //Method that takes a reference to another Rectangle as a parameter
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self { //Associated function, not a method because it doesn't take self as a parameter
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    {
        let width1 = 30;
        let height1 = 50;

        let rect1 = (30, 50);

        let rect2 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1),
        );

        println!(
            "The area of the rectangle is {} square pixels.",
            area_tuple(rect1),
        );

        dbg!(&rect2);

        println!(
            "Rect 2 is {rect2:#?}. The area of the rectangle is {} square pixels. The implemented area method returns {} square pixels.",
            area_rectangle(&rect2),
            rect2.area(),
        );
    }
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }
    {
        let square = Rectangle::square(5);
        dbg!(&square);
        println!("A square with side length 5 has area {}.", square.area());
    }

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}