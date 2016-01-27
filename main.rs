fn main() -> int{
  let active = true;
  while (active == true){
      let x = run();
      if(x == true){
        active = false;
        }
        else{
        active = true;
        }
    }
}
