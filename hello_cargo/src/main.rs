fn main() {
    println!("Hello, world!\n");

    println!(
        "Some cargo commands!
    cargo new project_name - sets up a project directory for rust.
    cargo build - builds the executable
    cargo check - checks code for compiler errors
    cargo run - runs executable, builds if no executable\n"
    );

    println!(
        "Releasing code
    cargo build --release - compiles with optimizations.
    Can be found in target/release instead of target/debug.\n
    "
    );
}
