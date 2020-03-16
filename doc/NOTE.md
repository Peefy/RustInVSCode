
# Rust笔记

## Rust安装

对于Linux或者macOS，使用终端并运行如下命令：

```
$ curl https://sh.rustup.rs -sSf | sh
```

对于Windows，[安装链接](https://www.rust-lang.org/tools/install)，以及VS2013或更高版本的C++生成工具，或者使用VS2019直接安装。

## 第一个Rust程序

```rust
fn main() {
    println!("Hello, world!");
}
```

注意，`println!`调用Rust宏。如果改为调用函数，则将其输入为`println（不带!）`

## Rust包管理器Cargo

Cargo是Rust的构建系统和包管理器。大多数Rustaceans使用此工具来管理他们的Rust项目，因为Cargo会为处理很多任务，例如构建代码，下载代码所依赖的库以及构建这些库。

Cargo.toml

```
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

第一行`[package]`是节标题，指示以下语句正在配置程序包。当向该文件添加更多信息时，将添加其他部分。

接下来的四行设置了Cargo编译程序所需的配置信息：名称，版本，编写者以及要使用的Rust版本。Cargo从的环境中获取的姓名和电子邮件信息，因此，如果该信息不正确，请立即修复此信息，然后保存文件。将edition在附录E中讨论密钥。

最后一行`[dependencies]`是该部分的开头，可以列出项目的任何依赖项。

## Rust程序进阶

```rust
use std::io; // 标准库

/* 主函数声明 */
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /* 可变变量 */
    let mut guess = String::new();

    /* 传变量的引用 */
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");  /* std::result::Result */

    /* 使用println!占位符打印值 */
    println!("You guessed: {}", guess);

    /* 定义不可变变量 */
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);

}
```

最后使用`cargo run`指令运行程序，使用`cargo build`构建项目，

**使用Cargo.lock文件确保可复制的内部版本**

Cargo具有一种机制，可确保或其他任何人每次构建代码时都可以重建相同的工件：Cargo将仅使用指定的依赖项版本，除非另有说明。例如，如果下周发布的版本为0.5.6的rand包

**产生一个随机数**

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

**比较猜测于秘密号码**

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
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
                break;
            }
        }
    }
}

```

## Rust基本概念

### Rust变量和可变性

默认情况下变量是不可变的。这是Rust推动编写代码的一种方式，它可以利用Rust提供的安全性和易并发性。但是，仍然可以选择使变量可变。让探讨一下Rust如何以及为什么鼓励支持不变性，以及为什么有时可能想要退出。

