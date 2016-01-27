mod logic.rs;
fn run(x:i32) -> bool{
  let on:bool = true;
    let add:bool = subtr:bool = mult:bool = divd:bool = true;
    let adt:i32 = subt:i32 = multt:i32 = divt:i32 = 0;
  while(on == true){
      if(x/2 == int(x/2) && divd == true){
        x = x/2;
        divd = logic(x, divt);
      }
      if(x > 50 && subtr == true){
        x = x - 1;
        subtr = logci(x, subt);
      }
      if(x < 50 && add == true){
        x = x + 1;
        add = logic(x, adt);
      }
      if(x*2 > 10 && mult == true){
        x = x*2;
        mult = logic(x, multt);
      }
      if(x == 50){
        return false;
      }
    }
  }
