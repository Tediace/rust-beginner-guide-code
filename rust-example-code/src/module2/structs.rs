pub fn structs_data() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
// Refactoring with Tuples
pub fn refactoring_tuples() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        refactored_area(rect1)
    );
}

fn refactored_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Refactoring with structs: Adding more meaning
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn refactored_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        structs_area(&rect1)
    );
}
fn structs_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Adding useful functionality with derived traits
pub fn derived_traits() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
}

pub fn derived_impls() {
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}