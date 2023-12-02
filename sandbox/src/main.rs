fn main() {
    let vector_of_strings: Vec<String> = vec!["10".to_string(), "20".to_string(), "30".to_string()]; // Example vector of Strings

    // Parse the second string into an integer or provide a default value (in this case, 0)
    let parsed_int = vector_of_strings[1].parse::<i32>().unwrap_or(0);

    println!("Parsed integer: {}", parsed_int);

    println!("{}", vector_of_strings[1]);
}
