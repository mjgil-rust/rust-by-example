use std::convert::From;

#[derive(Debug)]
struct Number {
  value: i32,
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number { value: item }
  }
}

fn main() {
  let from_num = Number::from(30);
  println!("My number is {:?}", from_num);

  let int = 5;
  let into_num: Number = int.into();
  println!("My number is {:?}", into_num);

  let my_str = "hello";
  let my_string = String::from(my_str);
  println!("my_string: {:?}", my_string)
}