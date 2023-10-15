use rand::seq::SliceRandom;
const N: u16 = 5;

fn main() {
    let mut nums = [0; 75];

    for i in 0..75 {
        nums[i] = i+1;
    }

    //shuffle
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    let mut i = 0;

    for _ in 0..N {
        for _ in 0..N{
            if i == (N*N/2) as usize {
                print!("{:>3}, ", "*");
            }else{
                print!("{:3}, ", nums[i]);
            }
            i += 1;
        }
        println!("");
    }
}
