use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//1초 마다 메시지를 보내는 함수
fn sleep_sender(name: &str, sender: mpsc::Sender<String>){
    let whales = ["큰고래","혹등고래","향유고래","남바은돌고래","븍극고레"];

    for whale in whales {
        let msg = format!("{} : {}", name, whale);

        sender.send(msg).unwrap(); //송신
        thread::sleep(Duration::from_millis(1000));
    }

    sender.send("quit".to_string()).unwrap();
}


fn main() {
    // 쓰레드간 통신용 채널
    let (tx, rx) = mpsc::channel::<String>();

    // 쓰레드 1생성
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("우영우", sender)
    });

    //쓰레드2
    let sender = tx.clone();
    thread::spawn(|| { sleep_sender("이준호",sender)});

    //쓰레드로 부터 메시지르르 반복해서 받음
    loop {
        let buf = rx.recv().unwrap();

        println!("수신 {}", buf);
        if buf == "quit" {
            break;
        }
    }
}