当变量是不可变的时，将值绑定到名称后，就无法更改该值。为了说明这一点，使用来 在项目目录中生成一个名为变量的新项目。

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;   //error!
    println!("The value of x is: {}", x);
}
```

#### 变量和常量之间的差异

无法更改变量的值可能使想起了大多数其他语言都具有的另一个编程概念：常量。像不可变变量一样，常量是绑定到名称且不允许更改的值，但是常量和变量之间存在一些差异。

首先，不允许使用`mut`常量。默认情况下，常量不仅是不可变的，它们始终是不可变的。

可以使用`const`关键字而不是关键字声明常量let，并且值的类型必须带有注释。

可以在任何范围（包括全局范围）中声明常量，这使它们对于许多代码部分需要了解的值很有用。

最后一个区别是，只能将常量设置为常量表达式，而不是函数调用的结果或只能在运行时计算的任何其他值。

这是一个常量声明的示例，其中常量的名称为 `MAX_POINTS`，其值设置为100,000。（Rust常量的命名约定是使用所有大写字母在单词之间使用下划线，并且可以在数字文字中插入下划线以提高可读性）

```rust
fn main() {
const MAX_POINTS: u32 = 100_000;
}
```

### Rust数据类型

Rust中的每个值都具有特定的数据类型，它告诉Rust指定了哪种数据，因此知道如何使用该数据。将研究两个数据类型子集：标量和复合。

请记住，Rust是一种静态类型的语言，这意味着它必须在编译时知道所有变量的类型。编译器通常可以根据值以及如何使用它来推断要使用的类型。在当许多类型是可能的，

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

#### 标量类型

标量类型表示一个单一的值。Rust具有四种主要的标量类型：整数，浮点数，布尔值和字符。可能会从其他编程语言中识别这些。让跳到他们在Rust中的工作方式。

##### 整数类型

一个整数是没有小数部分的数。在第2章中，使用了一种整数`u32`类型。此类型声明表示与之关联的值应为占用32位空间的无符号整数（有符号整数类型以开头`i`，而不是`u`）。表3-1显示了Rust中的内置整数类型。`Signed`和`Unsigned`列中的每个变体（例如`i16`）都可以用来声明整数值的类型。

长度|	符号|	无符号
-|-|-
8位|	i8|	u8
16位|	i16|	u16
32位|	i32|	u32
64位|	i64|	u64
128位|	i128|	u128

每个变体可以是有符号的也可以是无符号的，并且具有明确的大小。 带符号和无符号表示数字是负数还是正数，换句话说，数字是否需要带有正负号（带正负号），或者数字将永远是正数，因此可以不带正负号表示（未签名）。这就像在纸上写数字一样：当符号很重要时，数字上会显示加号或减号。但是，可以放心地假设数字为正数时，它不会显示任何符号。带符号的数字使用二进制补码表示存储。

每个带符号的变量可以存储\[-2^{n - 1}\]到\[ 2^{n-1}-1 \]之间的数字，其中n是变量使用的位数。因此，一个 `i8`可从存储-128至127，其等于-128到127无符号的变体可以存储从0编号.

此外，`isize`和`usize`类型取决于的程序所运行的计算机类型：如果使用的是64位体系结构，则为64位；如果使用的是32位体系结构，则为32位。

数字文字|	例
-|-
小数|	`98_222`
十六进制|	`0xff`
八进制|	`0o77`
二元|	`0b1111_0000`
字节（`u8`）|	`b'A'`

##### 浮点类型

Rust对于浮点数也有两种原始类型，即带小数点的数字。Rust的浮点类型为f32和f64，大小分别为32位和64位。默认类型是f64 因为在现代CPU上，它的速度与之大致相同，f32但精度更高。

##### 数值运算

Rust支持期望所有数字类型的基本数学运算：加，减，乘，除和余数。以下代码显示了如何在let语句中使用每个代码

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

##### 布尔类型

与大多数其他编程语言一样，Rust中的布尔类型具有两个可能的值：true和false。布尔值的大小为1个字节。Rust中的布尔类型使用指定bool。例如：

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

##### 字符类型

到目前为止，仅处理数字，但是Rust也支持字母。Rust的 char类型是该语言最原始的字母类型，下面的代码显示了一种使用它的方式。（请注意，char文字是用单引号指定的，而字符串文字是使用双引号的。）

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Rust的`char`类型为4个字节，代表Unicode标量值，这意味着它可以代表的不仅仅是ASCII。重音字母；中文，日文和韩文字符；表情符号 和零宽度空格char在Rust 中都是有效值。Unicode标值的范围从`U+0000`到 `U+D7FF`和`U+E000`到`U+10FFFF`包容性。但是，“字符”在Unicode中并不是真正的概念，因此对“字符”是什么的直觉可能与`char`Rust中的a 不一致。

##### 复合类型

复合类型可以将多个值组合为一种类型。Rust有两种原始的复合类型：元组和数组。

###### 元组类型

元组是一种将多种类型的值组合为一个复合类型的一般方法。元组的长度是固定的：声明后，它们的大小就无法增长或缩小。

通过在括号内编写逗号分隔的值列表来创建元组。元组中的每个位置都有一个类型，并且元组中不同值的类型不必相同。在此示例中，添加了可选的类型注释：

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```

程序首先创建一个元组并将其绑定到变量`tup`。然后，它使用带有`let`采取`tup`并把它变成三个独立的变量`x`，`y`和`z`。这称为解构，因为它将单个元组分为三部分。最后，程序将输出的值 `y`，即`6.4`。

除了通过模式匹配进行结构分解之外，还可以通过使用句点（.）和要访问的值的索引直接访问元组元素。例如：

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

###### 数组类型

收集多个值的另一种方法是使用数组。与元组不同，数组的每个元素都必须具有相同的类型。Rust中的数组与某些其他语言中的数组不同，因为Rust中的数组具有固定长度，例如元组。

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

当希望将数据分配在堆栈而不是堆上时（当在第4章中讨论堆栈和堆时），或者要确保始终拥有固定数量的元素时，数组很有用。但是，数组不像矢量类型那样灵活。载体是由标准库提供一个类似集合类型是允许生长或尺寸的缩小。如果不确定使用数组还是向量，则可能应该使用向量。第8章将更详细地讨论向量。

