mod 
fn main() -> int{
  let x:i32 = 25;
  let active = true;
  while (active == true){
      let x = run(x);
      if(x == false){
        active = false;
        }
        else{
        active = true;
        }
    }
  println!("we have ended");
  return 0;
}
