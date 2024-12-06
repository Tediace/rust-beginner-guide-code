mod functions;

fn main() {
    // VARIABLE AND MUTABILITY
    // every variable in rust is immutable by default
    // add mut for mutable
    let mut x= 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);

    // CONSTANTS
    // rust naming convention for constants is to use all uppercase with underscore between words
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60;
    println!("the value of x is {}", THREE_HOURS_IN_SECONDS);

    // SHADOWING
    // shadowed allowing created new variable with same name as previous
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x is {}", x);
        // 12
    }
    // the value back to 6 because out of scope
    println!("the value of x is {}", x);
    // shadowing is different from marking a var as mut
    // we can change the type of the value but reuse the same name
    let spaces = "   ";
    let spaces = spaces.len();
    println!("the spaces is {}", spaces);
    // 3
    functions::another_function(5, 't');
    // The measurement is: 5 and t
    functions::statements_expressions();

    let x = plus_one(4);
    println!("the value of x is {}", x);
}
// Function with return value
// Function can return values to the code that calls them
// We don't name return values, but we must declare their type after an arrow(->)
// In Rust, the return value of the function is synonymous with the value of the final expression
// You can return function by using the return keyword, but
// Most function return the last expression implicitly
fn plus_one(x: i32) -> i32 {
    x + 1
}
