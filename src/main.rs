use std::cmp::Ordering;
use std::fmt;
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
    let another_arr = [0; 5]; // [0, 0, 0, 0, 0]
    let v = vec![1, 2, 3];
    println!(
        "无符号整数类型：u8 u16 u32 u64 u128 {} {} {} {} {}",
        u8_var, u16_var, u32_var, u64_var, u128_var
    );
    println!(
        "有符号整数类型：i8 i16 i32 i64 i128 {} {} {} {} {}",
        i8_var, i16_var, i32_var, i64_var, i128_var
    );
    println!(
        "isize 和 usize 两种类型为64位或者32位取决于程序所运行的计算机类型 {} {}",
        isize_var, usize_var
    );
    println!("浮点类型：f32 f64 {} {}", f32_var, f64_var);
    println!("布尔类型 bool {}", bool_var);
    println!("字符类型(用单引号表示) char {}", c_var);
    println!(
        "字符串引用类型(用双引号表示) &str {} {}",
        str_var, another_str_var
    );
    println!("元组类型 {} {}", tuple_var.0, tuple_var.1);
    println!("数组类型 {} {} {}", arr_var[0], arr_var[1], another_arr[0]);
    println!("容器类型 vec! {}", v[0]);
    println!(
        "字符串类型转整数类型: \"42\".parse().expect(msg=\"\") {}",
        u32_var_from_str
    );
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
    let another_person = Person { ..person };
    println!("{}", another_person.name)
}

fn my_func(para1: i32, para2: i32) -> i32 {
    println!("my_func call {} {}", para1, para2);
    para1 + para2
}

#[allow(unused_variables)]
fn rust_function() {
    println!("Rust 中的函数");
    println!("函数定义以 fn 关键字开头");
    println!("函数支持多个参数，但是每个参数都不支持默认值");
    for i in 1..5 {
        println!("The function return value is {}", my_func(i, i));
    }
    println!("函数实体包含语句和表达式");
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y);
}

fn rust_control_flow() {
    println!("Rust 中的控制流");
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisble by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }
    let condition = true;
    let mut numberx = if condition { 2 } else { 3 };
    println!("使用 if 的 let 语句 {}", numberx);
    println!("Rust 中的 if let 语句");
    println!("Rust 中的循环与重复 loop, while, for");
    // 死循环
    // loop { println!("Rust 中的永久循环 loop ); }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("从循环返回值 {}", result);
    while numberx != 0 {
        println!("{}!", numberx);
        numberx -= 1;
    }
    println!("遍历一个集合");
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", arr[index]);
        index += 1;
    }
    for elem in arr.iter() {
        println!("The value is: {}", elem);
    }
    for n in (1..4).rev() {
        println!("{}?", n);
    }
}

fn rust_rand() {
    println!("\nHello Rust rand!");
    let secret_number = 1; // rand::thread_rng().gen_range(1, 101);
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
    println!("Rust 中的表达式");
}

