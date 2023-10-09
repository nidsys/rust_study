//클로저 Closure

fn encrypt(text:&str, shift:i16) -> String {
  let a = 'A' as i16;
  let is_az = |c| 'A' <= c && c <= 'Z'; // return true, false ?
  let conv = |c| (((c-a+shift) % 26 + a) as u8) as char; // shift 된 문자리턴
  let enc1 = |c| if is_az(c) { conv(c as i16)}else{c}; // A-Z에 속하면 shift 문자 리턴, 아니면 그냥 리턴
  text.chars().map(|c| enc1(c)).collect() //enc1 으로 text 문자를 1자씩 넘겨서 다시 합치기. 리턴이므로 ; 없음
}

fn main() {
  let enc = encrypt("ABCDEF", 3);
  let dec = encrypt(&enc, -3);

  println!("{} => {}", enc, dec);
}