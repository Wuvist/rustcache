struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&mut rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.width
    );
}

fn area(rectangle: &mut Rectangle) -> u32 {
    rectangle.width = 10;
    return rectangle.width * rectangle.height
}
