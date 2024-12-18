pub fn references_borrowing() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("The Result is {}", word);
    // The Result is 5
}

pub fn first_word(s: &String) -> usize {
    // as_bytes is changes string into slice array
    // hello_world (string), 012345678910 (slice array)
    let bytes = s.as_bytes();

    // enumerate added index (i) each element of iteration
    // iter given reference (&u8) in slice byte
    // because iter give reference of value string (hello world)
    // we need to dereference them using & (&item)
    // or manually (item), let item = *item
    for (i, &item) in bytes.iter().enumerate() {
        // b ' ' is byte literal in Rust and represent of character space
        if item == b' ' {
            return i; // if condition met get value from return i
            // 5
        }
    }

    s.len()
    // get all string if no return value from i
}