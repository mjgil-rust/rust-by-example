fn main() {
  let an_integer = 1u32;
  let a_boolean = true;
  let unit = ();

  let mut copied_integer = an_integer;

  println!("An integer: {:?}", copied_integer);
  println!("An integer: {:?}", an_integer);

  copied_integer = 2u32;
  println!("An integer: {:?}", copied_integer);
  println!("An integer: {:?}", an_integer);

  println!("A boolean: {:?}", a_boolean);
  println!("Meet the unit vale: {:?}", unit);

  // supress compiler warnings with _ prefix
  let _unused_variable = 3u32;
}