fn rust_ownership() {
    println!("Rust 中的所有权");
    println!("Rust 的主要特征之一是所有权");
    println!("所有程序必须在运行时管理它们使用计算机内存的方式，某些语言具有垃圾回收功能，该垃圾回收功能会在程序运行时不断寻找不再使用的内存。");
    println!("在其他语言中，程序员必须显式分配和释放内存");
    println!("Rust使用第三种方法：通过所有权系统管理内存，该系统具有一组在编译时检查的规则。程序运行时，所有所有权的功能都不会影响性能，做到零开销抽象");
    println!("在许多语言中，不必经常考虑堆和栈。但是在像 Rust 这样的系统编程语言中，值是在堆上还是在栈上对语言的行为以及为什么做出某些决定影响更大");
    println!("堆和栈都是内存的一部分，可以在运行时使用。编译时大小未知或者大小可能更改的数据必须存在堆中。堆的组织性较差");
    println!("Rust 的所有权规则");
    println!("1. Rust 中的每个值都有一个变量, 称其为所有者");
    println!("2. 一次只能有一个所有者");
    println!("3. 当所有者超出范围时，该值将被删除");
    // 可以改变值的字符串
    let mut s = String::from("Hello");
    s.push_str(" World");
    println!("{}", s);
    println!("Rust 可拷贝类型：所有基本类型与所有基本类型组成的Tuple/Struct类型");
    let takes_ownership = |some_string: String| {
        println!("{}", some_string);
        some_string
    };
    let makes_copy = |some_int: i32| println!("{}", some_int);
    let not_copy_str = String::from("ss");
    let copy_int = 5;
    takes_ownership(not_copy_str);
    // println!("{}", not_copy_str);  // can not use str again
    makes_copy(copy_int);
    println!("{}", copy_int); // can use int again
    println!("函数返回值也可以转移所有权");
    let s2 = String::from("ss");
    let s3 = takes_ownership(s2);
    println!("{} {}", s3, s3);
    println!("变量的所有权每次都遵循相同的模式：将值分配给另一个变量将其移动。当包含堆上数据的变量超出范围时，将清除该值");
    let s4 = String::from("ss");
    let s5 = s4;
    // let s6 = s4;  // Error: s4 has be moved to s5
    let s6 = s5; // Ok: s4 -> s5 -> s6, now s6 has the ownership, s4 and s5 can not be used again
    println!("{}", s6); // Ok
    println!("由于函数调用会把输入的参数的所有权也交出去，一个解决的方式是通过多返回值把输入参数也通过函数值返回，相当于把交出去的变量所有权再拿回来，比如下面的例子");
    let cal_len = |s: String| {
        let l = s.len();
        (s, l)
    };
    let s7 = String::from("sss");
    let (s7, s7_l) = cal_len(s7);
    println!("str: {}, str len: {}", s7, s7_l);
}

fn rust_reference() {
    println!("Rust 中的引用");
    println!("引用运算符 &, 解引用运算符 *");
    println!("如果变量在默认情况下是不可变的，那么引用也是默认不可变的");
    // 可变引用
    let mut s = String::from("Hello");
    let change = |s: &mut String| s.push_str(", World");
    change(&mut s);
    println!("{}", s);
    println!("不允许同时存在两个及以上的可变引用");
    let sr_1 = &mut s;
    println!("{}", sr_1);
    // let sr_2 = &mut s;  // cannot borrow `s` as mutable more than once at a time
    // println!("{}", sr_2);
    println!("像C语言那样，可以使用花括号创建新的 Scope, 从而间接允许多个可变引用");
    {
        let sr_2 = &mut s;
        println!("{}", sr_2);
    }
    println!("Rust 中的悬空引用");
    println!("在带有指针的语言中，很容易错误地常见一个悬空指针，即通过在保留指向该内存的指针同时释放一些内存来引用可能已分配给他人的内存中某个位置的指针");
    println!("在 Rust 中，编译器保证引用永远不会成为悬空引用；如果某些数据具有引用，则编译器将确保数据不会超过对数据的引用范围");
    /*下面的代码是错误的 cannot return reference to local variable `s`
    returns a reference to data owned by the current function
    let f = || {
        let s = String::from("s");
        &s
    };
    let f = || {
        &String::from("s")
    };
    fn dangle() -> &String {}
    需要声明的返回引用的生命周期, 比如静态的生命周期而不是局部函数的生命周期
    fn dangle() -> &'static String {}
    */
}

fn rust_slice() {
    println!("Rust 中的切片类型");
    println!("Rust slice 是没有所有权的数据类型，切片可以引用集合中连续的元素序列，而不是整个集合");
    let first_word = |s: &String| -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    };
    println!("{}", first_word(&String::from("sss")));
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    let mut s_mut = "Hello World".to_string();
    first_word(&s_mut);
    s_mut.clear();
    println!("{}", s_mut);
    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{}", slice[1]);
}