一个程序可能需要使用数组而不是向量，例如，该程序需要知道一年中各个月份的名称。这样的程序不太可能需要添加或删除月份，因此可以使用数组，因为知道它将始终包含12个元素：

```rust

#![allow(unused_variables)]
fn main() {
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
}
```

将使用方括号编写数组的类型，并且在方括号内包括每个元素的类型，分号，然后是数组中元素的数量，如下所示

```rust

#![allow(unused_variables)]
fn main() {
let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

在这里，`i32`是每个元素的类型。分号后的数字`5` 表示数组包含五个元素。

以这种方式编写数组的类型看起来类似于初始化数组的另一种语法：如果要创建一个数组，该数组的每个元素都包含相同的值，则可以指定初始值，后跟一个分号，然后指定长度数组放在方括号中，如下所示：

```rust

#![allow(unused_variables)]
fn main() {
let a = [3; 5];
}
```

名为的数组`a`将包含`5`将全部设置为`3`初始值的元素 。这与写作相同，`let a = [3, 3, 3, 3, 3]`;但更为简洁。

###### 访问数组元素

数组是在堆栈上分配的单个内存块。可以使用索引访问数组的元素，如下所示：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

```

在此示例中，名为的变量`first`将获得值`1`，因为这是`[0]`数组中index处的值。名为的变量`second`将从数组中的`2`索引获取值`[1]`。

### Rust函数

Rust代码中普遍存在函数。已经看到了该语言中最重要的功能之一：main函数，它是许多程序的入口点。还看到了fn关键字，该关键字使可以声明新功能。

Rust代码使用蛇形大小写作为函数和变量名的常规样式。在蛇形的情况下，所有字母均为小写，并在下划线分开单词。这是一个包含示例函数定义的程序：

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

```

Rust中的函数定义以`fn`函数名称开头，并在函数名称后带有一组括号。大括号告诉编译器函数体的开始和结束位置。

可以调用定义的任何函数，方法是输入其名称，然后输入一组括号。因为`another_function`是在程序中定义的，所以可以从`main`函数内部调用它。注意，在源代码中的函数`another_function` 之后定义`main`；本来也可以定义它。Rust不在乎在何处定义函数，只需在某个地方定义它们即可。

让开始一个新的名为函数的二进制项目，以进一步探索函数。将another_function示例放在src / main.rs中并运行它。应该看到以下输出：

#### 函数参数

也可以将函数定义为具有parameters，parameters是作为函数签名一部分的特殊变量。当函数具有参数时，可以为其提供这些参数的具体值。从技术上讲，具体值称为自变量，但是在随意的交谈中，人们倾向于将参数和自变量一词交替用于函数定义中的变量或调用函数时传递的具体值。

以下重写的版本`another_function`显示了Rust中的参数：

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

```

当希望一个函数具有多个参数时，请用逗号分隔参数声明，如下所示：

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

#### 函数实体包含语句和表达式

函数体由一系列可选地以表达式结尾的语句组成。到目前为止，仅介绍了没有结尾表达式的函数，但是已经将表达式视为语句的一部分。由于Rust是一种基于表达式的语言，因此这是需要理解的重要区别。其他语言没有相同的区别，因此让看看什么是语句和表达式以及它们的差异如何影响函数体。

实际上已经使用过语句和表达式。语句是执行某些操作且不返回值的指令。表达式的 计算结果为结果值。让看一些例子。

创建变量并使用`let`关键字为其分配值是一条语句。

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

是在这种情况下求值为的块`4`。该值绑定`y` 为`let`语句的一部分。请注意，该`x + 1`行的末尾没有分号，这与到目前为止所看到的大多数行不同。表达式不包括结尾分号。如果在表达式的末尾添加分号，则将其变成一条语句，然后该语句将不返回值。在接下来探索函数返回值和表达式时，请记住这一点。

#### 具有返回值的函数

函数可以将值返回到调用它们的代码中。没有命名返回值，但确实在箭头（`->`）后声明了它们的类型。在Rust中，函数的返回值与函数主体块中最终表达式的值同义。可以通过使用`return`关键字并指定值从函数中提前返回，但是大多数函数都隐式返回最后一个表达式。这是一个返回值的函数的示例：

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

