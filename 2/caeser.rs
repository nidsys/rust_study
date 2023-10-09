fn encrypt (text:&str, shift:i16) -> String{
  let code_a = 'A' as i16;
  let code_z = 'Z' as i16;

  //let mut result = "".to_string();
  let mut result = String::new();

  for ch in text.chars() {
    let mut code = ch as i16;

    if code_a <= code && code <= code_z {
      //shift
      code = (code - code_a + shift) % 26 + code_a;
    }
    // u8로 변환후 다시 char 로 변환
    result.push((code as u8) as char);
  }
  result
}

fn main(){
  let enc = encrypt ("ABCDEF", 3);
  let dec = encrypt (&enc, -3);

  println!("{}", enc);
  println!("{}", dec);
}