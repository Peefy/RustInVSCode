use std::io; // 标准库
use std::cmp::Ordering;
// use rand;
// use front_of_house::hosting;

fn rust_rand() {
    println!("\nHello Rust rand!");
    let secret_number = 1;  // rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
}

fn rust_match() {
    println!("\nHello Rust match!");
    let mut number = 2;
    let mut guess = "1";
    let deaultVal = 1;

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => deaultVal,
    };

    match guess.cmp(&number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

fn test_trait() {
    println!("hello test trait!");
}

fn expr() {

}

/* 主函数声明 */
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /* 可变变量 */
    let mut guess = String::new();

    /* 传变量的引用 */
    // io::stdin().read_line(&mut guess)
    //    .expect("Failed to read line");  /* std::result::Result */

    /* 使用println!占位符打印值 */
    println!("You guessed: {}", guess);

    /* 定义不可变变量 */
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);
    test_trait();
    // rand number
    rust_rand();
    // match keyword
    rust_match();
}
