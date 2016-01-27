fn logic(x:i32,times:i32,name:bool) -> bool{
  if(x<50 && times >= 3){
  bool = false;
  }
  if(x>50){
  bool = true;
  times = times + 1;
  logic(x,times,name);
  }
}