```

函数中没有函数调用，宏甚至`let`语句，`five` 仅是数字`5`本身。在Rust中，这是一个非常有效的函数。请注意，函数的返回类型也指定为`-> i32`.

### Rust注释

所有程序员都努力使他们的代码易于理解，但是有时需要额外的说明。在这种情况下，程序员在其源代码中留下注释或 注释，编译器将忽略它们，但阅读源代码的人可能会发现有用。

这是一个简单的注释：

```rust

#![allow(unused_variables)]
fn main() {
// hello, world
}
```

### Rust控制流

一种`if`表达式允许分支根据条件的代码。提供一个条件，然后说：“如果满足此条件，请运行此代码块。如果不满足条件，请不要运行此代码块。”

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

可选地，还可以包括一个`else`表达式，在此处选择了该表达式，以在条件评估为false时为程序提供替代的代码块来执行。如果不提供`else`表达式且条件为假，则程序将跳过该if块并继续执行下一部分代码。

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

#### 使用if的let语句

因为if是表达式，所以可以在let 语句的右侧使用它，

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

#### 循环重复

`loop`,`while`,`for`

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

##### 从循环返回值

a的用途之一loop是重试知道可能会失败的操作，例如检查线程是否已完成其工作。但是，可能需要将该操作的结果传递给其余代码。为此，可以在break停止循环的表达式后添加要返回的值。该值将从循环中返回，因此可以使用它，如下所示：

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

##### 有条件循环 while

对于程序而言，在循环中评估条件通常很有用。当条件为真时，循环运行。当条件不再为真时，程序将调用break，从而停止循环。这个环型可使用的组合来实现loop，if，else，和break; 可以根据需要在程序中立即尝试。

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

##### 遍历一个集合 for

可以使用该while构造遍历集合的元素，例如数组。

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

for循环的安全性和简洁性使其成为Rust中最常用的循环构造。即使在想要多次运行某些代码的情况下（例如while清单3-3 中使用循环的倒计时示例），大多数Rustaceans也会使用for循环。这样做的方法是使用Range，这是标准库提供的一种类型，它按顺序生成所有数字，从一个数字开始到另一个数字之前结束。

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

### Rust所有权

Rust的主要特征是所有权。尽管该功能易于解释，但对其余语言有深远的影响。

所有程序必须在运行时管理它们使用计算机内存的方式。某些语言具有垃圾回收功能，该垃圾回收功能会在程序运行时不断寻找不再使用的内存。在其他语言中，程序员必须显式分配和释放内存。Rust使用第三种方法：通过所有权系统管理内存，该系统具有一组在编译时检查的规则。程序运行时，所有所有权功能都不会减慢其运行速度。

因为所有权是许多程序员的新概念，所以它确实需要一些时间来习惯。好消息是，对Rust和所有权系统的规则越有经验，就越能自然开发安全有效的代码。继续吧！

了解所有权后，将拥有坚实的基础，可以理解使Rust独树一帜的功能。在本章中，将通过一些针对非常常见的数据结构的示例来学习所有权：字符串。

在许多编程语言中，不必经常考虑堆栈和堆。但是在像Rust这样的系统编程语言中，值是在堆栈上还是在堆上对语言的行为以及为什么必须做出某些决定的影响更大。所有权的各个部分将在本章后面的堆栈和堆中进行介绍，因此这里是准备工作的简要说明。

堆栈和堆都是内存的一部分，的代码可在运行时使用，但是它们的结构不同。堆栈按获取值的顺序存储值，并以相反的顺序删除值。这称为后进先出。想想一堆盘子：添加更多盘子时，将它们放在堆的顶部，而当需要盘子时，从顶部取下一个盘子。从中间或底部添加或删除板都无法正常工作！添加数据称为压入堆栈，而删除数据称为弹出堆栈。

堆栈中存储的所有数据必须具有已知的固定大小。编译时大小未知或大小可能更改的数据必须存储在堆中。堆的组织性较差：将数据放在堆上时，需要一定数量的空间。操作系统在堆中找到一个足够大的空白点，将其标记为正在使用中，然后返回一个 指针，该指针是该位置的地址。此过程称为 在堆上分配，有时也简称为allocating。将值压入堆栈不被视为分配。由于指针是已知的固定大小，因此可以将指针存储在堆栈上，但是当需要实际数据时，必须遵循指针。

#### Rust所有权规则

* Rust中的每个值都有一个变量，称为其所有者。
* 一次只能有一个所有者。
* 当所有者超出范围时，该值将被删除

可以改变的字符串

```rust

#![allow(unused_variables)]
fn main() {
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
}

