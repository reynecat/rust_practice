/* 
std 표준 라이브러리 안에 있는 io 라이브러리 스코프로 가져오기
rust는 자동으로 사용할 수 있는 표준 라이브러리의 기본 아이템 집합, 
프렐류드 (prelude)가 존재
Vec
원하는 타입이 프렐류드ㅏ 

*/

use rand::Rng; // Rng는 난수 생성기 트레이트
use std::cmp::Ordering; // 열거형 less, Greater, Equal 베리언트를 보유
use std::io;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //

    println!("The Secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()  // 사용자 입력 받기
        .read_line(&mut guess) // 참조자 사용 mut을 넣어서 가변적으로 
        .expect("Failed to read line"); 

    println!("You guessed: {guess}");
    // {}는 자리 표시자 

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!");
        Ordering::Greater => println("Too big!");
        Ordering::Equal =>
    }
}