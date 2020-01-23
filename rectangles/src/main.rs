// https://doc.rust-lang.org/book/ch05-00-structs.html

fn main() {
    // strategy 1: plain variables

    let width = 30;
    let height = 50;

    println!(
        "The area of the {}x{} rectangle is {} square pixels",
        width, height, area(width, height)
    );

    // strategy 2: tuples

    let rect = (20, 40);

    println!(
        "The area of the {}x{} rectangle is {} square pixels",
        rect.0, rect.1, area2(rect)
    );

    // strategy 3: structs

    let rect1 = Rectangle { width: 33, height: 45 };

    println!(
        "The area of the {:?} rectangle is {} square pixels",
        rect1, rect1.area()
    );

    let rect2 = Rectangle { width: 25, height: 40 };
    let rect3 = Rectangle { width: 50, height: 30 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // associated functions are not methods, they don't have access to self
    println!("Square! {:#?}", Rectangle::square(12));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1
}

#[derive(Debug)] // this allow us to println
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let actual = area(2, 3);
        assert_eq!(actual, 6);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }
}
