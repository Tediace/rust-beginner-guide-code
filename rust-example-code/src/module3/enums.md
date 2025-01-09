## Enums and Pattern Matching
Enums allow you to define a type by enumerating its possible *variants*. First we'll define and use an enum to show how
an enum can encode meaning along with data.

### Defining an Enum
Enum is value one of a possible set of value.
```rust
enum IpAddrKind{
    v4,
    v6,
}
```
IpAddrKind is now a custom data type that we can use elsewhere in our code.

### Enum Values
Create instances of two variants:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
Example enum Storing the data and IpAddrKind variant of an IP address using a struct
```rust
 enum IpAddrKind {
    V4,
    V6,
 }
 struct IpAddr {
     kind: IpAddrKind,
     address: String,
 }
 let home = IpAddr {
     kind: IpAddrKind::V4,
     address: String::from("127.0.0.1"),
 };
 let loopback = IpAddr {
     kind: IpAddrKind::V6,
     address: String::from("::1"),
 };
```
Example of enum with variety of types:
```rust
enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Write(String),
    ChangedColor(i32, i32, i32),
}
```
This enum four variants with different types:
1. **Quit** has no data associated with it at all.
2. **Move** has named fields, like a struct does.
3. **Write** includes a single String.
4. **ChangeColor** includes three i32 values.

