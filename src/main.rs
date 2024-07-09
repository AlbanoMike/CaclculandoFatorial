
use core::num;
use std::{f32::DIGITS, io::{self, stdin}, str::Chars};
fn convert_to_int(data: &str) -> i32{
  let x = data.trim().parse::<i32>().unwrap();
  x
}

fn main() {
  let mut entrada = String::new();
  io::stdin().read_line(&mut entrada).expect("error");
  let mut entrada_int = convert_to_int(&entrada);
  let mut contador = 1;
  

  while entrada_int > 1 {
      contador = entrada_int * contador;
      entrada_int -= 1;
  }
  print!("valor {}", contador);
}