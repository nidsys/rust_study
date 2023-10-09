// 지구에서 달까지의 거리는 384400km이다. 지구에서 달까지 80km/h 속도의 자동차와 300km/h 속도의 KTX 로 간다면 각 이동 수단은 며칠이 걸리는지 계산하시오

fn main () {
  let moon = 388440.0;
  let car = 80.0;
  let ktx = 300.0;

  println!("달까지 차로 {}일", moon / car / 24.0);
  println!("달까지 ktx로 {}일", moon / ktx / 24.0);
}