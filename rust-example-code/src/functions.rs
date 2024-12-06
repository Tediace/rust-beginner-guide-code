// In function signature, you must declare the type of each parameter
// When defining multiple parameters, separate the parameter declarations with commas
// Statements are instructions that perform some action and do not return a value
// Expressions evaluate to a resultant value
pub fn another_function(x: i32, y: char) {
    println!("The value of x is {} and {}", x, y);
}
// Expressions do not include ending semicolons
// if you add a semicolon to the end an expression, you turn it into statement
pub fn statements_expressions() {
    let y = {
        // this is expression
        let x = 5;
        x + 1
        // this is expression
    };
    println!("The value of y is {}", y);
}