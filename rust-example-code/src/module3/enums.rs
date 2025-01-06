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
use core::option;

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
 // The match control flow construct
/*
enum Coin{
     Penny,
     Nickel,
     Dime,
     Quarter,
 }
 */

/*
fn value_in_cents(coin: Coin) -> u8 {
    // An enum and a match expression that has the variants of the enum as its patterns
    // use curly brackets to run multiple lines in a match arm
    /*
    Coin::Penny => {
        Println!("Lucky penny!");
        1
    }
     */
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
 */
// Pattern that bind to values
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

pub(crate) enum Coin {
    // coin enum in which the quarter variant also hold UsState value
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>
pub fn plus_one(x: Option<i32>) -> core::option::Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


// Catch-all patterns and the_Placeholder
pub fn catch_patterns(){
    let dice_role = 9;
    match dice_role {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other), // catch value and save into fn move_player
        //_ => reroll(), // ignore value other than 3 or 7
        _ => (), // express that by using the unit value
    }
}

fn add_fancy_hat(){}
fn remove_fancy_hat(){}
// fn move_player(num_spaces: u8){}
// fn reroll(){}

// Concise control flow with if let
pub fn control_flow(){
    /*
    // a match that only cares about executing code when the value is some
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
     */
    // shorter way using if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}