```

#### 内存和分配

对于字符串文字，在编译时就知道了内容，因此文本直接硬编码到最终的可执行文件中。这就是为什么字符串文字快速高效的原因。但是这些属性仅来自字符串文字的不变性。不幸的是，对于在编译时未知大小且在运行程序时大小可能会改变的每段文本，无法将内存块放入二进制文件中。

**变量与数据交互的方式：克隆**

```rust

#![allow(unused_variables)]
fn main() {
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
}

```

#### Rust可拷贝类型

* 所有整数类型，例如u32。
* 布尔类型，bool值true和false。
* 所有浮点类型，例如f64。
* 字符类型char。
* 元组（如果它们仅包含also的类型）Copy。例如， (i32, i32)是Copy，但(i32, String)不是。

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

```

#### 返回值和范围

返回值也可以转移所有权。

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

变量的所有权每次都遵循相同的模式：将值分配给另一个变量将其移动。当包含堆上数据的变量超出范围时，将清除该值，drop除非已将数据移至另一个变量所拥有。

拥有所有权然后返回所有功能的所有权有点乏味。如果要让函数使用值而不是所有权怎么办？令人十分烦恼的是，除了可能还想返回的函数主体所产生的任何数据之外，如果想要再次使用它，则还需要将返回的信息传递回去。

可以使用元组返回多个值，

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

#### Rust引用

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

*注意：使用`&`进行引用的反义词是解引用，这是通过解引用运算符来完成的`*`。*

如果变量在默认情况下是不可变的，引用也是如此，不允许修改引用的内容。

##### 可变引用

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

但是可变引用有一个很大的限制：只能在一个特定范围内对一个特定的数据进行一个可变引用。下面代码将失败：

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);

```

该限制允许突变，但是以非常受控的方式。这是新的Rustaceans苦苦挣扎的事情，因为大多数语言都允许随时更改。

具有此限制的好处是Rust可以防止在编译时发生数据争用。一个数据的比赛相似，竞争条件，当这三种行为的发生情况：

* 两个或多个指针同时访问相同的数据。
* 至少有一个指针用于写入数据。
* 没有用于同步对数据的访问的机制。

与往常一样，可以使用大括号创建新的范围，从而允许多个可变引用，而不能同时引用：

```rust
#![allow(unused_variables)]
fn main() {
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
}
```

##### 悬空引用

在带有指针的语言中，很容易错误地创建一个悬空指针，即通过在保留指向该内存的指针的同时释放一些内存来引用可能已分配给他人的内存中某个位置的指针。相比之下，在Rust中，编译器保证引用永远不会成为悬挂引用：如果对某些数据具有引用，则编译器将确保数据不会超出对数据的引用范围。

让尝试创建一个悬空的引用，Rust将通过编译时错误防止它：

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

这是错误:

```rust
error[E0106]: missing lifetime specifier
 --> main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

### Rust切片类型

slice也是另一个没有所有权的数据类型。切片使可以引用集合中连续的元素序列，而不是整个集合。

```rust

#![allow(unused_variables)]
fn main() {
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
}
```

```rust

#![allow(unused_variables)]
fn main() {
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
}
```

这类似于对整体进行引用，`String`但要多加一 `[0..5]`点点。而不是整个参考String，而是对的一部分的参考String。

通过指定`[starting_index..ending_index]`，可以使用方括号内的范围来创建切片 ，其中，`starting_index`是切片中的第一个位置，比切片中`ending_index`的最后一个位置大。在内部，切片数据结构存储切片的起始位置和长度，该长度与`ending_indexminus` 相对应`starting_index`。因此，在的情况下`let world = &s[6..11];`，`world`将是一个切片，该切片包含一个指向第7个字节（从1开始）的指针，其`s`长度值为5。

使用Rust的..range语法，如果要从第一个索引（零）开始，则可以在两个句点之前删除该值。换句话说，这些是相等的：

```rust

#![allow(unused_variables)]
fn main() {
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
}
```

同样，如果的分片包含的最后一个字节，则String可以删除尾随数字。这意味着这些是相等的：

```rust
#![allow(unused_variables)]
fn main() {
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
}
```

还可以删除两个值以截取整个字符串的一部分。所以这些是相等的：

```rust
#![allow(unused_variables)]
fn main() {
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
}
```

*注意：字符串切片范围索引必须出现在有效的UTF-8字符边界处。如果尝试在多字节字符的中间创建字符串片段，则程序将退出并显示错误。*

