pub fn references_borrowing() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("The Result is {}", word);
    // The Result is 5
}

pub fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // if condition met get value from return i
            // 5
        }
    }

    s.len()
    // get all string if no return value from i
}