fn main() {
  let a_binding;

  {
    let x = 2;
    a_binding = x * x;
  }

  println!("a binding: {}", a_binding);
  // this is an error if it's never initialized
  let another_binding;

  // println!("another binding: {}", another_binding);
  another_binding = 1;
  println!("another binding: {}", another_binding);
}