use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
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
    // Rust 匹配守卫
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    };
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    let x = 1;
    let y = 1;
    // Use tuple to match multiple var
    match (x, y) {
        (1, 1) => println!("multiple match"),
        _ => println!("no"),
    }
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 3 };
    match msg {
        Message::Hello { id: id_var @ 3..=7 } => {
            println!("Found and id in range: {}", id_var);
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
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
    // 可以使用 if let 语句直接从枚举中取值，类似 C# 类型的中的 is 类型强转 if (ss is String ss) {}
    if let Some(ss) = some_string {
        println!("some_string is a string enum type");
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
    use my_core::io::print::*;
    use my_core::io::print::{print, print2};
    print();
    print2();
    // 使用外部软件包, Rust社区的成员已经在crates.io上提供了许多软件包
    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("A secret number {:?}", secret_number);
}

fn rust_string() {
    println!("Rust 中的字符串");
    // 字符串是UTF-8编码的，因此可以在其中包含任何正确编码的数据
    let data = "initial contents";
    let s = data.to_string();
    let mut b = "ss".to_string();
    b.push_str("bar");
    println!("{} {} {}", data, s, b);
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s2);
    let s4 = format!("{}-{}", s2, s3);
    println!("{}", s4);
    for c in "ssss".chars() {
        println!("{}", c);
    }
}

fn rust_data_type() {
    println!("Rust 中的数据结构");
    let v1 = Vec::<i32>::new();
    let mut v2: Vec<i32> = vec![1, 2, 3];
    v2.push(0);
    println!("{:?} {:?}", v1, v2);
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);
    // 使用 [index] 和 get 方法均可获得引用
    if let Some(v) = v2.get(2) {
        println!("The third element is {}", v);
    }
    // 遍历容器中的值
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i)
    }
    // 迭代对向量中元素的可变引用
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
    #[derive(Debug)]
    enum Spread {
        Int(i64),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Spread::Int(3),
        Spread::Text(String::from("blue")),
        Spread::Float(10.12),
    ];
    println!("{:?}", row);
    // Rust 哈希
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    println!("{:?}", scores);
    let teams = vec![String::from("1"), String::from("2")];
    let scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    if let Some(score) = map.get(&String::from("blue")) {
        println!("{}", score);
    }
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn rust_clousure() {
    println!("Rust 中的闭包");
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x: u32| x + 1;
    // Rust 中的闭包 Trait
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation: calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    let mut result = Cacher::new(|num| num);
    // 闭包捕获变量
    let x = 4;
    let equal_to_x = |z| z == x;
    assert_eq!(equal_to_x(4), true);
    // 使用 move 关键字获得变量 x 的所有权，当 x 没有实现 Copy Trait 时
    let equal_to_x = move |z| z == x;
    assert_eq!(equal_to_x(4), true);
    println!("{}", x);
}

