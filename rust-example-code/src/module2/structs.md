## Using Structs to Structure Related Data
A *struct*, or *structure*, is a custom data type that lets you package together and name multiple related value that
make up meaningful group. It's like object-oriented language, a *struct* is like an object's data attributes.

### Defining and Instantiating Structs
Structs in programming are similar to tuples because they group multiple related values, which can be of different 
types. However, unlike tuples, structs provide named fields for each value, making it easier to understand and access 
the data without relying on its order. Structs are defined using the struct keyword followed by a descriptive name and 
curly brackets containing the field names and their types. This naming flexibility makes structs more versatile than 
tuples. For example, a struct could be used to represent a user account by clearly naming fields such as username and 
email.
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
To use a struct, you create an instance by providing specific values for its fields. This involves stating the struct's 
name and using curly brackets to define key-value pairs, where the keys are the field names and the values are the 
corresponding data. The order of the fields in the instance doesn't need to match their order in the struct definition. 
The struct serves as a template, and each instance populates it with unique data, creating values of the defined type.

Example declare a particular user:
```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("username@example.com"),
        sign_in_count: 1,
    };
}
```
To get a specific value from a struct, we use dot notation. For example, to access this user’s
email address, we use user1.email . If the instance is mutable, we can change a value by
using the dot notation and assigning into a particular field. 

Example how to change the value in the *email* field of a mutable *User* instance:
```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("username@example.com"),
        sign_in_count: 1,
    };
    
    user.email = String::from("anotheruser@example.com");
}
```
Note that the entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable. As with any 
expression, we can construct a new instance of the struct as the last expression in the function body to implicitly 
return that new instance.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
 }
```
*build_user* function that returns a *User* instance with the given email and username. The *active* field gets the 
value of *true*, and the *sign_in_count* gets a value of 1.

It makes sense to name the function parameters with the same name as the struct fields,
but having to repeat the email and username field names and variables is a bit tedious. If
the struct had more fields, repeating each name would get even more annoying. Luckily,
there’s a convenient shorthand!

#### Using the Field Init Shorthand
We can use the *field init shorthand* syntax to rewrite *build_user* so it behaves exactly the same but doesn't have the
repetition of *username* and *email*.
```rust
 fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
 }
```
Because the *email* parameter have the same name, we only need to write *email* rather than *email: email*.

#### Creating Instances from Other Instances with Struct Update Syntax
using *struct update syntax*  useful to create new instance of a struct that includes most of the values from another
instance.

```rust
fn main() {
 // --snip-
let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
 }
```
Or we can do more simple with:
```rust
 fn main() {
 // --snip-
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
    };
 }
```
To create a new struct instance (user2) that shares some values with an existing instance (user1), you can specify new 
values for certain fields and use the ..user1 syntax to copy the remaining fields. The ..user1 must be placed last and 
copies values from user1 for any unspecified fields. This syntax reuses data efficiently but moves ownership of fields 
like String, making user1 unusable if its fields are moved into user2. However, if only fields with types implementing 
the Copy trait are reused (e.g., integers), user1 remains valid after creating user2.

#### Using Tuple Structs Without Named Fields to Create Different Types
Rust supports tuple structs, which combine the simplicity of tuples with the added meaning provided by a struct name. 
Unlike regular structs, tuple structs don’t assign names to their fields, only types. This makes them useful when naming
individual fields would be unnecessary or verbose. Tuple structs give the tuple a unique type distinct from other 
tuples. They are defined using the struct keyword, followed by the struct name and the types of the fields in 
parentheses.

Example tuple structs:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main(){
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
In Rust, each tuple struct you define is its own unique type, even if the fields have the same types. For example, a 
Color struct and a Point struct can both contain three i32 values, but they are treated as distinct types. A function 
expecting a Color parameter cannot accept a Point. Like tuples, tuple struct instances can be destructured into 
individual values or accessed by index using the . operator. This combination of type uniqueness and tuple-like 
functionality adds flexibility and clarity to the code.

#### Unit-Like Structs Without Any Fields
You can also define structs that don't have any fields! These are called *unit-like structs*. This is useful when you 
need to implement a trait on some type but don't have any data you want to store in the type itself.
```rust
struct AlwaysEqual;

