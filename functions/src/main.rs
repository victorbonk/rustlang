fn main() {

    // scope blocks
    let y = {
        let x = 3;
        // !important: return value cannot have a semicolon at the end.
        x + 1
    };

    // snake_case for rust functions
    let x = plus_one(5);

    // -> for return type
    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    println!("y is {}", y);
    println!("x is {}", x);
}
