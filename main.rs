fn logic(x:i32,times:i32,name:bool) -> bool{
  if x<50 && times >= 3{
  bool = false;
  }
  if x>50 {
  bool = true;
  times = times + 1;
  logic(x,times,name);
  }
fn run(x:i32) -> bool{
  let on = true;
    let add = true;
    let subtr = true;
    let mult = true; 
    let divd = true;
    let adt = 0; 
    let subt = 0;
    let multt = 0;
    let divt = 0;
  while on == true {
      if x/2 > 1 && divd == true{
        x = x/2;
        divd = logic(x, divt, divd);
      }
      if x > 50 && subtr == true {
        x = x - 1;
        subtr = logci(x, subt, subtr);
      }
      if x < 50 && add == true {
        x = x + 1;
        add = logic(x, adt, add);
      }
      if x*2 > 10 && mult == true {
        x = x*2;
        mult = logic(x, multt, mult);
      }
      if x == 50 {
        return false;
      }
     println!("{}",x);
    }
  }}
fn main() -> int{
  let x:i32 = 25;
  let active = true;
  while active == true {
      let x = run(x);
      if x == false {
        active = false;
        }
        else{
        active = true;
        }
    }
  println!("we have ended");
  return 0;
}
