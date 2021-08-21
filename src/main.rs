use std::io; // 标准库
use std::cmp::Ordering;
// use rand;
// use front_of_house::hosting;

struct Person {
    name: String,
    age: u8,
}

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
    println!("const is- {}", max_points);
}

// rust_datatype print the rust datatype introduction
fn rust_datatype() {
    println!("Rust 中的数据类型");
    let u8_var: u8 = 1;
    let u16_var: u16 = 2;
    let u32_var: u32 = 3;
    let u64_var: u64 = 4;
    let u128_var: u128 = 5;
    let i8_var: i8 = 1;
    let i16_var: i16 = 2;
    let i32_var: i32 = 3;
    let i64_var: i64 = 4;
    let i128_var: u128 = 5;
    let usize_var: usize = 1;
    let isize_var: isize = 1;
    let u32_var_from_str: u32 = "42".parse().expect("Not a number");
    let f32_var: f32 = 1.0;
    let f64_var: f64 = 1.0;
    let bool_var: bool = true;
    let c_var: char = 'a';
    let str_var: &str = "xx";
    let another_str_var: String = String::from("xx");
    let tuple_var: (i32, f32) = (1, 1.0);
    let arr_var: [i32; 4] = [1, 2, 3, 4];
    let another_arr = [0; 5];  // [0, 0, 0, 0, 0]
    let v = vec![1, 2, 3];
    println!("无符号整数类型：u8 u16 u32 u64 u128 {} {} {} {} {}", u8_var, u16_var, u32_var, u64_var, u128_var);
    println!("有符号整数类型：i8 i16 i32 i64 i128 {} {} {} {} {}", i8_var, i16_var, i32_var, i64_var, i128_var);
    println!("isize 和 usize 两种类型为64位或者32位取决于程序所运行的计算机类型 {} {}", isize_var, usize_var);
    println!("浮点类型：f32 f64 {} {}", f32_var, f64_var);
    println!("布尔类型 bool {}", bool_var);
    println!("字符类型(用单引号表示) char {}", c_var);
    println!("字符串引用类型(用双引号表示) &str {} {}", str_var, another_str_var);
    println!("元组类型 {} {}", tuple_var.0, tuple_var.1);
    println!("数组类型 {} {} {}", arr_var[0], arr_var[1], another_arr[0]);
    println!("容器类型 vec! {}", v[0]);
    println!("字符串类型转整数类型: \"42\".parse().expect(msg=\"\") {}", u32_var_from_str);
}

fn rust_operator() {
    println!("Rust 中的运算符");
    // 宏展开
    let a: Vec<u8> = vec![1, 2, 3];
    let mut var = 1;
    let foo = 1;
    let bar = 1;
    println!("宏展开运算符! {}", a[0]);
    println!("按位非或者逻辑非 {}", !var);
    println!("加法 + {}", foo + bar);
    println!("减法 - {}", foo - bar);
    println!("乘法 * {}", foo * bar);
    println!("除法 / {}", foo / bar);
    println!("算数取模 % {}", foo % bar);
    println!("按位与 ^ {}", foo & bar);
    println!("按位或 | {}", foo | bar);
    println!("按位异或 ^ {}", foo ^ bar);
    println!("逻辑与 {}", true && false);
    println!("逻辑或 % {}", true || false);
    println!("左移 << {}", foo << bar);
    println!("右移 >> {}", foo >> bar);
    println!("等于 == {}", foo == bar);
    println!("不等 != {}", foo != bar);
    println!("大于 > {}", foo > bar);
    println!("大于等于 >= {}", foo >= bar);
    println!("小于 < {}", foo < bar);
    println!("小于等于 <= {}", foo <= bar);
    println!("借用 &var {}", &var);
    println!("可变借用 &mut var {}", &mut var);
    println!("生命周期可变借用 &'a var");
    println!("生命周期借用 &'a mut var");
    println!("错误传播 expr?");
    println!("模式绑定 expr @ pat");
    println!("模式匹配 => ");
    println!("范围 1..3");
    println!("其余运算符 ..");
    for i in 1..3 {
        print!("{} ", i)
    }
    let person = Person {
        name: String::from("Alice"),
        age: 18,
    };
    let another_person = Person {
        ..person
    };
    println!("{}", another_person.name)
}

fn rust_function() {
    println!("Rust 中的函数");
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
    println!()
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
    // Rust operator
    rust_operator();
    // Rust function
    rust_function();
    // Rust expression
    rust_expr();
}
