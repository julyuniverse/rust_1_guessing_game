use rand::Rng; // rand crate
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // rand를 통해 랜덤 비밀번호 생성
    let secret_number = rand::thread_rng().gen_range(1..101);

    // 루프 생성
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // 사용자로부터 입력을 받기 위해 표준 입력 핸들
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 값이 같을 경우 루프 종료
            }
        }
    }
}
