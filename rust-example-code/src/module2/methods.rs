// defining methods
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
}

pub fn methods(){
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixes",
        rect1.area()
    );
}

struct Rec {
    width: u32,
    height: u32,
}
impl Rec {
    fn width(&self)->bool{
        self.width > 0
    }
}

pub fn methods2(){
    let rect1 = Rec{
        width: 30,
        height: 50,
    };

    if rect1.width(){
        println!("The Rectangle has a nonzero width: it is {}", rect1.width());
    }
}

struct Rectangle2{
    width: u32,
    height: u32,
}
impl Rectangle2{
    fn area(&self)->u32{
        self.width * self.height
    }

    fn can_hold(&self, another: &Rectangle2)->bool{
        self.width > another.width && self.height > another.height
    }
}
// or we can use associated functions
// it easier to create a square rather than having specify the same value twice
/*
 impl Rectangle {
     fn square(size: u32) -> Self {
         Self {
             width: size,
             height: size,
        }
    }
 }
 */
// methods with more parameters
pub fn methods3(){
    let rect1 = Rectangle2{
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle2{
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle2{
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false
}

// multiple impl blocks
// each struct is allowed to have multiple impl blocks

struct Circle{
    radius: f64,
    diameter: f64,
}

impl Circle{
    fn area(&self) -> f64{
        self.radius * self.diameter
    }
}

impl Circle {
    fn can_hold(&self, other: &Circle)->bool{
        self.radius > other.radius && self.diameter > other.diameter
    }
}
// there's no reason to separate these methods into multiple impl blocks here,
// but this is valid syntax