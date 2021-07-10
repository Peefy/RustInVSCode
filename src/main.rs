use std::io; // 标准库
use std::cmp::Ordering;
// use rand;
// use front_of_house::hosting;

fn rust_keyword() {
    println!("Rust 中的关键字");
    println!("as - 强制类型转换，消除特定包含项的 trait 的歧义，或者对 use 和 extern crate 语句中的项重命名");
    println!("break - 立刻退出循环)");
    println!("const - 定义常量或不变裸指针（constant raw pointer）");
    println!("continue - 继续进入下一次循环迭代");
    println!("crate - 链接（link）一个外部 crate 或一个代表宏定义的 crate 的宏变量");
    println!("dyn - 动态分发 trait 对象");
    println!("else - 作为 if 和 if let 控制流结构的 fallback");
    println!("enum - 定义一个枚举");
    println!("extern - 链接一个外部 crate 、函数或变量");
    println!("false - 布尔字面值 false");
    println!("fn - 定义一个函数或 函数指针类型 (function pointer type)");
    println!("for - 遍历一个迭代器或实现一个 trait 或者指定一个更高级的生命周期");
    println!("if - 基于条件表达式的结果分支");
    println!("impl - 实现自有或 trait 功能");
    println!("in - for 循环语法的一部分");
    println!("let - 绑定一个变量");
    println!("loop - 无条件循环");
    println!("match - 模式匹配");
    println!("mod - 定义一个模块");
    println!("move - 使闭包获取其所捕获项的所有权");
    println!("mut - 表示引用、裸指针或模式绑定的可变性");
    println!("pub - 表示结构体字段、impl 块或模块的公有可见性");
    println!("ref - 通过引用绑定");
    println!("return - 从函数中返回");
    println!("Self - 实现 trait 的类型的类型别名");
    println!("self - 表示方法本身或当前模块");
    println!("static - 表示全局变量或在整个程序执行期间保持其生命周期");
    println!("struct - 定义一个结构体");
    println!("super - 表示当前模块的父模块");
    println!("trait - 定义一个 trait");
    println!("true - 布尔字面值 true");
    println!("type - 定义一个类型别名或关联类型");
    println!("unsafe - 表示不安全的代码、函数、trait 或实现");
    println!("use - 引入外部空间的符号");
    println!("where - 表示一个约束类型的从句");
    println!("while - 基于一个表达式的结果判断是否进行循环");
    println!("Rust 中的保留关键字");
    println!("* abstract");
    println!("* async");
    println!("* await");
    println!("* become");
    println!("* box");
    println!("* do");
    println!("* final");
    println!("* macro");
    println!("* override");
    println!("* priv");
    println!("* try");
    println!("* typeof");
    println!("* unsized");
    println!("* virtual");
    println!("* yield");
}

fn rust_identifier() {
    println!("Rust 中的标识符");
    let r#async = "1";
    println!("raw identifier is {}", r#async);
}

fn rust_variable() {
    println!("Rust 中的变量");
    // 不可变变量
    let x = 1;
    // 可变变量
    let y = 3;
    println!("immutable var is {}", x);
    println!("mutable var is {}", y);
    // 常量
    let max_points: u32 = 100_000;
    println!("const is {}", max_points);
}

fn rust_datatype() {
    println!("Rust 中的数据类型");
}

fn rust_rand() {
    println!("\nHello Rust rand!");
    let secret_number = 1;  // rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
}

fn rust_match() {
    println!("\nHello Rust match!");
    let number = 2;
    let guess = "1";
    let default_val = 1;

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => default_val,
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

fn rust_expr() {

}

/* 主函数声明 */
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /* 可变变量 let mut guess = String::new(); */
    let guess = String::new();

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
    // Rust keyword
    rust_keyword();
    // Rust idenfier
    rust_identifier();
    // Rust variable
    rust_variable();
    // Rust data type
    rust_datatype();
    // Rust expression
    rust_expr();
}
