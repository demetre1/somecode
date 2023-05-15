fn add(num_one: i32, num_two:i32)->i32{
    num_one + num_two

}

fn main() {
  let mut total = add(15,5);
  let mut free_shipping = false;

  if total > 50 {
    println!("You are qulified for free shipping");
    free_shipping = true;
  }
  else if total > 20 {
    println!("if you add mroe items, you can be qulify for free shippng");
  }
  else {
    println!("No Free Shipiing")
  }

  total = match(free_shipping){
    true =>  total + 0,
    false =>  total + 5,
  };
  match total { 
    1 => println!("1"),
    2 => println!("2"),
    3 => println!("3"),
    _ => println!("no match found!")
  } 
  
  println!(" {:?}  ", total);
}
