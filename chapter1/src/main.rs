extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //추리게임 답을 알고 싶으면 주석해제
    // println!("The secret number is: {}", secret_number);

    
    loop {
        //이 변수에 수정을 가할 수 있다. 기본은 수정 못함.
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
                    .expect("failed to read line");
    
        //타입추론을 해줘야한다. 똑똑하지 못하네
        //match를 두어서 u32 타입이 될 때까지 계속 입력을 받을 수 있다.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input correct Number");
                continue;
            },
        };
    
        // place holder
        println!("your guess : {}", guess);
    
        //match
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("Equal!");
                break;
            },
            Ordering::Greater => println!("Too Big"),
        }

    }
}
