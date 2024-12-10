<div align="center">
  <img src="./img/rustacean-flat-happy.png" alt="Rust Logo" >
</div>

# Rust Sample with Code Explanation
## Data Types
Every value in Rust is of certain *data type*, which tells Rust what kind of data is being specified so it knows how to work with that data.

Rust is *statically types* language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to used based on the value and how we use it.

### Different Type Annotations for other data types
<details>
  <summary>1) Scaler Types </summary>
  
  - **Integer Types**

    An *integer* is a number without a fractional component. 
    
    ![image](https://github.com/user-attachments/assets/7eb12391-3e57-4041-80a0-2ad310acd7e5)
    
    Each variant can be either signed or unsigned and has an explicit size.

    ![image](https://github.com/user-attachments/assets/0588b9e2-2e4e-402e-a940-04a82f1eea5c)

```rust
  fn main() {
       let x = 2.0; // f64
       let y: f32 = 3.0; // f32
  }
  ```
  - **Floating-Point Types**

    Rust's floatin-point type are *f32* and *f64*, which are 32 bits and 64 bits in size, respectively. The default type is *f64* on modern CPUs.
```rust
  fn main() {
 let x = 2.0; // f64
 let y: f32 = 3.0; // f32
 }
 ```
 Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is
 a single-precision float, and f64 has double precision.
  - **Numeric Operations**

     Rust supports the basic mathematical operations you’d expect for all the number types:
 addition, subtraction, multiplication, division, and remainder. Integer division truncates
 toward zero to the nearest integer.
```rust
 fn main() {
 // addition
 let sum = 5 + 10;
 // subtraction
 let difference = 95.5 - 4.3;
 // multiplication
 let product = 4 * 30;
 // division
 let quotient = 56.7 / 32.2;
 let truncated = -5 / 3; // Results in -1
 // remainder
 let remainder = 43 % 5;
 }
```
  - **The Boolean Type**

    As in most other programming languages, a Boolean type in Rust has two possible values: *tru* and *false* With one byte in size.
```rust
fn main() {
 let t = true;
 let f: bool = false; // with explicit type annotation
 }
```
  - **The Character Type**

     Rust's *char* type sis the language's most primitive alphabetic type.
```rust
fn main() {
 let c = 'z';
 let z: char = 'ℤ'; // with explicit type annotation
 let heart_eyed_cat = '�
 ��
 ��
 ��
 �';
 }
```
Rust's *char* type is four bytes in size and represents a Unicode Scalar Value, Which mean it can represent a lot more than just ASCII.
</details>

<details>
  <summary>2) Compound Types </summary>
  
 - **The Tuple Type**

   A *tuple* is a general way of grouping together a number of values. Tuples have a fixed length: they cannot grow or shrink in size. Create a tuple by writing a comma-separated list of values inside parentheses.
```rust
 fn main() {
 let tup: (i32, f64, u8) = (500, 6.4, 1);
 }
```
 The variable tup binds to the entire tuple because a tuple is considered a single compound
 element. To get the individual values out of a tuple, we can use pattern matching to
 destructure a tuple value, like this:
 ```rust
fn main() {
 let tup = (500, 6.4, 1);
 let (x, y, z) = tup;
 println!("The value of y is: {y}");
 }
```
 This program first creates a tuple and binds it to the variable tup . It then uses a pattern
 with let to take tup and turn it into three separate variables, x , y , and z . This is called
 destructuring because it breaks the single tuple into three parts. Finally, the program prints
 the value of y , which is 6.4 .

 We can also access a tuple element directly by using a period ( . ) followed by the index of
 the value we want to access. For example:
 ```rust
fn main() {
 let x: (i32, f64, u8) = (500, 6.4, 1);
 let five_hundred = x.0;
 let six_point_four = x.1;
 let one = x.2;
 }
```
In Rust index in tuple start from 0....n.
 - **The Array Type**

Another way to have a collection of multiple values in with an *array*. Every element of an array must have the same type and have a fixed length.
```rust
fn main() {
 let a = [1, 2, 3, 4, 5];
 }
```
Array are useful when you want data allocated on the stack rather than the heap or when you want to ensure you always have fixed number of elements.

Arrays is good for number will not need to change.  For example, if you were using the names of the month in a program, you would
 probably use an array rather than a vector because you know it will always contain 12
 elements:
 ```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
 "August", "September", "October", "November", "December"];
```
 You write an array’s type using square brackets with the type of each element, a semicolon,
 and then the number of elements in the array, like so:
 ```rust
 let a: [i32; 5] = [1, 2, 3, 4, 5];
```
 Here, i32 is the type of each element. After the semicolon, the number 5 indicates the
 array contains five elements.
 You can also initialize an array to contain the same value for each element by specifying the
 initial value, followed by a semicolon, and then the length of the array in square brackets, as
 shown here:
 ```rust
let a = [3;5];
```
The array named *a* will contain 5 elements that will all be set to the value 3 initially. This is the same as writing **let a = [3, 3, 3, 3, 3];* but in a more concise way.

**Accessing Array Elements**

An array is a single chunk of memory of a known, fixed size can be allocated on the stack. You can access elements of an array using index, like this:
```rust
 fn main() {
 let a = [1, 2, 3, 4, 5];
 let first = a[0];
 let second = a[1];
 }
```
The variable named *first* will get the value *1* because that is the value at index [0] in th array and *second* from index [1] in the array.

**Invalid Array Element Access**

If you try to access an element of an array that is past the end of the array. Run this code:
```rust
 use std::io;
 fn main() {
 let a = [1, 2, 3, 4, 5];
 println!("Please enter an array index.");
 let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
 let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
 let element = a[index];
 println!("The value of the element at index {index} is: {element}");
 }
```
This code compiles successfully. but if you run code *cargo run* and enter a number 10, you'll see output like this:
```rust
 thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 
10', src/main.rs:19:19
 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
The program resulted in a *runtime* error at the point of using invalid in the indexing operation. The program exited with an error message and didn't execute the final *println!* statement. Rust will check index you've specified is less than array length. If the index is greater than or equal to the length, Rust will panic. This happan at runtime, especially in this case, because the compiler can't possibly know that value a user enter when they run the code later.

This is an example of Rust's memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you againts this kind of error immediately exiting instead of allowing the memory access and continuing.
</details>

## Comments

In Rust, comment style starts a comment with two slashes //.

Here's simple comment:

```rust
// hello, world
```
Comments can also be places at the end of lines containing code:
```rust
 fn main() {
    let lucky_number = 7; // I’m feeling lucky today
 }
```
But you'll more often see them used in this format.
```rust
 fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
 }
```
![image](https://github.com/user-attachments/assets/89fbd73c-e352-439b-bf72-b88be69e2214)
source: https://doc.rust-lang.org/book/appendix-02-operators.html


This Rust code and explanation get reference from https://github.com/rust-lang/book/tree/main/src
