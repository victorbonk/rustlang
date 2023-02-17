// Here’s a small programming problem: write a function that takes a string of words separated by
// spaces and returns the first word it finds in that string. If the function doesn’t find a space in
// the string, the whole string must be one word, so the entire string should be returned.

fn main() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' { // b' ' byte of ' '
                return &s[0..i]; // String slice
            }
        }

        &s[..]
    }

    let teststring = String::from("hello man");
    let result = first_word(&teststring);
    println!("result: {}", result);
}
