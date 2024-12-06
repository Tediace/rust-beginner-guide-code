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
  <br>
 * The Tuple Type
  * The Array Type
</details>
     
This Rust code and explanation get reference from https://github.com/rust-lang/book/tree/main/src
