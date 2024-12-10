// the program just skip if you don't provide an else block and move on to the next code
pub fn if_expression() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("condition was false");
    }

    // if the condition isn't a bool, we'll get an error
    // The error indicates that Rust expected a bool but got an integer
    // Rust will not automatically try to convert non-Boolean type to a Boolean
   /*
    if number {
       println!("number was three");
   }
   */

    // If we want the if code block to run only when a number is not equal to 0
    // != is not equal
    if number != 0 {
        println!("number was something other than zero");
    }
    // % is Arithmetic remainder ( modulus operator that produces the remainder of division )
    // Rust only executes block for the first true condition, it doesn't even check the rest
    // Using to many else can clutter your code, you might want to refactor you code
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {5} else {6}; // if statements must be same type

    println!("The value of number is: {}", number);


}