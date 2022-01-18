fn main() {
 let a_binding;
 {
  let x = 2;
  a_binding = x*x;
 }

 println!("a binding: {}", a_binding);

 let another_binding;
 // error! use of uninitialized binding
 // println!("another binding: {}", another_binding);

 another_binding = 2;
 println!("another binding: {}", another_binding);
}