```rust

#![allow(unused_variables)]
fn main() {
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
}
```

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

这是编译器错误：

```rust
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here
```

##### 数组切片

```rust

#![allow(unused_variables)]
fn main() {
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
}
```

该切片具有类型`&[i32]`。通过存储对第一个元素和长度的引用，它的工作方式与字符串切片相同。

### Rust结构

要定义一个结构，输入关键字`struct`并命名整个结构。结构的名称应说明将数据分组在一起的重要性。然后，在大括号内，定义数据段的名称和类型，将其称为field。

```rust
#![allow(unused_variables)]
fn main() {
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
}
```

要在定义结构后使用结构，可以通过为每个字段指定具体值来创建该结构的实例。通过说明结构的名称来创建实例，然后添加包含`key: value`对的大括号，其中键是字段的名称，值是要存储在这些字段中的数据。不必按照在结构中声明它们的顺序来指定字段。换句话说，结构定义就像该类型的通用模板，实例用特定的数据填充该模板以创建该类型的值。

```rust

#![allow(unused_variables)]
fn main() {
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
}

```

#### 使用结构更新语法从其他实例创建实例

```rust

#![allow(unused_variables)]
fn main() {
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
}
```

#### 使用没有命名字段的元组结构创建不同的类型

还可以定义看起来类似于元组的结构，称为元组结构。元组结构具有附加的含义，即结构名称提供的含义，但没有与其字段关联的名称；相反，它们只是字段的类型。当想给整个元组起一个名字并使元组成为与其他元组不同的类型时，元组结构很有用，并且像常规结构中那样命名每个字段都是冗长或多余的。

要定义元组结构，请从`struct`关键字和结构名称开始，后跟元组中的类型。例如，以下是两个名为`Color`和的元组结构的定义和用法`Point`：

```rust
#![allow(unused_variables)]
fn main() {
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
}
```

请注意，`black`和`origin`值是不同的类型，因为它们是不同元组结构的实例。您定义的每个结构都是其自己的类型，即使该结构中的字段具有相同的类型。例如，即使两个类型都由三个值组成，`Color`带有类型参数的函数也不能将a `Point`作为参数`i32`。否则，元组`struct`实例的行为类似于元组：您可以将它们分解为各自的片段，可以使用.后跟索引的索引来访问单个值，依此类推。

#### 没有任何字段的类似单元的结构

您还可以定义没有任何字段的结构！这些之所以称为 单元状结构，是因为它们的行为类似于`()`单元类型。在需要在某种类型上实现特征但又不想在类型本身中存储任何数据的情况下，类似单元的结构很有用。

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

#### 使用结构的示例

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```

### Rust方法

方法类似于函数：它们用fn关键字及其名称声明，它们可以具有参数和返回值，并且它们包含一些从其他地方调用它们时将运行的代码。但是，方法与函数的不同之处在于，它们是在struct的上下文中定义的，并且它们的第一个参数始终为self，它表示调用该方法的struct实例。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

```rust
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

```rust

#![allow(unused_variables)]
fn main() {
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
}
```

的另一个有用的功能`impl`块的是，我们能定义范围内的功能impl块是不带self作为参数。这些被称为关联函数，因为它们与结构关联。它们仍然是函数，而不是方法，因为它们没有可使用的结构实例。您已经使用了String::from关联的功能。

关联函数通常用于将返回该结构的新实例的构造函数。例如，我们可以提供一个关联的函数，该函数将具有一个维度参数并将其用作宽度和高度，从而使创建正方形Rectangle而不是必须两次指定相同的值变得更加容易：

```rust

#![allow(unused_variables)]
fn main() {
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
}
```

每个结构允许具有多个`impl`块。

```rust

#![allow(unused_variables)]
fn main() {
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
}

```

### Rust枚举和模式匹配

```rust

#![allow(unused_variables)]
fn main() {
enum IpAddrKind {
    V4,
    V6,
}
}
```

我们可以`IpAddrKind`像这样创建两个变体的每一个的实例：

```rust

#![allow(unused_variables)]
fn main() {
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
}
```

请注意，枚举的变体在其标识符下命名空间，并且我们使用双冒号将两者分开。之所以有用，是因为现在两个值`IpAddrKind::V4`和`IpAddrKind::V6`都具有相同的类型： `IpAddrKind`。例如，我们然后可以定义一个接受任意值的函数 `IpAddrKind`：

