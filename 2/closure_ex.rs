//클로저 Closure
//let 이름 = |인수| 정의;

fn main() {
  let x2 = |n| n*2;

  println!("{}", x2(2));
  println!("{}", x2(8));
}