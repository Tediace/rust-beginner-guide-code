pub fn repetition() {
    // Returning value from Loops
    loop {
        println!("again!");
        break; // stop executing loop
    }

    let mut count = 0;

    let result = loop {
        count += 1;
        // == is equal
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is {}", result);
    // The result is 20

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0; // shadow count
    // The outer loop has the label counting_up, and it will count up from 0 to 2
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
    /*
    count = 0
    remaining = 10
    remaining = 9
    count = 1
    remaining = 10
    remaining = 9
    count = 2
    remaining = 10
    End count = 2
     */

    // Conditional loops with while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
    /*
    3!
    2!
    1!
    LIFTOFF!!!
    */

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    // This approach is can cause an error
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    /*
    the value is: 10
    the value is: 20
    the value is: 30
    the value is: 40
    the value is: 50
    */
    // As a more concise alternative, Use syntax for loop
    // .iter() is reading elements as references
    let b = [10, 20, 30, 40, 50];
    for element in b.iter() {
        println!("the value is: {}", element);
    }
    /*
    the value is: 10
    the value is: 20
    the value is: 30
    the value is: 40
    the value is: 50
    */
    // for loop with moved array to loop so if loop done no longer use
    /*
    for element in b {
        println!("{}!", element);
    }
    */

    // Mostly Rustaceans would use a for loop
    // (1..4) indicated index 1 - 3
    // .rev() Reversing the order of ranges
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    /*
    3!
    2!
    1!
    LIFTOFF!!!
    */
}