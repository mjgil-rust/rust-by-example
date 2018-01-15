fn main() {
  // .iter() borrows a reference to elements 
  // in the array
  let names1 = vec!["Bob", "Frank", "Ferris"];
  for name in names1.iter() {
    match name {
      &"Ferris" => println!("There is a rustacean among us!"),
      _ => println!("Hello {}", name),
    }
  }
  println!("{:?}", names1);


  // .into_iter() borrows a copy of the array
  // and moves it into the loop
  let names2 = vec!["Bob", "Frank", "Ferris"];
  for name in names2.into_iter() {
    match name {
      "Ferris" => println!("There is a rustacean among us!"),
      _ => println!("Hello {}", name),
    }
  }
  // error: name2 moved into for loop
  // println!("{:?}", names2);
  

  // .iter_mut() borrows a mutable reference
  // to elements in the array for modification
  let mut names3 = vec!["Bob", "Frank", "Ferris"];
  for name in names3.iter_mut() {
    match name {
      &mut "Ferris" => println!("There is a rustacean among us!"),
      _ => println!("Hello {}", name),
    }
  }
  println!("{:?}", names3);
}