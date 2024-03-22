#[derive(Debug)]
struct Rectangle { 
    height:u32,
    width:u32
}

fn main() {
    let rect1: Rectangle = Rectangle{
        height:30,
        width:50
    };
    println!("Rectangle: {:#?}", rect1);
    println!("The area of rectange is {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height*rectangle.width
}