fn rust_iterator() {
    println!("Rust 中的迭代器");
    let v1 = vec![1, 2, 3];
    // 注意 v1_iter 需要是可变的，在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态
    let mut v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    // 使用 filter 迭代器适配器和捕获环境的闭包
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    let shoes = vec![
        Shoe {
            size: 10,
            style: "a".to_string(),
        },
        Shoe {
            size: 20,
            style: "b".to_string(),
        },
        Shoe {
            size: 30,
            style: "c".to_string(),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("in my size: {:?}", in_my_size);
    // 使用 Iterator trait 创建自定义迭代器
    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(sum, 18);
}

fn rust_error_handling() -> Result<(), std::io::Error> {
    println!("Rust 中的错误处理");
    let f = File::open("hello.txt");
    let file = match f {
        Ok(file) => Some(file),
        Err(error) => {
            None
            // panic!("Problem opening the file: {:?}", error);
        }
    };

    use std::io::ErrorKind;

    let f = File::open("hello.txt");
    let file = match f {
        Ok(file) => Some(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => None,
            other_kind => None,
        },
    };
    /*
    let f = File::open("hello.txt").unwrap_or_else(|err| {
        File::create("hello.txt").unwrap()
    });
    */
    println!("传播错误的捷径：? 运算符");
    // 使用错误传播运算符可以将错误很方便地传递到上一层
    let f = File::open("hello.txt")?;
    Ok(())
}

fn rust_general_type_and_trait() {
    println!("Rust 中的通用类型和 Trait");

    trait TPoint: PartialOrd + Copy {}

    #[derive(Eq, Clone)]
    struct Person {
        id: u32,
        name: String,
        height: u32,
    }

    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Person {
        fn cmp(&self, other: &Self) -> Ordering {
            self.height.cmp(&other.height)
        }
    }

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            self.height == other.height
        }
    }

    struct Point<T> {
        x: Box<T>,
        y: Box<T>,
    }

    impl<T> Point<T> {
        fn x(self) -> Box<T> {
            self.x
        }
    }
    impl Point<f32> {
        fn distance(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    // Trait: 定义共同的行为
    trait Summary {
        fn summarize(&self) -> String;
    }
    struct Article {
        headline: String,
    }
    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("headline: {}", self.headline)
        }
    }
    // Trait 作为参数
    fn notify(item: impl Summary) {}
    // Trait 绑定语法
    fn notify_two<T: Summary>(item: T) {}
    // 使用 + 绑定多个 Trait
    fn notify_three(T: impl Summary + Clone) {}
    // 使用 where 约束
    fn some_func<T, U>(t: T, u: U)
    where
        T: Clone,
        U: std::fmt::Display,
    {
    }

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item
            }
        }

        largest
    }
    let number_list = vec![0, 1, 2, 3, 4];
    println!("The largest number is {}", largest(&number_list));
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T: std::fmt::Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x > self.y {
                println!("x > y");
            } else {
                println!("x <= y");
            }
        }
    }
    // ToString Trait
    println!("{}", 3.to_string());
}

fn rust_lifetime() {
    println!("Rust 中的生命周期");
    println!("Rust 中使用生命周期验证引用，比如不允许悬空引用");
    /*
    let r;         // 生命周期 'a
    {
        let x = 5;  // 生命周期 'b, 花括号内外是不同的生命周期
        r = &x;  // x 的生命周期在离开花括号之后就结束了，会显式地销毁x，因此不能得到一个要销毁对象的引用
    }
    println!("{}", r);
    */
    println!("Rust 中存在一个借用检查器");
    println!("Rust 中采用形如 'a 的语法定义一个生命周期参数，该语法不会改变任何引用的生命周期，像类型参数那样，通过制定生命周期参数，函数可以接受具有任何生命周期的引用");
    println!("生命周期参数可以描述多个引用生命周期彼此的关系，但是不会影响任何生命周期");
    println!("&i32 // a reference");
    println!("&'a i32 // a reference with an explicit lifetime");
    println!("&'a mut i32 // a mutable reference with an explicit lifetime");
    println!("生命周期参数的定义方式与类型参数定义方式一样，始终需要在 impl 关键字之后定义生命周期参数, 因为生命周期是结构类型的一部分");
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("This is a annotation");
            self.part
        }
    }
    // 静态生命周期
    let _s: &'static str = "The string has a static lifetime";
    // 函数中指定泛型类型参数和生命周期的语法
    fn _longest_with_announcement<'a, T>(x: &'a str, y: &'a str, _ann: T) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn rust_test() {
    println!("Rust 中的测试");
    #[cfg(test)]
    mod tests {
        #[test]
        fn test_it_work() {
            assert_eq!(2 + 2, 4);
        }
    }
    // 可以使用 cargo test 进行测试
}

