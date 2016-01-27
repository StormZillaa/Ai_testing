mod 
fn main() -> int{
  let x:i32 = 25;
  let active = true;
  while (active == true){
      let x = run(x);
      if(x == true){
        active = false;
        }
        else{
        active = true;
        }
    }
}
