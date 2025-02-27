// declaring a variable

fn main() {
  let string = "StringSTRING"; //str
  let ratio_float = 4.5;
  let is_boolean = true;
  let ic_char = "#";
// A String has three parts:
    // 1.i A pointer to the heap memory
    // 2. A length
    // 3. A capacity
    // These three values are stored on the stack, but the actual string data
    // is on the heap
    let s = String::from("hello");

  println!("book name is : {}", string);
  println!("company rating on 6 is: {} ",ratio_float);
  println!("i dont know rust: {}",is_boolean);
  println!("use this hastag: {}",ic_char);

}