fn main{
    let subject = alwaysEqual;
}
```

#### Ownership to Struct Data
It’s also possible for structs to store references to data owned by something else, but
to do so require the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10.
Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
Let’s say you try to store a reference in a struct without specifying lifetimes, like the
following; this won’t work:

```rust
struct User {
       active: bool,
       username: &str,
       email: &str,
       sign_in_count: u64,
 }
 fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
 }
```
The compiler will complain that it needs lifetime specifiers:
```text
 $ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
 error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
   |
 3 |     username: &str,
   |               ^ expected named lifetime parameter
   |
 help: consider introducing a named lifetime parameter
   |
 1 ~ struct User<'a> {
 2 |     active: bool,
 3 ~     username: &'a str,
   |
 error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
   | 
 4 |     email: &str,
   |            ^ expected named lifetime parameter
   |
 help: consider introducing a named lifetime parameter
   |
 1 ~ struct User<'a> {
 2 |     active: bool,
 3 |     username: &str,
 4 ~     email: &'a str,
   |
 For more information about this error, try `rustc --explain E0106`.
 error: could not compile `structs` due to 2 previous errors
```
For now, we'll fix errors like these using owned types like *String* instead of references like *&str*.

### An Example Program Using Structs
Calculating the area of a rectangle specified by separate width and height variables.
```rust
 fn main() {
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
```
Run program using *cargo*:
```text
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/rectangles`
 The area of the rectangle is 1500 square pixels.
```
#### Refactoring with Tuples
```rust
//  Specifying the width and height of the rectangle with a tuple
fn main() {
    let rect1 = (30, 50);
    println!(
    "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
 }
 fn area(dimensions: (u32, u32)) -> u32 {
     dimensions.0 * dimensions.1
 }
```
In one way, this program is better. Tuples let us add a bit of structure, and we’re now
passing just one argument. But in another way, this version is less clear: tuples don’t name
their elements, so we have to index into the parts of the tuple, making our calculation less
obvious.

#### Refactoring with Structs: Adding More Meaning
We can transform the tuple we're using into a struct with a name for the whole as well as names for the parts.
```rust
// defining a rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
 }
 fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
    "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
 }
 fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
 }
```
Our function signature for area now says exactly
what we mean: calculate the area of Rectangle , using its width and height fields. This
conveys that the width and height are related to each other, and it gives descriptive names
to the values rather than using the tuple index values of 0 and 1 . This is a win for clarity.

#### Adding Useful Functionality with Derived Traits
It'd be useful to be able to print an instance of *Rectangle* while we're debugging our program and see the values for
all its fields.

```rust
// Attempting to print a rectangle instance
 struct Rectangle {
    width: u32,
    height: u32,
 }
 fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
 println!("rect1 is {}", rect1);
 }
```
This code will compile with error message:
```text
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for 
pretty-print) instead
```
Putting the specifier **:?** inside the curly brackets tells **println!** we want to use an output format called
**Debug**. The **Debug** trait enables us to print out struct in a wat that is useful for developers. But, we still
get an error:
```text
error[E0277]: `Rectangle` doesn't implement `Debug`
= help: the trait `Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```
We need add the outer attribute **#[drive(Debug)]** just before the struct definition.
```rust
//  Adding the attribute to derive the Debug trait and printing the Rectangle instance using debug formatting
#[derive(Debug)]
 struct Rectangle {
    width: u32,
    height: u32,
 }
 fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
 }
```
The result:
```text
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
 rect1 is Rectangle { width: 30, height: 50 }
```
Another way to print out a value using the **Debug** format is to use the **dgb! macro**.
```rust
 #[derive(Debug)]
 struct Rectangle {
    width: u32,
    height: u32,
 }
 fn main() {
     let scale = 2;
     let rect1 = Rectangle {
         width: dbg!(30 * scale),
         height: 50,
    };
     
    dbg!(&rect1);
 }
```
The result:
```text
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
 [src/main.rs:10] 30 * scale = 60
 [src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
 }
```
The dbg! macro helps debug code by printing the value of an expression and its location in the source file. 
In this case:
- The first dbg! call outputs the result of 30 * scale (value: 60), formatted as a simple integer.
- The second dbg! call outputs &rect1, using pretty Debug formatting for the Rectangle struct.

This macro is especially useful for understanding how your code behaves during execution.

### Method Syntax