```rust

#![allow(unused_variables)]
fn main() {
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) { }
}
```

```rust

#![allow(unused_variables)]
fn main() {
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
}
```

通过将数据直接放入每个枚举变量中，我们可以仅使用枚举而不是结构内部的枚举以更简洁的方式表示相同的概念。这个新的`IpAddr`枚举定义表示`V4`和`V6` 变体都将具有关联的`String`值：

```rust

#![allow(unused_variables)]
fn main() {
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
}
```

```rust

#![allow(unused_variables)]
fn main() {
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
}
```

```rust

#![allow(unused_variables)]
fn main() {
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
}
```

```rust

#![allow(unused_variables)]
fn main() {
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
}
```

```rust

#![allow(unused_variables)]
fn main() {
enum Option<T> {
    Some(T),
    None,
}
}
```

该`Option<T>`枚举是非常有用，它甚至包括中拉开序幕; 您无需将其明确纳入范围。此外，它的变体也是如此：您可以直接使用`Some`和`None`不带`Option::`前缀。该 `Option<T>`枚举仍然只是一个普通的枚举，并`Some(T)`和`None`类型仍然变种`Option<T>`。

该`<T>`语法是，我们还没有谈到尚锈的特点。这是一个泛型类型参数，我们将在第10章中更详细地介绍泛型。现在，您只需要知道，这`<T>`意味着枚举的Some变体 Option可以容纳任何类型的数据。以下是一些使用Option值保存数字类型和字符串类型的示例：

```rust

#![allow(unused_variables)]
fn main() {
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
}

```

#### Rust控制流运算符match

Rust具有一个非常强大的控制流运算符match，该运算符使您可以将值与一系列模式进行比较，然后根据匹配的模式执行代码。模式可以由文字值，变量名，通配符和许多其他内容组成；

```rust

#![allow(unused_variables)]
fn main() {
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
}
```

##### 搭配Option<T>

```rust

#![allow(unused_variables)]
fn main() {
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
}

```

##### `_`占位符

```rust

#![allow(unused_variables)]
fn main() {
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
}

```

### Rust模块

文件src.lib.rs

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

```

#### 使用pub关键字公开路径

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

```

#### 起始相对路径 super

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
fn main() {}

```

#### 公开结构和枚举

```rust

#![allow(unused_variables)]
fn main() {
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
}

```

#### 使用use关键字将路径纳入范围

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
fn main() {}

```

还可以通过`use`和相对路径将某项纳入范围。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
fn main() {}

```

#### 创建惯用use路径

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
fn main() {}

```

#### 使用as关键字提供新名称

```rust

#![allow(unused_variables)]
fn main() {
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
}

```

用以下方式重新导出名称 pub use,当我们使用use关键字将名称带入范围时，新范围中可用的名称是私有的。为了使调用我们代码的代码能够像在该代码范围内定义该名称一样引用该名称，我们可以将pub 和组合在一起use。这项技术称为重新导出，因为我们将某个项目纳入范围，同时也使该项目可供其他人进入其范围。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
fn main() {}

```

通过使用pub use，外部代码现在可以add_to_waitlist使用调用该函数hosting::add_to_waitlist。如果未指定pub use，则该 eat_at_restaurant函数可以hosting::add_to_waitlist在其作用域内调用，但是外部代码无法利用此新路径。

当代码的内部结构与调用代码的程序员对域的思考方式不同时，重新导出很有用。

#### 使用外部软件包

该项目使用一个名为的外部软件包rand来获取随机数。要rand在我们的项目中使用，我们将此行添加到Cargo.toml中：

```toml
[dependencies]
rand = "0.5.5"

```

```rust
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}

```

Rust社区的成员已经在crates.io上提供了许多软件包 ，

#### 使用嵌套路径清理大use列表

```rust

#![allow(unused_variables)]
fn main() {
use std::{cmp::Ordering, io};
// ---snip---
}

```

这两个路径的共同部分是std::io，这就是完整的第一个路径。要将这两个路径合并为一条use语句，我们可以使用self嵌套路径，

```rust

#![allow(unused_variables)]
fn main() {
use std::io::{self, Write};
}
r4
```

#### 全局运算符

如果要将路径中定义的所有公共项目都纳入范围，可以指定该路径，后跟*，全局运算符：

```rust

#![allow(unused_variables)]
fn main() {
use std::collections::*;
}
```

### 用向量存储值列表
