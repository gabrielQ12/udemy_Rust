fn main() {
  let a = String::from("Hello");
  let clos = move || println!("{}", a);
  clos();
  println!("{}", a);
}