fn rust_smart_pointer() {
    println!("Rust 中的智能指针");
    println!("Rust 标准库中常用的三种智能指针");
    println!("1. Box<T>: 用于在堆上分配内存值");
    println!("2. Rc<T>: 一个引用计数类型，其数据可以有多个所有者，注意 Rc 只能适用于单线程场景，在多线程场景可以使用 Arc<T>");
    println!("3. Ref<T> 和 RefMut<T>, 通过 RefCell<T> 访问，一个在运行时而不是在编译时执行借用规则的类型");
    println!("Box<T> 允许创建递归类型，Rust 需要在编译时知道类型占用了多少空间，一种无法在编译时知道大小的类型是递归类型");
    println!("Box<T> 类型是一个智能指针，因为它实现了 Deref trait (用于使用*运算符), 它允许 Box<T> 值被当作引用对待");
    println!(
        "当 Box<T> 值离开作用域时，由于 Box<T> 类型 Drop trait 实现，box 所指向的堆数据也会被清除"
    );
    let a = Box::new(1);
    println!("a = {}", a);
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::*;
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("常规引用是一个指针类型，一种理解指针的方式就是将其看成指向存储在其他某处值的箭头");
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // 像引用一样使用 Box<T>
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    use std::rc::Rc;
    enum RcList {
        RcCons(i32, Rc<RcList>),
        RcNil,
    }
    use RcList::*;
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    let b = RcCons(3, Rc::clone(&a));
    let c = RcCons(4, Rc::clone(&a));
    println!("内部可变性是 Rust 中的一个设计模式，它允许即使在有不可变引用时改变数据，这通常是借用规则所不允许的");
    println!("通过 RefCell 在运行时检查借用规则");
    println!("不同于 Rc<T>, RefCell<T> 代表其数据的唯一的所有权");
    println!("* 在任意给定时间，只能拥有一个可变引用或任意数量的不可变引用之一(而不是全部)");
    println!("* 引用必须总是有效的");
    println!("三种智能指针 Box<T>, Rc<T>, RefCell<T> 的选择方式");
    println!("* Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者");
    println!("* Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T> 仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查");
    println!("* 因为 RefCell<T> 允许在运行时执行可变借用检查，所以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值");
    trait Messenger {
        fn send(&self, msg: &str);
    }
    struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }
        pub fn set_value(&mut self, value: usize) {
            self.value = value;
            self.messenger.send("ABCDEFG");
        }
    }
    use std::cell::RefCell;
    struct MockMessenger {
        sent_msgs: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_msgs: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_msgs.borrow_mut().push(String::from(message));
            // 这个会在运行时检查是否借用了多次
        }
    }
    println!("Rust 中的引用循环与内存泄漏");
    println!(
        "Rust 的内存安全保证使其难以意外地制造永远也不会被清理的内存(内存泄漏)，但并不是不可能。"
    );
    println!("与在编译时拒绝数据竞争不同，Rust并不保证完全地避免内存泄漏，这意味着内存泄漏在 Rust 被认为是安全的");
    println!("这一点可以通过 Rc<T> 和 RefCell<T> 看出：创建引用循环的可能性是存在的");
    println!("可以使用弱指针避免引用循环 Weak");
    use std::rc::Weak;
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let branch = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}

fn rust_concurrent() {
    println!("Rust 中的并发");
    use std::thread;
    use std::time::Duration;
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spwaned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    // 线程与 Move 闭包
    let v = vec![0, 1, 2];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
    // Rust channel
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("Hello")];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    for rec in rx {
        println!("Got: {}", rec);
    }
    use std::sync::{Arc, Mutex};
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

fn rust_unsafe() {
    println!("Rust 中的不安全代码");
    println!("* 解引用裸指针");
    println!("* 调用不安全的函数或方法");
    println!("* 访问或修改可变静态变量");
    println!("* 实现不安全的trait");
    println!("* 访问union字段");
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is {}", *r1);
        println!("r1 is {}", *r2);
    }
    unsafe fn unsafe_func() {}
    unsafe {
        unsafe_func();
    }
    use std::slice;
    let addr = 0x012345usize;
    let r = addr as *mut i32;
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("The abs value is {}", abs(-3));
    }
}

fn rust_operator_overload() {
    println!("Rust 中的运算符重载");
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    use std::ops::Add;
    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    struct Mellimeters(u32);
    struct Meters(u32);
    impl Add<Meters> for Mellimeters {
        type Output = Mellimeters;
        fn add(self, other: Meters) -> Mellimeters {
            Mellimeters(self.0 + (other.0 * 1000))
        }
    }
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
    // Rust string
    rust_string();
    // Rust data type
    rust_data_type();
    // Rust clousure
    rust_clousure();
    // Rust iterator
    rust_iterator();
    // Rust error handling
    assert_eq!(rust_error_handling().is_ok(), false);
    // Rust general type and trait
    rust_general_type_and_trait();
    // Rust lifetime
    rust_lifetime();
    // Rust test
    rust_test();
    // Rust smart pointer
    rust_smart_pointer();
    // Rust concurrent
    rust_concurrent();
    // Rust unsafe
    rust_unsafe();
    // Rust operator overload
    rust_operator_overload();
}
