
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Refactor by using tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle using tuples is {} square pixels.",
        area_tuple(rect1)
    );

    // Calling the Rectangle struct
    let rect1 = Rectangle {
        width: 60,
        height: 100,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
