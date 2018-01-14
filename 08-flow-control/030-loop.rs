fn main() {
  let mut count = 0u32;
  println!("Let's count to infinity");

  loop {
    count += 1;
    if count == 3 {
      println!("three");
      continue;
    }

    println!("{}", count);
    if count == 1020 { // lines my terminal saves (1025)
      println!("OK, that's enough");
      break;
    }
  }
}