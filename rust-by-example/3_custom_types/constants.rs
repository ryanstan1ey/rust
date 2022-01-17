// Globals

// static: A possibly mutable variable with 'static lifetime. The static 
// lifetime is inferred and doesn't have to be specified. Accessing and modifying
// a mutable static variable is unsafe.
static LANGUAGE: &str = "Rust";

const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
 n > THRESHOLD
}

fn main() {
 let n = 16;

 println!("This is {}", LANGUAGE);
 println!("The threshold is {}", THRESHOLD);
 println!("{} is {}", n, if is_big(n) {"big"} else {"small"});
}