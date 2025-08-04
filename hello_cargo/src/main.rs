use std:: io; 


fn main() {
    println!("Guess the number!");
    println!("Please input you guess.")

    let mut guess = String::new(); // 변수 생성 
    let apples = 5; //immutable
    let mut bananas = 5; //mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("")




}
