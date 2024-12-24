// Defining an Enum
//pub fn pattern_matching(){
//    println!("enums");
//}

/*
enum IpAddrKind{
    V4(String),
    V6(String),
}

let home = IpAddrKind::V4(String::from("127.0.0.1"));

let loopback = IpAddrKind::V6(String::from("::1"));
*/

/*
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}
 */

/*
let home = IpAddr{
    kind: IpAddKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr{
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
 */

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // has no data associated with it at all
    Move { x: i32, y: i32 },    // has named fields, like a struct does
    Write(String),              // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values
}

// define with struct
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

pub fn pattern_matching() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

// The option enum and its advantages over null values


// Concept value being present or absent in Rust (Option<t>)

enum Option<T>{
    None,
    Some(T),
}

pub fn pattern_matching_options() {
    let some_number = Some(5);
    let some_string = Some("a");
    let absent_number: core::option<i32> = None;
}








