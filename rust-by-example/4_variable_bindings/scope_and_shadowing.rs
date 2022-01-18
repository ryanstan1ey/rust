fn main() {
 let long_lived_binding = 1;

 {
  let short_lived_binding = 2;
  println!("inner short: {}", short_lived_binding);  
 }

 // doesn't exist in scope
 // println!("outer short: {}", short_lived_binding)

 println!("long lived binding: {}", long_lived_binding);

 // shadowing long_lived_binding
 let long_lived_binding = 3;
 println!("shadowed long lived binding: {}", long_lived_binding);
}