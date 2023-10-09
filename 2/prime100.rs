//소수 판단
fn is_prime(n:usize) -> bool {
  for i in 2..n {
    if n % i == 0{
      return false;
    }
  }
  true
}

// 갯수만큼
fn get_primes(n:usize) -> Vec<usize> {
  let mut res = vec![];

    let mut i=2;
    
    loop {
        if is_prime(i) {
            res.push(i);
        }
        i += 1;
        if res.len() == n {
            break;
        }
    }

  res
}

fn main() {
  println!("{:?}", get_primes(100));
}
