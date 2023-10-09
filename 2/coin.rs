fn main () {
  let price = 3950;

  for i500 in 0..=10 {
    for i100 in 0..=3 {
      for i50 in 0..=10 {
        let tot = i500*500 + i100*100 + i50*50;
        if tot == price {
          println!("500원x{}개, 100원x{}개, 50원x{}개 = {}", i500,i100,i50,tot);
        }
      }
    }
  }
}