#[allow(unused_variables)]
fn rust_struct() {
    println!("Rust 中的结构");
    struct User {
        username: String,
        age: i8,
        account: u64,
        active: bool,
    }
    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "({}, {}, {}, {})",
                self.username, self.age, self.account, self.active
            )
        }
    }
    let user1 = User {
        age: 12,
        username: String::from("name"),
        account: 1,
        active: true,
    };
    // 使用结构更新语法 .. 从其他实例创建实例，类似 Python 的解包运算符 **
    let user2 = User {
        ..user1 // 注意此处 user2 借用了 user1, 因为 User 结构存在不可拷贝字段 username, 此后不能再次被任何变量或者函数参数借用
    };
    println!("{}", user2);
    // 使用没有命名字段的元组结构创建不同的类型
    struct Color(i32, i32, i32);
    struct EmptyStruct {}
    let black = Color(0, 0, 0);
    let empty = EmptyStruct {};
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.height * self.width
        }
        // 成员方法
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        // 整个结构的关联方法
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle {
                width: width,
                height: height,
            }
        }
    }
    let area = |rec: &Rectangle| rec.width * rec.height;
    println!(
        "The area is {}",
        area(&Rectangle {
            width: 2,
            height: 4,
        })
    );
    println!(
        "{} {} {:?}",
        Rectangle {
            width: 2,
            height: 4,
        }
        .area(),
        Rectangle {
            width: 3,
            height: 4,
        }
        .can_hold(&Rectangle {
            width: 2,
            height: 4,
        }),
        Rectangle::new(1, 2),
    )
}

#[allow(unused_variables)]
fn rust_enum() {
    println!("Rust 中的枚举");
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?} {:?}", four, six);
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:?} {}", home.kind, home.address);
    println!("一个更简单的方式，通过将数据直接放入每个枚举变量中，可以仅使用枚举以更简洁的方式表示相同的概念");
    #[derive(Debug)]
    enum IpAddrWithPara {
        V4(String),
        V6(String),
    }
    println!(
        "{:?}, {:?}",
        IpAddrWithPara::V4(String::from("127.0.0.1")),
        IpAddrWithPara::V6("::1".to_string())
    );
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(self) -> i32 {
            match self {
                Message::Quit => 1,
                Message::Write(s) => s.parse().unwrap(),
                Message::Move { x, y } => x + y,
                Message::ChangeColor(a, b, c) => a + b + c,
            }
        }
    }
    println!("{}", Message::Quit.call());
    println!("{}", Message::Write(String::from("123")).call());
    println!("{}", Message::Move { x: 1, y: 1 }.call());
    println!("{}", Message::ChangeColor(1, 2, 3).call());
    // Option 枚举
    let none_number: Option<i32> = None;
    println!("{:?} {}", none_number, none_number.is_none());
    let not_none_number: Option<i32> = Some(1);
    println!("{:?} {}", not_none_number, not_none_number.is_none());
    let some_string = Some("ss");
    // match 运算符
    let plus_one = |x: Option<i32>| match x {
        Some(i) => Some(i + 1),
        None => None,
    };
    println!("{:?} {:?}", plus_one(None), plus_one(Some(1)));
    let some_u8_val = 0u8;
    match some_u8_val {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other value"),
    }
}

fn rust_module() {
    println!("Rust 中的模块");
    mod back {
        #[derive(Debug)]
        pub struct Person {
            name: String,
            age: u8,
        }
        impl Person {
            pub fn new(name: String, age: u8) -> Person {
                Person {
                    name: name,
                    age: age,
                }
            }
        }
    }
    println!("Alice: {:?}", back::Person::new("Alice".to_string(), 12));
    mod my_core {
        pub mod io {
            pub mod print {
                pub fn print() {}
                pub fn print2() {}
            }
        }
    }
    use my_core::io::print;
    use my_core::io::print as alias_print;
    print::print();
    alias_print::print();
    // use my_core::io::print::print;
    use my_core::io::print::{print, print2};
    use my_core::io::print::*;
    print();
    print2();
}

fn rust_data_type() {
    println!("Rust 中的数据结构");
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(0);
    println!("{:?}", v);
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
    // Rust expression
    rust_expr();
    // Rust data type
    rust_datatype();
    // Rust operator
    rust_operator();
    // Rust function
    rust_function();
    // Rust control flow
    rust_control_flow();
    // Rust ownership
    rust_ownership();
    // Rust reference
    rust_reference();
    // Rust slice
    rust_slice();
    // Rust struct
    rust_struct();
    // Rust enum
    rust_enum();
    // Rust module
    rust_module();
    // Rust data type
    rust_data_type();
}
