use rand::Rng;

const MAP_N: usize = 25;

fn main() {
    let mut rng = rand::thread_rng();

    //초기화
    let mut maze = [[0; MAP_N]; MAP_N];

    for n in 0..MAP_N {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAP_N-1] = 1;
        maze[MAP_N-1][n] = 1;
    }

    //2칸 마다 1개 벽 설치
    for y in 2..MAP_N-2 {
        for x in 2..MAP_N-2 {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }
            maze[y][x] = 1; //벽

            //상하좌우 중 어느 하나 벽 만들기
            let r = rng.gen_range(0..=3);

            match r {
                0 => maze[y-1][x] = 1, //상
                1 => maze[y+1][x] = 1, //하
                2 => maze[y][x-1] = 1, //좌
                3 => maze[y][x+1] = 1, //우
                _ => {},
            }
        }
    }

    //미로
    let titles = ["☐","◼︎"]; // 

    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", titles[maze[y][x]]);
        }
        println!("");
    }

}
