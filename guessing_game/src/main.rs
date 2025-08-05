/* 
std 표준 라이브러리 안에 있는 io 라이브러리 스코프로 가져오기
rust는 자동으로 사용할 수 있는 표준 라이브러리의 기본 아이템 집합, 
프렐류드 (prelude)가 존재
Vec
원하는 타입이 프렐류드ㅏ 

*/

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}