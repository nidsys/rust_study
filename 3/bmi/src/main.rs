fn input(text:&str) -> f64 {
    println!("{}", text);

    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("입력오류");

    s.trim().parse().expect("숫자가 아님")
}

fn main() {
    let height_cm = input("키 입력 : ");
    let weight = input("몸무게 : ");

    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0); // float형 제곱이므로 powf 

    println!("bmi : {:.1}", bmi);

    if bmi < 18.5 {
        println!("저체중");
    }
    else if bmi < 23.0 {
        println!("정상");
    }
    else if bmi < 25.0 {
        println!("비만전")
    }
    else if bmi < 30.0 {
        println!("1단계");
    }
    else if bmi < 35.0 {
        println!("2단계");
    }
    else {
        println!("3단계")
    }
}
