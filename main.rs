fn main(){
  pattern_matching();
}

fn pattern_matching() {
  for x in 1..= 20 {
    println!("{} : {}", x, match x {
      1 => "pouco",
      2 | 3 => "um pouquinho",
      4..=10 => "um bocado",
      _ if x % 2 == 0 => "uma boa quantidade",
      _ => "muito"
    });
  }
  
}