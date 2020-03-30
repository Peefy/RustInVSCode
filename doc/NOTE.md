
# Rust笔记

[官方中文教程](http://120.78.128.153/rustbook/title-page.html)

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

当希望将数据分配在堆栈而不是堆上时（当在第4章中讨论堆栈和堆时），或者要确保始终拥有固定数量的元素时，数组很有用。但是，数组不像矢量类型那样灵活。载体是由标准库提供一个类似集合类型是允许生长或尺寸的缩小。如果不确定使用数组还是容器，则可能应该使用容器。

一个程序可能需要使用数组而不是容器，例如，该程序需要知道一年中各个月份的名称。这样的程序不太可能需要添加或删除月份，因此可以使用数组，因为知道它将始终包含12个元素：

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

请注意，`black`和`origin`值是不同的类型，因为它们是不同元组结构的实例。定义的每个结构都是其自己的类型，即使该结构中的字段具有相同的类型。例如，即使两个类型都由三个值组成，`Color`带有类型参数的函数也不能将a `Point`作为参数`i32`。否则，元组`struct`实例的行为类似于元组：可以将它们分解为各自的片段，可以使用.后跟索引的索引来访问单个值，依此类推。

#### 没有任何字段的类似单元的结构

还可以定义没有任何字段的结构！这些之所以称为 单元状结构，是因为它们的行为类似于`()`单元类型。在需要在某种类型上实现特征但又不想在类型本身中存储任何数据的情况下，类似单元的结构很有用。

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

的另一个有用的功能`impl`块的是，能定义范围内的功能impl块是不带self作为参数。这些被称为关联函数，因为它们与结构关联。它们仍然是函数，而不是方法，因为它们没有可使用的结构实例。已经使用了String::from关联的功能。

关联函数通常用于将返回该结构的新实例的构造函数。例如，可以提供一个关联的函数，该函数将具有一个维度参数并将其用作宽度和高度，从而使创建正方形Rectangle而不是必须两次指定相同的值变得更加容易：

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

可以`IpAddrKind`像这样创建两个变体的每一个的实例：

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

请注意，枚举的变体在其标识符下命名空间，并且使用双冒号将两者分开。之所以有用，是因为现在两个值`IpAddrKind::V4`和`IpAddrKind::V6`都具有相同的类型： `IpAddrKind`。例如，然后可以定义一个接受任意值的函数 `IpAddrKind`：

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

通过将数据直接放入每个枚举变量中，可以仅使用枚举而不是结构内部的枚举以更简洁的方式表示相同的概念。这个新的`IpAddr`枚举定义表示`V4`和`V6` 变体都将具有关联的`String`值：

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

该`Option<T>`枚举是非常有用，它甚至包括中拉开序幕; 无需将其明确纳入范围。此外，它的变体也是如此：可以直接使用`Some`和`None`不带`Option::`前缀。该 `Option<T>`枚举仍然只是一个普通的枚举，并`Some(T)`和`None`类型仍然变种`Option<T>`。

该`<T>`语法是，还没有谈到尚锈的特点。这是一个泛型类型参数，将在第10章中更详细地介绍泛型。现在，只需要知道，这`<T>`意味着枚举的Some变体 Option可以容纳任何类型的数据。以下是一些使用Option值保存数字类型和字符串类型的示例：

```rust

#![allow(unused_variables)]
fn main() {
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
}

```

#### Rust控制流运算符match

Rust具有一个非常强大的控制流运算符match，该运算符使可以将值与一系列模式进行比较，然后根据匹配的模式执行代码。模式可以由文字值，变量名，通配符和许多其他内容组成；

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

用以下方式重新导出名称 pub use,当使用use关键字将名称带入范围时，新范围中可用的名称是私有的。为了使调用代码的代码能够像在该代码范围内定义该名称一样引用该名称，可以将pub 和组合在一起use。这项技术称为重新导出，因为将某个项目纳入范围，同时也使该项目可供其他人进入其范围。

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

该项目使用一个名为的外部软件包rand来获取随机数。要rand在的项目中使用，将此行添加到Cargo.toml中：

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

这两个路径的共同部分是std::io，这就是完整的第一个路径。要将这两个路径合并为一条use语句，可以使用self嵌套路径，

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

### 用容器存储值列表

要研究的第一个集合类型是容器`Vec<T>`。容器使可以在单个数据结构中存储多个值，该结构将所有值彼此相邻放置在内存中。

要创建一个新的空容器，可以调用该Vec::new函数，

```rust

#![allow(unused_variables)]
fn main() {
let v: Vec<i32> = Vec::new();
}

```

在更实际的代码中，Rust通常可以在插入值后就推断出要存储的值的类型，因此几乎不需要执行此类型注释。创建`Vec<T>`具有初始值的更为常见，Rust提供了该vec!宏以方便使用。宏将创建一个新容器，其中包含提供的值。

```rust

#![allow(unused_variables)]
fn main() {
let v = vec![1, 2, 3];
}
```

#### 更新vector

```rust

#![allow(unused_variables)]
fn main() {
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
}
```

#### 删除vector元素

```rust

#![allow(unused_variables)]
fn main() {
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v

} // <- v goes out of scope and is freed here
}

```

既然知道如何创建，更新和销毁容器，那么下一步就是了解如何读取容器的内容。有两种方法可以引用存储在容器中的值。

```rust

#![allow(unused_variables)]
fn main() {
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
}
```

Rust有两种引用元素的方式，因此可以选择在尝试使用向量没有元素的索引值时程序的行为。

```rust

#![allow(unused_variables)]
fn main() {
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
}

```

#### 遍历容器中的值

```rust

#![allow(unused_variables)]
fn main() {
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
}
```

#### 迭代对向量中元素的可变引用

```rust

#![allow(unused_variables)]
fn main() {
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
}
```

#### 定义将一个enum不同类型的值存储在一个向量中

```rust

#![allow(unused_variables)]
fn main() {
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
}

```

### Rust字符串

从new创建字符串的函数

```rust

#![allow(unused_variables)]
fn main() {
let mut s = String::new();
}
```

#### 使用该to_string方法String从字符串文字创建一个

```rust

#![allow(unused_variables)]
fn main() {
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
}

```

字符串是UTF-8编码的，因此可以在其中包含任何正确编码的数据，

```rust

#![allow(unused_variables)]
fn main() {
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
}
```

**更新字符串**

```rust

#![allow(unused_variables)]
fn main() {
let mut s = String::from("foo");
s.push_str("bar");
}

```

**与+运算符或format!宏串联**

```rust

#![allow(unused_variables)]
fn main() {
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

```

```rust

#![allow(unused_variables)]
fn main() {
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
}

```

**遍历字符串**

```rust

#![allow(unused_variables)]
fn main() {
for c in "नमस्ते".chars() {
    println!("{}", c);
}
}
```

```rust

#![allow(unused_variables)]
fn main() {
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
}

```

### Rust哈希

```rust

#![allow(unused_variables)]
fn main() {
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
}

```

```rust

#![allow(unused_variables)]
fn main() {
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

```

**哈希和所有权**

```rust

#![allow(unused_variables)]
fn main() {
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
}

```

**访问遍历哈希的值**

```rust

#![allow(unused_variables)]
fn main() {
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
}

```

```rust

#![allow(unused_variables)]
fn main() {
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
}

```

通常检查特定键是否具有值，如果没有，请为其插入值。哈希映射为此有一个特殊的API entry ，它将想要检查的键作为参数。该entry方法的返回值 是一个称为的枚举Entry，表示可能存在或可能不存在的值。假设要检查Yellow团队的密钥是否具有与其关联的值。如果不是，要插入值50，蓝色团队也要插入值50。使用entryAPI，

```rust

#![allow(unused_variables)]
fn main() {
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
}

```

**根据旧值更新值**

```rust

#![allow(unused_variables)]
fn main() {
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
}

```

#### Rust散列函数

默认情况下，HashMap使用“加密强度高”的1哈希函数，该函数可以抵抗拒绝服务（DoS）攻击。这不是可用的最快的哈希算法，但是性能下降带来的更好安全性的权衡是值得的。如果对代码进行概要分析并发现默认的哈希函数对于的目的而言太慢，则可以通过指定其他hasher来切换到另一个函数 。散列器是实现BuildHasher特征的一种类型。

### Rust错误处理

panic!宏引发错误

#### 可恢复错误Result

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}

```

**匹配不同的错误**

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

```

File::open在Err变体内部返回的值的类型为 io::Error，这是标准库提供的结构。该结构具有一个kind可以调用以获取io::ErrorKind值的方法。枚举 io::ErrorKind由标准库提供，并且具有表示io 操作可能导致的各种错误的变体。要使用的变体是ErrorKind::NotFound，表示要打开的文件尚不存在。所以匹配f，但也有一个内部匹配error.kind()。

要检查内部匹配项的条件是，返回的值是否为枚举error.kind()的NotFound变体ErrorKind。如果是这样，尝试使用创建文件File::create。但是，由于File::create 也可能失败，因此需要在内部match表达式中添加第二个分支。无法创建文件时，将输出其他错误消息。外部的第二个臂match保持不变，因此程序会因缺少文件错误而对任何错误感到错误。

好多match！该match表达式非常有用，但也非常原始。在第13章中，将学习闭包。该`Result<T, E>`类型具有许多接受闭包并使用match表达式实现的方法 。使用这些方法将使的代码更简洁。

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}

```

expect类似于的另一种方法，unwrap让也可以选择 panic!错误消息。使用expect代替unwrap并提供良好的错误消息可以传达的意图，并使跟踪错误的根源更加容易。

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

```

**传播错误**

当编写一个函数的实现调用可能失败的函数时，可以将错误返回给调用代码，以便它可以决定要做什么，而不是处理该函数中的错误。这被称为 传播错误，并赋予了调用代码更多的控制权，在这里，可能有更多的信息或逻辑规定了错误的处理方式，而不是在代码上下文中可用的信息或逻辑。

```rust

#![allow(unused_variables)]
fn main() {
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
}

```

**传播错误的捷径：?运算符**

```rust

#![allow(unused_variables)]
fn main() {
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
}

```

```rust

#![allow(unused_variables)]
fn main() {
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
}

```

将文件读入字符串是一个相当常见的操作，因此Rust提供了便捷的fs::read_to_string功能来打开文件，创建新文件 String，读取文件内容，将内容放入文件中String并返回。当然，使用fs::read_to_string不会给提供解释所有错误处理的机会，因此首先采用了更长的方法。

```rust

#![allow(unused_variables)]
fn main() {
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
}

```

让看看如果?在main函数中使用运算符会发生什么，会记得它的返回类型为()：

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}

```

```rust
error[E0277]: the `?` operator can only be used in a function that returns
`Result` or `Option` (or another type that implements `std::ops::Try`)
 --> src/main.rs:4:13
  |
4 |     let f = File::open("hello.txt")?;
  |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a
  function that returns `()`
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`

```

该错误指出只允许?在返回的函数`Result`或`Option`实现的其他类型中 使用运算符`std::ops::Try`。当在不返回这些类型之一的函数中编写代码时，并且要?在调用返回其他函数的其他函数时使用`Result<T, E>`，有两种选择可以解决此问题。一种技术是将函数的返回类型更改为`Result<T, E>`在没有限制的情况下返回。另一种技术是使用一种match或多种`Result<T, E>`方法以`Result<T, E>`适当的方式进行处理。

该main函数是特殊的，并且对它的返回类型必须有限制。main的一种有效返回类型是()，并且方便地，另一种有效返回类型是Result<T, E>，如下所示：

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

该`Box<dyn Error>`类型称为特征对象，

### 是否使用panic!函数

```rust

#![allow(unused_variables)]
fn main() {
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
}

```

IpAddr通过解析硬编码字符串来创建实例。可以看到这127.0.0.1是一个有效的IP地址，因此可以在unwrap 此处使用。但是，使用经过硬编码的有效字符串不会更改方法的返回类型parse：仍然会获得一个Result值，并且编译器仍将使能够Result像处理Err变体那样处理该变量，因为编译器不够聪明，无法看到此字符串始终是有效的IP地址。如果IP地址字符串来自用户而不是硬编码到程序中，因此确实有失败的可能性，肯定希望以Result更可靠的方式处理它。

```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // --snip--
}

```

可以创建一个新类型并将验证放入函数中以创建该类型的实例，而不是在所有地方重复进行验证。这样，函数在签名中使用新类型并放心使用其接收的值是安全的。

```rust

#![allow(unused_variables)]
fn main() {
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
}

```

### 通用类型，特征和寿命

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
 assert_eq!(largest, 100);
}

```

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

```

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

```

如果现在编译此代码，则会收到此错误：

```rust
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:12
  |
5 |         if item > largest {
  |            ^^^^^^^^^^^^^^
  |
  = note: an implementation of `std::cmp::PartialOrd` might be missing for `T`

```

还可以使用`<>`语法在一个或多个字段中定义结构以使用通用类型参数。

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

```

在结构定义中使用泛型的语法类似于在函数定义中使用的语法。首先，在结构名称之后的尖括号内声明类型参数的名称。然后，可以在结构定义中使用泛型类型，否则将指定具体的数据类型。

请注意，因为只用一个泛型类型定义`Point<T>`，这个定义说，`Point<T>`结构是通用在某种类型的T，和田野x，并y有两个同类型的，无论该类型而定。如果创建一个`Point<T>`具有不同类型值的实例，代码将无法编译。

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

```

可以定义枚举以将通用数据类型保存在它们的变量中。看看`Option<T>`和`Result<T, E>`标准库提供的枚举，

```rust

#![allow(unused_variables)]
fn main() {
enum Option<T> {
    Some(T),
    None,
}
}
```

```rust

#![allow(unused_variables)]
fn main() {
enum Result<T, E> {
    Ok(T),
    Err(E),
}
}

```

**范型方法**

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

可以仅在`Point<f32>`实例上实现方法，而不能在`Point<T>`具有任何泛型类型的实例上实现方法。

```rust

#![allow(unused_variables)]
fn main() {
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
}

```

结构定义中的泛型类型参数并不总是与该结构的方法签名中使用的参数相同。例如，

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

```

Rust通过在编译时对使用泛型的代码进行单态化来实现这一点。单色化是通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

在此过程中，

### 特性：定义共同的行为

类型的行为由可以在该类型上调用的方法组成。如果可以对所有这些类型调用相同的方法，则不同的类型具有相同的行为。特性定义是一种将方法签名组合在一起以定义实现某些目的所需的行为的方法。

举例来说，假设有多种结构，可容纳各种类型和数量的文本：一种NewsArticle结构，可容纳在特定位置归档的新闻报导，并且结构Tweet最多可包含280个字符以及指示它是否为新推文的元数据，转发或回复其他推文。

想要制作一个媒体聚合器库，以显示可能存储在NewsArticle或Tweet实例中的数据摘要。为此，需要每种类型的摘要，并且需要通过summarize在实例上调用方法来请求该摘要 。

```rust

#![allow(unused_variables)]
fn main() {
pub trait Summary {
    fn summarize(&self) -> String;
}
}

```

#### 在类型上实现特质

```rust

#![allow(unused_variables)]
fn main() {
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
}

```

#### 特性作为参数

```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

```

#### 特性绑定语法

```rust
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

```

该impl Trait语法是方便，使得在简单的情况下，更简洁的代码。在其他情况下，特征绑定语法可以表示更多的复杂性。例如，可以有两个实现的参数Summary。使用impl Trait语法如下所示：

```rust
pub fn notify(item1: impl Summary, item2: impl Summary) {

```

或者

```rust
pub fn notify<T: Summary>(item1: T, item2: T) {

```

**使用+语法指定多个特征界线**

```rust
pub fn notify(item: impl Summary + Display) {

```

```rust
pub fn notify<T: Summary + Display>(item: T) {

```

**具有where条款的更清晰的特质界限**

```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

```

还可以impl Trait在返回位置使用语法来返回实现特征的某种类型的值，

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

```

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

```

过使用与impl使用通用类型参数的块绑定的特征，可以有条件地为实现指定特征的类型实现方法。例如，`Pair<T>`类型始终实现该 new功能。但`Pair<T>`仅实现cmp_display方法，如果其内型T器具的PartialOrd，使比较性状和 所述Display使打印性状。

```rust

#![allow(unused_variables)]
fn main() {
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
}

```

对于任何实现了另一个特征的类型，也可以有条件地实现一个特征。满足特征界限的任何类型的特征实现都称为覆盖实现，并且在Rust标准库中得到了广泛的使用。例如，标准库可以在实现 ToString特征的任何类型上实现Display特征。impl 标准库中的块类似于以下代码：

```rust
impl<T: Display> ToString for T {
    // --snip--
}

```

因为标准库具有这种全面的实现，所以可以在实现该特性的任何类型上调用to_string由ToStringtrait 定义的 方法Display。例如，可以String像这样将整数转换为其相应的 值，因为整数实现Display：

```rust

#![allow(unused_variables)]
fn main() {
let s = 3.to_string();
}

```

特质和特征边界使可以编写使用通用类型参数减少重复的代码，同时还向编译器指定希望通用类型具有特定行为。然后，编译器可以使用特征绑定信息来检查与的代码一起使用的所有具体类型是否提供正确的行为。在动态类型的语言中，如果在未实现定义该方法的类型的类型上调用方法，则在运行时会出错。但是Rust会将这些错误转移到编译时，因此不得不在代码无法运行之前解决问题。另外，不必编写在运行时检查行为的代码，因为已经在编译时进行了检查。这样做可以提高性能，而不必放弃泛型的灵活性。

### 使用生命周期验证引用

生命周期的主要目的是防止悬而未决的引用，这些引用使程序引用的不是其要引用的数据。考虑下面的程序，它具有一个外部作用域和一个内部作用域。

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

外部作用域声明一个r没有初始值的变量，内部作用域声明一个x初始值5 的变量。在内部作用域内，尝试将的值设置r为对的引用x。然后内部作用域结束，尝试在中打印值r。该代码不会编译，因为r在尝试使用该值之前，该值已超出范围。这是错误消息：

```rust
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:5
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here

```

#### 借用检查器

Rust编译器有一个借用检查器，可以比较作用域以确定所有借用是否有效。下面代码与上面代码相同，但带有注释，显示了变量的生存期。

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

#### 函数的通用生命周期

让编写一个返回两个字符串切片中较长者的函数。此函数将获取两个字符串切片并返回一个字符串切片。实现longest函数后，

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

#### 终身注释语法

终身注释不会更改任何引用的生存时间。正如签名可以指定通用类型参数时函数可以接受任何类型一样，通过指定通用寿命参数，函数可以接受具有任何生存期的引用。生存期批注描述了多个引用的生存期彼此之间的关系，而不会影响生存期。

生命周期注释的语法略有不同：生命周期参数的名称必须以撇号（'）开头，并且通常都是小写且非常短，就像泛型类型一样。大多数人使用这个名字'a。将生命周期参数注释放置在&引用的后面，并使用空格将注释与引用的类型分开。

以下是一些示例：对i32不带生命周期参数的的引用，对i32具有生命周期参数名为的的'a引用以及i32对也具有生命周期的的可变引用'a。

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

#### 方法定义中的生命周期注释

在具有生命周期的结构上实现方法时，使用与清单10-11中所示的泛型类型参数相同的语法。声明和使用生命周期参数的位置取决于它们是否与struct字段或方法参数以及返回值相关。

始终需要在impl 关键字之后声明结构字段的生命周期名称，然后在结构名称之后使用，因为这些生命周期是结构类型的一部分。

在impl块内的方法签名中，引用可能与结构字段中引用的生命周期相关，或者它们可能是独立的。此外，生存期省略规则通常使之成为必需，因此在方法签名中不需要生存期批注。让看一下使用ImportantExcerpt清单10-25中定义的命名结构的一些示例。

首先，将使用名为level的方法，该方法的唯一参数是对的引用， self而返回值是i32，而不是对任何内容的引用：

```rust

#![allow(unused_variables)]
fn main() {
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
}
```

impl要求在类型名称后使用生命周期参数声明以及在类型名称后使用它，但是self由于第一个省略规则，不需要注释引用的生命周期。

这是第三个生存期删除规则适用的示例：

```rust

#![allow(unused_variables)]
fn main() {
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
}

```

有两个输入寿命，所以锈病应用第一生存法则省音并给出了两个&self和announcement自己的寿命。然后，由于参数之一是&self，返回类型的生存期为&self，并且所有生存期均已考虑在内。

#### 静态寿命

需要讨论的一个特殊寿命是'static，这意味着该引用可以在程序的整个过程中有效。所有字符串文字都有'static生命周期，可以如下注释：

```rust

#![allow(unused_variables)]
fn main() {
let s: &'static str = "I have a static lifetime.";
}

```

该字符串的文本直接存储在程序的二进制文件中，该二进制文件始终可用。因此，所有字符串文字的生存期为 'static。

可能会'static在错误消息中看到有关使用生存期的建议。但是在指定'static作为参考的生存期之前，请考虑一下所拥有的参考是否真正存在于程序的整个生存期中。可能会考虑是否希望它寿命这么长，即使可以。在大多数情况下，问题是由于尝试创建悬空参考或可用寿命不匹配而导致的。在这种情况下，解决方案将解决这些问题，而不指定'static寿命。

#### 通用类型参数，特质界限和寿命

简要地看一下在一个函数中指定泛型类型参数，特征范围和生存期的语法！

```rust

#![allow(unused_variables)]
fn main() {
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
}
```

### 编写自动化测试

Edsger W. Dijkstra在1972年的论文“谦虚的程序员”中说：“程序测试可以是显示错误存在的非常有效的方法，但是它不足以显示错误的存在。” 这并不意味着不应该尝试尽可能多地进行测试！

程序中的正确性是的代码按预期执行的程度。Rust的设计高度关注程序的正确性，但是正确性很复杂并且不容易证明。Rust的类型系统承担了很大的负担，但是类型系统无法捕获各种错误。因此，Rust支持使用该语言编写自动软件测试。

例如，假设编写了一个名为的函数add_two，该函数会将传递给它的任何数字加2。该函数的签名接受一个整数作为参数，并返回一个整数作为结果。当实现并编译该函数时，Rust会进行到目前为止所学的所有类型检查和借位检查，以确保例如，不会String对该函数传递值或无效引用。但是Rust 无法检查此函数是否能够准确执行的预期，即返回参数加2而不是参数加10或参数减50！那就是测试的地方。

可以编写测试来断言例如，当传递3给 add_two函数时，返回的值为5。只要对代码进行更改，就可以运行这些测试，以确保任何现有的正确行为都没有更改。

测试是一项复杂的技能：尽管不能在一章中涵盖有关如何编写良好测试的所有细节，但将讨论Rust测试设施的机制。将讨论编写测试时为提供的注释和宏，为运行测试提供的默认行为和选项，以及如何将测试组织为单元测试和集成测试。

测试是Rust函数，用于验证非测试代码是否按预期方式工作。测试功能的主体通常执行以下三个操作：

* 设置任何需要的数据或状态。
* 运行要测试的代码。
* 断言结果就是所期望的。
* 
让看一下Rust为编写执行这些操作的测试而专门提供的功能，这些测试包括`test`属性，一些宏和 `should_panic`属性。

```rust
fn main() {}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

```

函数体使用assert_eq!宏声明2 + 2等于4。此声明用作典型测试格式的示例。让运行它以查看此测试是否通过。

该`cargo test`命令运行项目中的所有测试，

```rust
fn main() {}
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

```

assert!当要确保测试中的某些条件求和时，标准库提供的宏非常有用true。给 assert!宏一个参数，其结果为布尔值。如果值为 true，assert!则不执行任何操作，测试通过。如果值为false，则assert!宏将调用该panic!宏，这将导致测试失败。使用assert!宏有助于检查代码是否按预期的方式运行。

```rust
fn main() {}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

```

```rust
fn main() {}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

```

```rust
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }
}

```

到目前为止，已经编写了在失败时会出现异常的测试。也可以编写使用的测试`Result<T, E>`！这是清单11-1中的测试，被重写为使用`Result<T, E>`并返回a Err而不是异常：

```rust

#![allow(unused_variables)]
fn main() {
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
}

```

#### 控制测试的运行方式

就像cargo run编译代码然后运行生成的二进制文件一样， cargo test在测试模式下编译代码并运行生成的测试二进制文件。可以指定命令行选项来更改的默认行为 cargo test。例如，生成的二进制文件的默认行为 cargo test是并行运行所有测试，并捕获测试运行期间生成的输出，从而阻止显示输出，并使读取与测试结果相关的输出更加容易。

有些命令行选项转到cargo test，有些转到生成的测试二进制文件。要分隔这两种类型的参数，请列出要传递的参数，cargo test后跟分隔符--，然后列出要传递至测试二进制文件的参数。运行cargo test --help显示可以使用的选项cargo test，运行cargo test -- --help显示在分隔符之后使用的选项--。

##### 并行或连续运行测试

当运行多个测试时，默认情况下，它们使用线程并行运行。这意味着测试将更快地完成运行，因此可以更快地获得有关代码是否正常工作的反馈。由于测试是同时运行的，因此请确保的测试不相互依赖，也不依赖任何共享状态，包括共享环境，例如当前的工作目录或环境变量。

例如，假设的每个测试都运行一些代码，这些代码会在磁盘上创建一个名为test-output.txt的文件，并将一些数据写入该文件。然后，每个测试读取该文件中的数据，并断言该文件包含一个特定值，该值在每个测试中都不同。因为测试是同时运行的，所以在另一个测试写入和读取文件之间，一个测试可能会覆盖文件。然后，第二个测试将失败，这不是因为代码不正确，而是因为在并行运行时测试之间相互干扰。一种解决方案是确保每个测试都写入不同的文件。另一种解决方案是一次运行一次测试。

如果不想并行运行测试，或者想要对使用的线程数进行更细粒度的控制，则可以将--test-threads标志和要使用的线程数发送到测试二进制文件。看下面的例子：

```
$ cargo test -- --test-threads=1
```

#### 显示功能输出

默认情况下，如果测试通过，Rust的测试库将捕获打印到标准输出的所有内容。例如，如果调用println!一个测试并且测试通过，则println!在终端中将看不到输出。将仅看到指示测试通过的行。如果测试失败，将在其余失败消息中看到打印到标准输出的所有内容。

例如，清单11-10具有一个傻函数，该函数打印其参数的值并返回10，以及通过和失败的测试。

```rust

#![allow(unused_variables)]
fn main() {
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
}
```

#### 按名称运行测试子集

有时，运行完整的测试套件可能需要很长时间。如果在特定区域中处理代码，则可能只想运行与该代码有关的测试。可以通过传递cargo test要作为参数运行的测试名称来选择要运行的测试。

```rust

#![allow(unused_variables)]
fn main() {
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
}

```

#### 运行单项测试

可以传递任何测试函数的名称cargo test以仅运行该测试：

```rust
$ cargo test one_hundred
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out

```

#### 除非特别要求，否则忽略某些测试

有时执行一些特定的测试可能非常耗时，因此可能希望在大多数运行期间将其排除cargo test。可以使用ignore属性将它们排除在外，而不是将想运行的所有测试都列为参数，如下所示：

```rust

#![allow(unused_variables)]
fn main() {
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
}

```

### 测试组织

如本章开头所述，测试是一门复杂的学科，不同的人使用不同的术语和组织。Rust社区从两个主要类别来考虑测试：单元测试和 集成测试。单元测试体积小且重点突出，可以一次单独测试一个模块，并且可以测试专用接口。集成测试完全在的库外部，并且以与其他任何外部代码相同的方式使用的代码，仅使用公共接口，并且每个测试可能使用多个模块。

编写两种测试对于确保库中的各个部分分别或一起执行预期的工作非常重要。

#### 单元测试

单元测试的目的是与其余代码隔离地测试每个代码单元，以快速查明代码在哪里正常工作和不正常工作。将使用它们正在测试的代码将单元测试放在每个文件的src目录中。约定是tests 在每个文件中创建一个命名模块以包含测试功能，并使用对其进行注释 cfg(test)。

##### 测试模块和 `#[cfg(test)]`

该`#[cfg(test)]`测试模块上的注解告诉锈编译只有当运行运行测试代码cargo test，而不是当运行cargo build。当只想构建库时，这样可以节省编译时间，并且由于不包括测试，因此可以节省生成的编译工件中的空间。会看到，由于集成测试位于不同的目录中，因此它们不需要`#[cfg(test)]`注释。但是，由于单元测试与代码位于相同的文件中，因此将用于`#[cfg(test)]`指定不应将其包含在编译结果中。

```rust

#![allow(unused_variables)]
fn main() {
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
}

```

此代码是自动生成的测试模块。该属性cfg 代表配置，并告诉Rust仅在具有特定配置选项的情况下才应包括以下项目。在这种情况下，配置选项为test，由Rust提供，用于编译和运行测试。通过使用该cfg属性，仅当使用积极运行测试时，Cargo才会编译测试代码cargo test。除了使用注释的功能外，它还包括此模块中可能包含的所有帮助程序功能`#[test]`。

##### 测试私有功能

在测试社区中，是否应直接测试私有功能存在争论，而其他语言使测试私有功能变得困难或不可能。无论遵循哪种测试思想，Rust的隐私规则都可以让测试私有功能。

```rust
fn main() {}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

```

#### 整合测试

在Rust中，集成测试完全在的库外部。他们以与其他任何代码相同的方式使用的库，这意味着它们只能调用属于库的公共API的函数。它们的目的是测试库中的许多部分是否可以正常协同工作。单独工作的代码单元在集成时可能会出现问题，因此测试集成代码的覆盖范围也很重要。要创建集成测试，首先需要一个tests目录。

```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

```

##### 集成测试中的子模块

当添加更多集成测试时，可能希望在tests目录中制作多个文件来帮助组织它们。例如，可以按测试功能对测试功能进行分组。如前所述，tests目录中的每个文件都被编译为自己的单独板条箱。

将每个集成测试文件视为自己的板条箱对于创建单独的作用域非常有用，这些范围更像最终用户将使用的板条箱。但是，这意味着tests目录中的文件与src中的文件不具有相同的行为，正如在第7章中了解的有关如何将代码分为模块和文件的知识一样。

当具有一组在多个集成测试文件中有用的帮助程序功能并且尝试按照 第7章的“将模块分为不同的文件”一节中的步骤操作时，tests目录中文件的不同行为最为明显。将它们提取到一个通用模块中。例如，如果创建tests / common.rs并在其中放置一个命名函数，则可以在多个测试文件中添加要从多个测试函数调用的代码：setupsetup

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

```

### I/O项目：构建命令行程序

#### 接受命令行参数

##### 读取参数值

需要一个Rust标准库中提供的函数 std::env::args。

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

```

*注意：请注意，std::env::args如果任何参数包含无效的Unicode ，将感到异常。如果的程序需要接受包含无效Unicode的参数，请std::env::args_os改用。该函数返回一个迭代器，该迭代器生成OsString值而不是String值。之所以选择std::env::args此处为简单起见，因为OsString每个平台的值不同，并且使用起来String比值更复杂。*

#### 将参数值保存在变量中

打印参数向量的值说明该程序能够访问指定为命令行参数的值。现在需要将两个参数的值保存在变量中，以便可以在程序的其余部分中使用这些值。

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}

```

正如看到的那样，在打印矢量时，程序的名称占据了矢量中的第一个值`args[0]`，因此从index开始1。第一个参数minigrep是要搜索的字符串，因此在变量中引用了第一个参数query。第二个参数是文件名，因此在变量中引用了第二个参数filename。

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

```

#### 提取参数解析器

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

```

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

```

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

```

##### 改善错误信息

```rust
// --snip--
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // --snip--

```

```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

```

###### 调用Config::new和处理错误

```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--

```

在此程序中，使用了以前没有涉及的方法： unwrap_or_else，它是`Result<T, E>`由标准库定义的。使用unwrap_or_else允许定义一些自定义的，非panic!错误的处理。如果Result是一个Ok值，则此方法的行为类似于unwrap：它返回Ok包装的内部值。但是，如果值是一个Err值，则此方法将调用闭包中的代码，该闭包是定义的匿名函数，并作为参数传递给unwrap_or_else。

#### 从中提取逻辑 main

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// --snip--

```

#### 从run函数返回错误

将其余的程序逻辑分离到run函数中，可以像Config::new清单12-9中那样改进错误处理。当出现问题时expect，run 函数将返回a ，而不是通过调用使程序异常`Result<T, E>`。这将使进一步main以用户友好的方式整合到处理错误的逻辑中。

```rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

```

#### 处理错误返回从run在main

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

```

使用if let而不是unwrap_or_else检查是否run返回Err值，然后调用返回 值process::exit(1)。该run函数不会以 返回实例unwrap的相同方式Config::new返回想要的值Config。因为在成功情况下run返回()，所以只关心检测错误，因此不需要unwrap_or_else返回展开的值，因为它只会是()。

#### 将代码拆分为不同的文件

```rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}

```

```rust
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}

```

### 通过测试驱动开发来开发库的功能

使用测试驱动的开发（TDD）流程将搜索逻辑添加到程序中。此软件开发技术遵循以下步骤：

1. 编写一个失败的测试并运行它，以确保由于期望的原因而失败。
2. 编写或修改足够的代码以使新的测试通过。
3. 重构刚刚添加或更改的代码，并确保测试继续通过。
4. 从步骤1开始重复！

此过程只是编写软件的许多方法之一，但是TDD也可以帮助驱动代码设计。在编写使测试通过的代码之前编写测试有助于在整个过程中保持较高的测试覆盖率。

#### 编写失败的测试

```rust

#![allow(unused_variables)]
fn main() {
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
     vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
}

```

```rust

#![allow(unused_variables)]
fn main() {
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
}

```

要'a在签名中定义的显式生存期， search并与contents参数和返回值一起使用。

#### 编写代码通过测试

目前，的测试失败了，因为总是返回一个空向量。要解决此问题并实施search，的程序需要遵循以下步骤：

* 遍历内容的每一行。
* 检查该行是否包含的查询字符串。
* 如果是这样，请将其添加到要返回的值列表中。
* 如果没有，则什么也不做。
* 返回匹配的结果列表。

让完成每个步骤，从遍历行开始。

##### 通过该lines方法迭代线

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

##### 在每一行中搜索查询

接下来，将检查当前行是否包含查询字符串。幸运的是，字符串有一个有用的名为的方法contains，可以为做到这一点！contains在search函数中添加对方法的调用

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
        }
    }
}

```

##### 存储匹配的行

还需要一个方法来存储包含查询字符串的行。为此可以在 for 循环之前创建一个可变的 vector 并调用 push 方法在 vector 中存放一个 line。在 for 循环之后，返回这个 vector，

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

```

##### 在 run 函数中使用 search 函数

现在 search 函数是可以工作并测试通过了的，需要实际在 run 函数中调用 search。需要将 config.query 值和 run 从文件中读取的 contents 传递给 search 函数。接着 run 会打印出 search 返回的每一行：

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

```

#### 编写一个大小写不敏感 search 函数的失败测试

希望增加一个新函数 search_case_insensitive，并将会在设置了环境变量时调用它。这里将继续遵循 TDD 过程，其第一步是再次编写一个失败测试。将为新的大小写不敏感搜索函数新增一个测试函数，并将老的测试函数从 one_result 改名为 case_sensitive 来更清楚的表明这两个测试的区别，

```rust

#![allow(unused_variables)]
fn main() {
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
}
```

注意也改变了老测试中 contents 的值。还新增了一个含有文本 "Duct tape." 的行，它有一个大写的 D，这在大小写敏感搜索时不应该匹配 "duct"。修改这个测试以确保不会意外破坏已经实现的大小写敏感搜索功能；这个测试现在应该能通过并在处理大小写不敏感搜索时应该能一直通过。

大小写 不敏感 搜索的新测试使用 "rUsT" 作为其查询字符串。在将要增加的 search_case_insensitive 函数中，"rUsT" 查询应该包含带有一个大写 R 的 "Rust:" 还有 "Trust me." 这两行，即便他们与查询的大小写都不同。这个测试现在会编译失败因为还没有定义 search_case_insensitive 函数。请随意增加一个总是返回空 vector 的骨架实现，

#### 实现 search_case_insensitive 函数

search_case_insensitive 函数，如下所示，将与 search 函数基本相同。唯一的区别是它会将 query 变量和每一 line 都变为小写，这样不管输入参数是大写还是小写，在检查该行是否包含查询字符串时都会是小写。

```rust

#![allow(unused_variables)]
fn main() {
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
}
```

首先将 query 字符串转换为小写，并将其覆盖到同名的变量中。对查询字符串调用 to_lowercase 是必需的，这样不管用户的查询是 "rust"、"RUST"、"Rust" 或者 "rUsT"，都将其当作 "rust" 处理并对大小写不敏感。

注意 query 现在是一个 String 而不是字符串 slice，因为调用 to_lowercase 是在创建新数据，而不是引用现有数据。如果查询字符串是 "rUsT"，这个字符串 slice 并不包含可供使用的小写的 u 或 t，所以必需分配一个包含 "rust" 的新 String。现在当将 query 作为一个参数传递给 contains 方法时，需要增加一个 & 因为 contains 的签名被定义为获取一个字符串 slice。

接下来在检查每个 line 是否包含 search 之前增加了一个 to_lowercase 调用将他们都变为小写。现在将 line 和 query 都转换成了小写，这样就可以不管查询的大小写进行匹配了。

```rust

#![allow(unused_variables)]
fn main() {
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
}
```

这里增加了 case_sensitive 字符来存放一个布尔值。接着需要 run 函数检查 case_sensitive 字段的值并使用它来决定是否调用 search 函数或 search_case_insensitive 函数，

```rust

#![allow(unused_variables)]
fn main() {
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
     vec![]
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
     vec![]
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
}
```

最后需要实际检查环境变量。处理环境变量的函数位于标准库的 env 模块中，所以需要在 src/lib.rs 的开头增加一个 use std::env; 行将这个模块引入作用域中。接着在 Config::new 中使用 env 模块的 var 方法来检查一个叫做 CASE_INSENSITIVE 的环境变量，

```rust

#![allow(unused_variables)]
fn main() {
use std::env;
struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

// --snip--

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
}
```

这里创建了一个新变量 case_sensitive。为了设置它的值，需要调用 env::var 函数并传递需要寻找的环境变量名称，CASE_INSENSITIVE。env::var 返回一个 Result，它在环境变量被设置时返回包含其值的 Ok 成员，并在环境变量未被设置时返回 Err 成员。

使用 Result 的 is_err 方法来检查其是否是一个 error（也就是环境变量未被设置的情况），这也就意味着 需要 进行一个大小写敏感搜索。如果CASE_INSENSITIVE 环境变量被设置为任何值，is_err 会返回 false 并将进行大小写不敏感搜索。并不关心环境变量所设置的 值，只关心它是否被设置了，所以检查 is_err 而不是 unwrap、expect 或任何已经见过的 Result 的方法。

将变量 case_sensitive 的值传递给 Config 实例，这样 run 函数可以读取其值并决定是否调用 search 或者示例 12-22 中实现的 search_case_insensitive。

#### 检查错误应该写入何处

首先，让观察一下目前 minigrep 打印的所有内容是如何被写入标准输出的，包括那些应该被写入标准错误的错误信息。可以通过将标准输出流重定向到一个文件同时有意产生一个错误来做到这一点。没有重定向标准错误流，所以任何发送到标准错误的内容将会继续显示在屏幕上。

命令行程序被期望将错误信息发送到标准错误流，这样即便选择将标准输出流重定向到文件中时仍然能看到错误信息。目前的程序并不符合期望；相反将看到它将错误信息输出保存到了文件中。

#### 将错误打印到标准错误

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

```

### Rust 中的函数式语言功能：迭代器与闭包

Rust 的设计灵感来源于很多现存的语言和技术。其中一个显著的影响就是 函数式编程（functional programming）。函数式编程风格通常包含将函数作为参数值或其他函数的返回值、将函数赋值给变量以供之后执行等等。

本章不会讨论函数式编程是或不是什么的问题，而是展示 Rust 的一些在功能上与其他被认为是函数式语言类似的特性。

更具体的，将要涉及：

* **闭包（Closures）**，一个可以储存在变量里的类似函数的结构
* **迭代器（Iterators）**，一种处理元素序列的方式

#### 闭包：可以捕获环境的匿名函数

Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算。不同于函数，闭包允许捕获调用者作用域中的值。将展示闭包的这些功能如何复用代码和自定义行为。

#### 使用闭包创建行为的抽象

让来看一个存储稍后要执行的闭包的示例。其间会讨论闭包的语法、类型推断和 trait。

考虑一下这个假想的情况：在一个通过 app 生成自定义健身计划的初创企业工作。其后端使用 Rust 编写，而生成健身计划的算法需要考虑很多不同的因素，比如用户的年龄、身体质量指数（Body Mass Index）、用户喜好、最近的健身活动和用户指定的强度系数。本例中实际的算法并不重要，重要的是这个计算只花费几秒钟。只希望在需要时调用算法，并且只希望调用一次，这样就不会让用户等得太久。

```rust

#![allow(unused_variables)]
fn main() {
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
}
```

接下来，main 函数中将会包含本例的健身 app 中的重要部分。这代表当用户请求健身计划时 app 会调用的代码。因为与 app 前端的交互与闭包的使用并不相关，所以将硬编码代表程序输入的值并打印输出。

所需的输入有这些：

* 一个来自用户的 intensity 数字，请求健身计划时指定，它代表用户喜好低强度还是高强度健身。
* 一个随机数，其会在健身计划中生成变化。

程序的输出将会是建议的锻炼计划。示例 13-2 展示了将要使用的 main 函数：

```rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
fn generate_workout(intensity: u32, random_number: u32) {}

```

出于简单考虑这里硬编码了 simulated_user_specified_value 变量的值为 10 和 simulated_random_number 变量的值为 7；一个实际的程序会从 app 前端获取强度系数并使用 rand crate 来生成随机数，正如第二章的猜猜看游戏所做的那样。main 函数使用模拟的输入值调用 generate_workout 函数：

现在有了执行上下文，让编写算法。示例 13-3 中的 generate_workout 函数包含本例中最关心的 app 业务逻辑。本例中余下的代码修改都将在这个函数中进行：

```rust

#![allow(unused_variables)]
fn main() {
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(num: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
}
```

现在这份代码能够应对的需求了，但数据科学部门的同学告知将来会对调用 simulated_expensive_calculation 的方式做出一些改变。为了在要做这些改动的时候简化更新步骤，将重构代码来让它只调用 simulated_expensive_calculation 一次。同时还希望去掉目前多余的连续两次函数调用，并不希望在计算过程中增加任何其他此函数的调用。也就是说，不希望在完全无需其结果的情况调用函数，不过仍然希望只调用函数一次。

#### 使用函数重构

```rust

#![allow(unused_variables)]
fn main() {
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(num: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
}
```

#### 重构使用闭包储存代码

不同于总是在 if 块之前调用 simulated_expensive_calculation 函数并储存其结果，可以定义一个闭包并将其储存在变量中，如示例 13-5 所示。实际上可以选择将整个 simulated_expensive_calculation 函数体移动到这里引入的闭包中：

```rust

#![allow(unused_variables)]
fn main() {
use std::thread;
use std::time::Duration;

let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
expensive_closure(5);
}
```

闭包定义是 expensive_closure 赋值的 = 之后的部分。闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；之所以选择这个语法是因为它与 Smalltalk 和 Ruby 的闭包定义类似。这个闭包有一个参数 num；如果有多于一个参数，可以使用逗号分隔，比如 |param1, param2|。

参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号是可以省略的。大括号之后闭包的结尾，需要用于 let 语句的分号。因为闭包体的最后一行没有分号（正如函数体一样），所以闭包体（num）最后一行的返回值作为调用闭包时的返回值 。

注意这个 let 语句意味着 expensive_closure 包含一个匿名函数的 定义，不是调用匿名函数的 返回值。回忆一下使用闭包的原因是需要在一个位置定义代码，储存代码，并在之后的位置实际调用它；期望调用的代码现在储存在 expensive_closure 中。

定义了闭包之后，可以改变 if 块中的代码来调用闭包以执行代码并获取结果值。调用闭包类似于调用函数；指定存放闭包定义的变量名并后跟包含期望使用的参数的括号，

```rust

#![allow(unused_variables)]
fn main() {
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
}
```

仍然在第一个 if 块中调用了闭包两次，这调用了慢计算代码两次而使得用户需要多等待一倍的时间。可以通过在 if 块中创建一个本地变量存放闭包调用的结果来解决这个问题，不过闭包可以提供另外一种解决方案。稍后会讨论这个方案，不过目前让首先讨论一下为何闭包定义中和所涉及的 trait 中没有类型注解。

#### 闭包类型推断和注解

闭包不要求像 fn 函数那样在参数和返回值上注明类型。函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分。严格的定义这些接口对于保证所有人都认同函数使用和返回值的类型来说是很重要的。但是闭包并不用于这样暴露在外的接口：他们储存在变量中并被使用，不用命名他们或暴露给库的用户调用。

闭包通常很短并只与对应相对任意的场景较小的上下文中。在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型，类似于它是如何能够推断大部分变量的类型一样。

强制在这些小的匿名函数中注明类型是很恼人的，并且与编译器已知的信息存在大量的重复。

类似于变量，如果相比严格的必要性更希望增加明确性并变得更啰嗦，可以选择增加类型注解；

```rust

#![allow(unused_variables)]
fn main() {
use std::thread;
use std::time::Duration;

let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
}
```

有了类型注解闭包的语法就更类似函数了。如下是一个对其参数加一的函数的定义与拥有相同行为闭包语法的纵向对比。这里增加了一些空格来对齐相应部分。这展示了闭包语法如何类似于函数语法，除了使用竖线而不是括号以及几个可选的语法之外：

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;

```

第一行展示了一个函数定义，而第二行展示了一个完整标注的闭包定义。第三行闭包定义中省略了类型注解，而第四行去掉了可选的大括号，因为闭包体只有一行。这些都是有效的闭包定义，并在调用时产生相同的行为。

闭包定义会为每个参数和返回值推断一个具体类型。例如，下例中展示了仅仅将参数作为返回值的简短的闭包定义。除了作为示例的目的这个闭包并不是很实用。注意其定义并没有增加任何类型注解：如果尝试调用闭包两次，第一次使用 String 类型作为参数而第二次使用 u32，则会得到一个错误：

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);

```

#### 使用带有泛型和 Fn trait 的闭包

仍然调用了多于需要的慢计算闭包。解决这个问题的一个方法是在全部代码中的每一个需要多个慢计算闭包结果的地方，可以将结果保存进变量以供复用，这样就可以使用变量而不是再次调用闭包。但是这样就会有很多重复的保存结果变量的地方。

幸运的是，还有另一个可用的方案。可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。可能见过这种模式被称 memoization 或 lazy evaluation。

为了让结构体存放闭包，需要指定闭包的类型，因为结构体定义需要知道其每一个字段的类型。每一个闭包实例有其自己独有的匿名类型：也就是说，即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。为了定义使用闭包的结构体、枚举或函数参数，需要像第十章讨论的那样使用泛型和 trait bound。

Fn 系列 trait 由标准库提供。所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个。在 “闭包会捕获其环境” 部分会讨论这些 trait 的区别；在这个例子中可以使用 Fn trait。

为了满足 Fn trait bound 增加了代表闭包所必须的参数和返回值类型的类型。在这个例子中，闭包有一个 u32 的参数并返回一个 u32，这样所指定的 trait bound 就是 Fn(u32) -> u32。

```rust

#![allow(unused_variables)]
fn main() {
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
}
```

结构体 Cacher 有一个泛型 T 的字段 calculation。T 的 trait bound 指定了 T 是一个使用 Fn 的闭包。任何希望储存到 Cacher 实例的 calculation 字段的闭包必须有一个 u32 参数（由 Fn 之后的括号的内容指定）并必须返回一个 u32（由 -> 之后的内容）。

字段 value 是 `Option<u32>` 类型的。在执行闭包之前，value 将是 None。如果使用 Cacher 的代码请求闭包的结果，这时会执行闭包并将结果储存在 value 字段的 Some 成员中。接着如果代码再次请求闭包的结果，这时不再执行闭包，而是会返回存放在 Some 成员中的结果。

```rust

#![allow(unused_variables)]
fn main() {
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
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
            },
        }
    }
}
}
```

Cacher 结构体的字段是私有的，因为希望 Cacher 管理这些值而不是任由调用代码潜在的直接改变他们。

Cacher::new 函数获取一个泛型参数 T，它定义于 impl 块上下文中并与 Cacher 结构体有着相同的 trait bound。Cacher::new 返回一个在 calculation 字段中存放了指定闭包和在 value 字段中存放了 None 值的 Cacher 实例，因为还未执行闭包。

当调用代码需要闭包的执行结果时，不同于直接调用闭包，它会调用 value 方法。这个方法会检查 self.value 是否已经有了一个 Some 的结果值；如果有，它返回 Some 中的值并不会再次执行闭包。

如果 self.value 是 None，则会调用 self.calculation 中储存的闭包，将结果保存到 self.value 以便将来使用，并同时返回结果值。

```rust

#![allow(unused_variables)]
fn main() {
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
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
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
}
```

#### Cacher 实现的限制

值缓存是一种更加广泛的实用行为，可能希望在代码中的其他闭包中也使用他们。然而，目前 Cacher 的实现存在两个小问题，这使得在不同上下文中复用变得很困难。

```rust
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

```

这个测试使用返回传递给它的值的闭包创建了一个新的 Cacher 实例。使用为 1 的 arg 和为 2 的 arg 调用 Cacher 实例的 value 方法，同时期望使用为 2 的 arg 调用 value 会返回 2。

这里的问题是第一次使用 1 调用 c.value，Cacher 实例将 Some(1) 保存进 self.value。在这之后，无论传递什么值调用 value，它总是会返回 1。

尝试修改 Cacher 存放一个哈希 map 而不是单独一个值。哈希 map 的 key 将是传递进来的 arg 值，而 value 则是对应 key 调用闭包的结果值。相比之前检查 self.value 直接是 Some 还是 None 值，现在 value 函数会在哈希 map 中寻找 arg，如果找到的话就返回其对应的值。如果不存在，Cacher 会调用闭包并将结果值保存在哈希 map 对应 arg 值的位置。

当前 Cacher 实现的第二个问题是它的应用被限制为只接受获取一个 u32 值并返回一个 u32 值的闭包。比如说，可能需要能够缓存一个获取字符串 slice 并返回 usize 值的闭包的结果。请尝试引入更多泛型参数来增加 Cacher 功能的灵活性。

#### 闭包会捕获其环境

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

```

```rust
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}

```

编译器甚至会提示这只能用于闭包！

当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。这会使用内存并产生额外的开销，在更一般的场景中，当不需要闭包来捕获环境时，不希望产生这些开销。因为函数从未允许捕获环境，定义和使用函数也就从不会有这些额外开销。

闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 Fn trait：

* FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
* FnMut 获取可变的借用值所以可以改变其环境
* Fn 从其环境获取不可变的借用值

当创建一个闭包时，Rust 根据其如何使用环境中变量来推断希望如何引用环境。由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce 。那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut ，而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn 。

```rust
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

```

这个例子并不能编译，会产生以下错误：

```rust
error[E0382]: use of moved value: `x`
 --> src/main.rs:6:40
  |
4 |     let equal_to_x = move |z| z == x;
  |                      -------- value moved (into closure) here
5 |
6 |     println!("can't use x here: {:?}", x);
  |                                        ^ value used here after move
  |
  = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not
  implement the `Copy` trait

```

x 被移动进了闭包，因为闭包使用 move 关键字定义。接着闭包获取了 x 的所有权，同时 main 就不再允许在 println! 语句中使用 x 了。去掉 println! 即可修复问题。

大部分需要指定一个 Fn 系列 trait bound 的时候，可以从 Fn 开始，而编译器会根据闭包体中的情况告诉是否需要 FnMut 或 FnOnce。

为了展示闭包作为函数参数时捕获其环境的作用，让继续下一个主题：迭代器。

#### 使用迭代器处理元素序列

迭代器模式允许对一个项的序列进行某些处理。迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，无需重新实现这些逻辑。

在 Rust 中，迭代器是 惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果。

```rust

#![allow(unused_variables)]
fn main() {
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
}
```

```rust

#![allow(unused_variables)]
fn main() {
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
}
```

在标准库中没有提供迭代器的语言中，可能会使用一个从 0 开始的索引变量，使用这个变量索引 vector 中的值，并循环增加其值直到达到 vector 的元素数量。

迭代器为处理了所有这些逻辑，这减少了重复代码并消除了潜在的混乱。另外，迭代器的实现方式提供了对多种不同的序列使用相同逻辑的灵活性，而不仅仅是像 vector 这样可索引的数据结构.让看看迭代器是如何做到这些的。

#### Iterator trait 和 next 方法

迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait。这个 trait 的定义看起来像这样：

```rust

#![allow(unused_variables)]
fn main() {
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了方法的默认实现
}
}
```

注意这里有一下还未讲到的新语法：type Item 和 Self::Item，他们定义了 trait 的 关联类型（associated type）。第十九章会深入讲解关联类型，不过现在只需知道这段代码表明实现 Iterator trait 要求同时定义一个 Item 类型，这个 Item 类型被用作 next 方法的返回值类型。换句话说，Item 类型将是迭代器返回元素的类型。

next 是 Iterator 实现者被要求定义的唯一方法。next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None。

可以直接调用迭代器的 next 方法；下例  有一个测试展示了重复调用由 vector 创建的迭代器的 next 方法所得到的值：

```rust

#![allow(unused_variables)]
fn main() {
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
}
```

注意 v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。换句话说，代码 消费（consume）了，或使用了迭代器。每一个 next 调用都会从迭代器中消费一个项。使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。

另外需要注意到从 next 调用中得到的值是 vector 的不可变引用。iter 方法生成一个不可变引用的迭代器。如果需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。类似的，如果希望迭代可变引用，则可以调用 iter_mut 而不是 iter。

#### 消费迭代器的方法

Iterator trait 有一系列不同的由标准库提供默认实现的方法；可以在 Iterator trait 的标准库 API 文档中找到所有这些方法。一些方法在其定义中调用了 next 方法，这也就是为什么在实现 Iterator trait 时要求实现 next 方法的原因。

这些调用 next 方法的方法被称为 消费适配器（consuming adaptors），因为调用他们会消耗迭代器。一个消费适配器的例子是 sum 方法。这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和。

```rust

#![allow(unused_variables)]
fn main() {
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
}
```

调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。

#### 产生其他迭代器的方法

Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），他们允许将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。

下例 展示了一个调用迭代器适配器方法 map 的例子，该 map 方法使用闭包来调用每个元素以生成新的迭代器。 这里的闭包创建了一个新的迭代器，对其中 vector 中的每个元素都被加 1。不过这些代码会产生一个警告：

```rust

#![allow(unused_variables)]
fn main() {
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
}
```

得到的警告是：

```rust
warning: unused `std::iter::Map` which must be used: iterator adaptors are lazy
and do nothing unless consumed
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default

```

在 下例 中，将遍历由 map 调用生成的迭代器的结果收集到一个 vector 中，它将会含有原始 vector 中每个元素加 1 的结果：

```rust

#![allow(unused_variables)]
fn main() {
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
}
```

因为 map 获取一个闭包，可以指定任何希望在遍历的每个元素上执行的操作。这是一个展示如何使用闭包来自定义行为同时又复用 Iterator trait 提供的迭代行为的绝佳例子。

#### 使用闭包获取环境

现在介绍了迭代器，让展示一个通过使用 filter 迭代器适配器和捕获环境的闭包的常规用例。迭代器的 filter 方法获取一个使用迭代器的每一个项并返回布尔值的闭包。如果闭包返回 true，其值将会包含在 filter 提供的新迭代器中。如果闭包返回 false，其值不会包含在结果迭代器中。

```rust

#![allow(unused_variables)]
fn main() {
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
}
```

shoes_in_my_size 函数获取一个鞋子 vector 的所有权和一个鞋子大小作为参数。它返回一个只包含指定大小鞋子的 vector。

shoes_in_my_size 函数体中调用了 into_iter 来创建一个获取 vector 所有权的迭代器。接着调用 filter 将这个迭代器适配成一个只含有那些闭包返回 true 的元素的新迭代器。

闭包从环境中捕获了 shoe_size 变量并使用其值与每一只鞋的大小作比较，只保留指定大小的鞋子。最终，调用 collect 将迭代器适配器返回的值收集进一个 vector 并返回。

#### 实现 Iterator trait 来创建自定义迭代器

已经展示了可以通过在 vector 上调用 iter、into_iter 或 iter_mut 来创建一个迭代器。也可以用标准库中其他的集合类型创建迭代器，比如哈希 map。另外，可以实现 Iterator trait 来创建任何希望的迭代器。正如之前提到的，定义中唯一要求提供的方法就是 next 方法。一旦定义了它，就可以使用所有其他由 Iterator trait 提供的拥有默认实现的方法来创建自定义迭代器了！

作为展示，让创建一个只会从 1 数到 5 的迭代器。首先，创建一个结构体来存放一些值，接着实现 Iterator trait 将这个结构体放入迭代器中并在此实现中使用其值。

```rust

#![allow(unused_variables)]
fn main() {
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
}
```

Counter 结构体有一个字段 count。这个字段存放一个 u32 值，它会记录处理 1 到 5 的迭代过程中的位置。count 是私有的因为希望 Counter 的实现来管理这个值。new 函数通过总是从为 0 的 count 字段开始新实例来确保需要的行为。

接下来将为 Counter 类型实现 Iterator trait，通过定义 next 方法来指定使用迭代器时的行为，如 下例 所示：

```rust

#![allow(unused_variables)]
fn main() {
struct Counter {
    count: u32,
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
}
```

这里将迭代器的关联类型 Item 设置为 u32，意味着迭代器会返回 u32 值集合。再一次，这里仍无需担心关联类型，第十九章会讲到。

希望迭代器对其内部状态加一，这也就是为何将 count 初始化为 0：希望迭代器首先返回 1。如果 count 值小于 6，next 会返回封装在 Some 中的当前值，不过如果 count 大于或等于 6，迭代器会返回 None。

#### 使用 Counter 迭代器的 next 方法

一旦实现了 Iterator trait，就有了一个迭代器！下例 展示了一个测试用来演示使用 Counter 结构体的迭代器功能，通过直接调用 next 方法，

```rust

#![allow(unused_variables)]
fn main() {
struct Counter {
    count: u32,
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

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
}
```

这个测试在 counter 变量中新建了一个 Counter 实例并接着反复调用 next 方法，来验证实现的行为符合这个迭代器返回从 1 到 5 的值的预期。

#### 使用自定义迭代器中其他 Iterator trait 方法

通过定义 next 方法实现 Iterator trait，现在就可以使用任何标准库定义的拥有默认实现的 Iterator trait 方法了，因为他们都使用了 next 方法的功能。

```rust

#![allow(unused_variables)]
fn main() {
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // 迭代器会产生 u32s
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // count 自增 1。也就是为什么从 0 开始。
        self.count += 1;

        // 检测是否结束结束计数。
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
}
```

注意 zip 只产生四对值；理论上第五对值 (5, None) 从未被产生，因为 zip 在任一输入迭代器返回 None 时也返回 None。

所有这些方法调用都是可能的，因为指定了 next 方法如何工作，而标准库则提供了其它调用 next 的方法的默认实现。

#### 改进 I/O 项目

增加了一些代码获取一个 String slice 并创建一个 Config 结构体的实例，他们索引 slice 中的值并克隆这些值以便 Config 结构体可以拥有这些值。

```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

```

起初这里需要 clone 的原因是参数 args 中有一个 String 元素的 slice，而 new 函数并不拥有 args。为了能够返回 Config 实例的所有权，需要克隆 Config 中字段 query 和 filename 的值，这样 Config 实例就能拥有这些值。

在学习了迭代器之后，可以将 new 函数改为获取一个有所有权的迭代器作为参数而不是借用 slice。将使用迭代器功能之前检查 slice 长度和索引特定位置的代码。这会明确 Config::new 的工作因为迭代器会负责访问这些值。

一旦 Config::new 获取了迭代器的所有权并不再使用借用的索引操作，就可以将迭代器中的 String 值移动到 Config 中，而不是调用 clone 分配新的空间。

#### 直接使用 env::args 返回的迭代器

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}

```

```rust
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}

```

env::args 函数返回一个迭代器！不同于将迭代器的值收集到一个 vector 中接着传递一个 slice 给 Config::new，现在直接将 env::args 返回的迭代器的所有权传递给 Config::new。

接下来需要更新 Config::new 的定义。在 I/O 项目的 src/lib.rs 中，将 Config::new 的签名改为如 下例 所示。这仍然不能编译因为还需更新函数体：

```rust
fn main() {}
use std::env;

struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

```

请记住 env::args 返回值的第一个值是程序的名称。希望忽略它并获取下一个值，所以首先调用 next 并不对返回值做任何操作。之后对希望放入 Config 中字段 query 调用 next。如果 next 返回 Some，使用 match 来提取其值。如果它返回 None，则意味着没有提供足够的参数并通过 Err 值提早返回。对 filename 值进行同样的操作。

#### 使用迭代器适配器来使代码更简明

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

```

可以通过使用迭代器适配器方法来编写更简明的代码。这也避免了一个可变的中间 results vector 的使用。函数式编程风格倾向于最小化可变状态的数量来使代码更简洁。去掉可变状态可能会使得将来进行并行搜索的增强变得更容易，因为不必管理 results vector 的并发访问。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

```

#### 性能对比：循环 VS 迭代器

为了决定使用哪个实现，需要知道哪个版本的 search 函数更快一些：是直接使用 for 循环的版本还是使用迭代器的版本。

运行了一个性能测试，通过将阿瑟·柯南·道尔的“福尔摩斯探案集”的全部内容加载进 String 并寻找其中的单词 “the”。如下是 for 循环版本和迭代器版本的 search 函数的性能测试结果：

结果迭代器版本还要稍微快一点！这里将不会查看性能测试的代码，的目的并不是为了证明他们是完全等同的，而是得出一个怎样比较这两种实现方式性能的基本思路。

对于一个更全面的性能测试，将会检查不同长度的文本、不同的搜索单词、不同长度的单词和所有其他的可变情况。这里所要表达的是：迭代器，作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能代码。迭代器是 Rust 的 零成本抽象（zero-cost abstractions）之一，它意味着抽象并不会引入运行时开销，它与本贾尼·斯特劳斯特卢普（C++ 的设计和实现者）在 “Foundations of C++”（2012） 中所定义的 零开销（zero-overhead）如出一辙：

作为另一个例子，这里有一些取自于音频解码器的代码。解码算法使用线性预测数学运算（linear prediction mathematical operation）来根据之前样本的线性函数预测将来的值。这些代码使用迭代器链来对作用域中的三个变量进行了某种数学计算：一个叫 buffer 的数据 slice、一个有 12 个元素的数组 coefficients、和一个代表位移位数的 qlp_shift。例子中声明了这些变量但并没有提供任何值；虽然这些代码在其上下文之外没有什么意义，不过仍是一个简明的现实中的例子，来展示 Rust 如何将高级概念转换为底层代码：

```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}

```

为了计算 prediction 的值，这些代码遍历了 coefficients 中的 12 个值，使用 zip 方法将系数与 buffer 的前 12 个值组合在一起。接着将每一对值相乘，再将所有结果相加，然后将总和右移 qlp_shift 位。

像音频解码器这样的程序通常最看重计算的性能。这里，创建了一个迭代器，使用了两个适配器，接着消费了其值。Rust 代码将会被编译为什么样的汇编代码呢？好吧，在编写本书的这个时候，它被编译成与手写的相同的汇编代码。遍历 coefficients 的值完全用不到循环：Rust 知道这里会迭代 12 次，所以它“展开”（unroll）了循环。展开是一种移除循环控制代码的开销并替换为每个迭代中的重复代码的优化。

所有的系数都被储存在了寄存器中，这意味着访问他们非常快。这里也没有运行时数组访问边界检查。所有这些 Rust 能够提供的优化使得结果代码极为高效。现在知道这些了，请放心大胆的使用迭代器和闭包吧！他们使得代码看起来更高级，但并不为此引入运行时性能损失。

### 只能指针

指针 （pointer）是一个包含内存地址的变量的通用概念。这个地址引用，或 “指向”（points at）一些其他数据。Rust 中最常见的指针是第四章介绍的 引用（reference）。引用以 & 符号为标志并借用了他们所指向的值。除了引用数据没有任何其他特殊功能。它们也没有任何额外开销，所以应用的最多。

另一方面，智能指针（smart pointers）是一类数据结构，他们的表现类似指针，但是也拥有额外的元数据和功能。智能指针的概念并不为 Rust 所独有；其起源于 C++ 并存在于其他语言中。Rust 标准库中不同的智能指针提供了多于引用的额外功能。本章将会探索的一个例子便是 引用计数 （reference counting）智能指针类型，其允许数据有多个所有者。引用计数智能指针记录总共有多少个所有者，并当没有任何所有者时负责清理数据。

在 Rust 中，普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针；相反，在大部分情况下，智能指针 拥有 他们指向的数据。

实际上本书中已经出现过一些智能指针，比如第八章的 String 和 Vec<T>，虽然当时并不这么称呼它们。这些类型都属于智能指针因为它们拥有一些数据并允许修改它们。它们也带有元数据（比如他们的容量）和额外的功能或保证（String 的数据总是有效的 UTF-8 编码）。

智能指针通常使用结构体实现。智能指针区别于常规结构体的显著特性在于其实现了 Deref 和 Drop trait。Deref trait 允许智能指针结构体实例表现的像引用一样，这样就可以编写既用于引用、又用于智能指针的代码。Drop trait 允许自定义当智能指针离开作用域时运行的代码。本章会讨论这些 trait 以及为什么对于智能指针来说他们很重要。

考虑到智能指针是一个在 Rust 经常被使用的通用设计模式，本章并不会覆盖所有现存的智能指针。很多库都有自己的智能指针而也可以编写属于自己的智能指针。这里将会讲到的是来自标准库中最常用的一些：

* `Box<T>`，用于在堆上分配值
* `Rc<T>`，一个引用计数类型，其数据可以有多个所有者
* `Ref<T>` 和 `RefMut<T>`，通过 `RefCell<T>` 访问，一个在运行时而不是在编译时执行借用规则的类型。

#### 使用Box <T>指向堆上的数据

最简单直接的智能指针是 box，其类型是 `Box<T>`。 box 允许将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。如果想回顾一下栈与堆的区别请参考第四章。

除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：

* 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
* 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
* 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

下例展示了如何使用 box 在堆上储存一个 i32：

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}

```

这里定义了变量 b，其值是一个指向被分配在堆上的值 5 的 Box。这个程序会打印出 b = 5；在这个例子中，可以像数据是储存在栈上的那样访问 box 中的数据。正如任何拥有数据所有权的值那样，当像 b 这样的 box 在 main 的末尾离开作用域时，它将被释放。这个释放过程作用于 box 本身（位于栈上）和它所指向的数据（位于堆上）。

将一个单独的值存放在堆上并不是很有意义，所以像示例 15-1 这样单独使用 box 并不常见。将像单个 i32 这样的值储存在栈上，也就是其默认存放的地方在大部分使用场景中更为合适。让看看一个不使用 box 时无法定义的类型的例子。

#### Box 允许创建递归类型

Rust 需要在编译时知道类型占用多少空间。一种无法在编译时知道大小的类型是 递归类型（recursive type），其值的一部分可以是相同类型的另一个值。这种值的嵌套理论上可以无限的进行下去，所以 Rust 不知道递归类型需要多少空间。不过 box 有一个已知的大小，所以通过在循环类型定义中插入 box，就可以创建递归类型了。

##### cons list 的更多内容

cons list 是一个来源于 Lisp 编程语言及其方言的数据结构。在 Lisp 中，cons 函数（“construct function" 的缩写）利用两个参数来构造一个新的列表，他们通常是一个单独的值和另一个列表。

cons 函数的概念涉及到更常见的函数式编程术语；“将 x 与 y 连接” 通常意味着构建一个新的容器而将 x 的元素放在新容器的开头，其后则是容器 y 的元素。

cons list 的每一项都包含两个元素：当前项的值和下一项。其最后一项值包含一个叫做 Nil 的值且没有下一项。cons list 通过递归调用 cons 函数产生。代表递归的终止条件（base case）的规范名称是 Nil，它宣布列表的终止。注意这不同于第六章中的 “null” 或 “nil” 的概念，他们代表无效或缺失的值。 

注意虽然函数式编程语言经常使用 cons list，但是它并不是一个 Rust 中常见的类型。大部分在 Rust 中需要列表的时候，`Vec<T>` 是一个更好的选择。其他更为复杂的递归数据类型 确实 在 Rust 的很多场景中很有用，不过通过以 cons list 作为开始，可以探索如何使用 box 毫不费力的定义一个递归数据类型。

下例 包含一个 cons list 的枚举定义。注意这还不能编译因为这个类型没有已知的大小：

```rust
enum List {
    Cons(i32, List),
    Nil,
}

```

使用这个 cons list 来储存列表 1, 2, 3 将看起来如下例所示：

```rust
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}

```

第一个 Cons 储存了 1 和另一个 List 值。这个 List 是另一个包含 2 的 Cons 值和下一个 List 值。接着又有另一个存放了 3 的 Cons 值和最后一个值为 Nil 的 List，非递归成员代表了列表的结尾。

如果尝试编译下例的代码，会得到如下所示的错误：

```rust
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |               ----- recursive without indirection
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable

```

这个错误表明这个类型 “有无限的大小”。其原因是 List 的一个成员被定义为是递归的：它直接存放了另一个相同类型的值。这意味着 Rust 无法计算为了存放 List 值到底需要多少空间。让一点一点来看：首先了解一下 Rust 如何决定需要多少空间来存放一个非递归类型。

#### 计算非递归类型的大小

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

当 Rust 需要知道要为 Message 值分配多少空间时，它可以检查每一个成员并发现 Message::Quit 并不需要任何空间，Message::Move 需要足够储存两个 i32 值的空间，依此类推。因此，Message 值所需的空间等于储存其最大成员的空间大小。

与此相对当 Rust 编译器检查像示例 15-2 中的 List 这样的递归类型时会发生什么呢。编译器尝试计算出储存一个 List 枚举需要多少内存，并开始检查 Cons 成员，那么 Cons 需要的空间等于 i32 的大小加上 List 的大小。为了计算 List 需要多少内存，它检查其成员，从 Cons 成员开始。Cons成员储存了一个 i32 值和一个List值，这样的计算将无限进行下去.

#### 使用 Box<T> 给递归类型一个已知的大小

Rust 无法计算出要为定义为递归的类型分配多少空间，所以编译器给出了下例中的错误。这个错误也包括了有用的建议：

```rust
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable

```

因为 `Box<T>` 是一个指针，总是知道它需要多少空间：指针的大小并不会根据其指向的数据量而改变。这意味着可以将 Box 放入 Cons 成员中而不是直接存放另一个 List 值。Box 会指向另一个位于堆上的 List 值，而不是存放在 Cons 成员中。从概念上讲，仍然有一个通过在其中 “存放” 其他列表创建的列表，不过现在实现这个概念的方式更像是一个项挨着另一项，而不是一项包含另一项。

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}

```

Cons 成员将会需要一个 i32 的大小加上储存 box 指针数据的空间。Nil 成员不储存值，所以它比 Cons 成员需要更少的空间。现在知道了任何 List 值最多需要一个 i32 加上 box 指针数据的大小。通过使用 box ，打破了这无限递归的连锁，这样编译器就能够计算出储存 List 值需要的大小了。

box 只提供了间接存储和堆分配；他们并没有任何其他特殊的功能，比如将会见到的其他智能指针。它们也没有这些特殊功能带来的性能损失，所以他们可以用于像 cons list 这样间接存储是唯一所需功能的场景。

`Box<T>` 类型是一个智能指针，因为它实现了 Deref trait，它允许 `Box<T>` 值被当作引用对待。当 `Box<T>` 值离开作用域时，由于 `Box<T>` 类型 Drop trait 的实现，box 所指向的堆数据也会被清除。让更详细的探索一下这两个 trait。这两个 trait 对于在本章余下讨论的其他智能指针所提供的功能中，将会更为重要。

#### 通过 Deref trait 将智能指针当作常规引用处理

实现 Deref trait 允许重载 解引用运算符（dereference operator）*（与乘法运算符或 glob 运算符相区别）。通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。

让首先看看解引用运算符如何处理常规引用，接着尝试定义自己的类似 `Box<T>` 的类型并看看为何解引用运算符不能像引用一样工作。会探索如何实现 Deref trait 使得智能指针以类似引用的方式工作变为可能。最后，会讨论 Rust 的 解引用强制多态（deref coercions）功能和它是如何一同处理引用或智能指针的。

#### 通过解引用运算符追踪指针的值

常规引用是一个指针类型，一种理解指针的方式是将其看成指向储存在其他某处值的箭头。

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

```

变量 x 存放了一个 i32 值 5。y 等于 x 的一个引用。可以断言 x 等于 5。然而，如果希望对 y 的值做出断言，必须使用 *y 来追踪引用所指向的值（也就是 解引用）。一旦解引用了 y，就可以访问 y 所指向的整型值并可以与 5 做比较。

相反如果尝试编写 assert_eq!(5, y);，则会得到如下编译错误：

```rust
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `std::cmp::PartialEq<&{integer}>` is not implemented for
  `{integer}`

```

不允许比较数字的引用与数字，因为它们是不同的类型。必须使用解引用运算符追踪引用所指向的值。

#### 像引用一样使用 Box<T>

可以使用 `Box<T>` 代替引用来重写上例中的代码，解引用运算符也一样能工作，如下例所示：

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

```

#### 自定义智能指针

为了体会默认智能指针的行为不同于引用，让创建一个类似于标准库提供的 `Box<T>` 类型的智能指针。接着会学习如何增加使用解引用运算符的功能。

从根本上说，`Box<T>` 被定义为包含一个元素的元组结构体，所以下例以相同的方式定义了 `MyBox<T>` 类型。还定义了 new 函数来对应定义于 `Box<T>` 的 new 函数：

```rust

#![allow(unused_variables)]
fn main() {
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
}
```

这里定义了一个结构体 MyBox 并声明了一个泛型参数 T，因为希望其可以存放任何类型的值。MyBox 是一个包含 T 类型元素的元组结构体。MyBox::new 函数获取一个 T 类型的参数并返回一个存放传入值的 MyBox 实例。

尝试将下例中的代码加入示例 15-8 中并修改 main 使用定义的 `MyBox<T>` 类型代替 `Box<T>`。示例 15-9 中的代码不能编译，因为 Rust 不知道如何解引用 MyBox：

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

```

尝试以使用引用和 `Box<T>` 相同的方式使用 `MyBox<T>`

得到的编译错误是：

```rust
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^

```

`MyBox<T>` 类型不能解引用并没有为其实现这个功能。为了启用 * 运算符的解引用功能，需要实现 Deref trait。

#### 通过实现 Deref trait 将某类型像引用一样处理

为了实现 trait，需要提供 trait 所需的方法实现。Deref trait，由标准库提供，要求实现名为 deref 的方法，其借用 self 并返回一个内部数据的引用。下例包含定义于 MyBox 之上的 Deref 实现：

```rust

#![allow(unused_variables)]
fn main() {
use std::ops::Deref;

struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
}
```

type Target = T; 语法定义了用于此 trait 的关联类型。关联类型是一个稍有不同的定义泛型参数的方式，现在还无需过多的担心它；第十九章会详细介绍。

deref 方法体中写入了 &self.0，这样 deref 返回了我希望通过 * 运算符访问的值的引用。示例 15-9 中的 main 函数中对 `MyBox<T>` 值的 * 调用现在可以编译并能通过断言了！

没有 Deref trait 的话，编译器只会解引用 & 引用类型。deref 方法向编译器提供了获取任何实现了 Deref trait 的类型的值并调用这个类型的 deref 方法来获取一个它知道如何解引用的 & 引用的能力。

```rust
*(y.deref())

```

Rust 将 * 运算符替换为先调用 deref 方法再进行直接引用的操作，如此便不用担心是不是还需要手动调用 deref 方法了。Rust 的这个特性可以让写出行为一致的代码，无论是面对的是常规引用还是实现了 Deref 的类型。

deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有权。如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。在这里以及大部分使用解引用运算符的情况下并不希望获取 `MyBox<T>` 内部值的所有权。

注意，每次当在代码中使用 * 时， * 运算符都被替换成了先调用 deref 方法再接着使用 * 解引用的操作，且只会发生一次，不会对 * 操作符无限递归替换，解引用出上面 i32 类型的值就停止了，这个值与 下例 中 assert_eq! 的 5 相匹配。

#### 函数和方法的隐式解引用强制多态

解引用强制多态（deref coercions）是 Rust 表现在函数或方法传参上的一种便利。其将实现了 Deref 的类型的引用转换为原始类型通过 Deref 所能够转换的类型的引用。当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，解引用强制多态将自动发生。这时会有一系列的 deref 方法被调用，把提供的类型转换成了参数所需的类型。

解引用强制多态的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用。这个功能也使得可以编写更多同时作用于引用或智能指针的代码。

作为展示解引用强制多态的实例，让使用上例中定义的 `MyBox<T>`，以及下例中增加的 Deref 实现。下例展示了一个有着字符串 slice 参数的函数定义：

```rust

#![allow(unused_variables)]
fn main() {
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
}
```

可以使用字符串 slice 作为参数调用 hello 函数，比如 hello("Rust");。解引用强制多态使得用 `MyBox<String>` 类型值的引用调用 hello 成为可能，如下例 所示：

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

```

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

```

(*m) 将 `MyBox<String>` 解引用为 String。接着 `&` 和 `[..]` 获取了整个 String 的字符串 slice 来匹配 hello 的签名。没有解引用强制多态所有这些符号混在一起将更难以读写和理解。解引用强制多态使得 Rust 自动的帮处理这些转换。

当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型。这些解析都发生在编译时，所以利用解引用强制多态并没有运行时惩罚！

#### 解引用强制多态如何与可变性交互

类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。

Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：

* 当 `T: Deref<Target=U>` 时从 &T 到 &U。
* 当 `T: DerefMut<Target=U>` 时从 &mut T 到 &mut U。
* 当 `T: Deref<Target=U>` 时从 &mut T 到 &U。

头两个情况除了可变性之外是相同的：第一种情况表明如果有一个 &T，而 T 实现了返回 U 类型的 Deref，则可以直接得到 &U。第二种情况表明对于可变引用也有着相同的行为。

第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。但是反之是 不可能 的：不可变引用永远也不能强转为可变引用。因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。将一个可变引用转换为不可变引用永远也不会打破借用规则。将不可变引用转换为可变引用则需要数据只能有一个不可变引用，而借用规则无法保证这一点。因此，Rust 无法假设将不可变引用转换为可变引用是可能的。

#### 使用 Drop Trait 运行清理代码

对于智能指针模式来说第二个重要的 trait 是 Drop，其允许在值要离开作用域时执行一些代码。可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源。在智能指针上下文中讨论 Drop 是因为其功能几乎总是用于实现智能指针。例如，Box<T> 自定义了 Drop 用来释放 box 所指向的堆空间。

在其他一些语言中，不得不记住在每次使用完智能指针实例后调用清理内存或资源的代码。如果忘记的话，运行代码的系统可能会因为负荷过重而崩溃。在 Rust 中，可以指定一些代码应该在值离开作用域时被执行，而编译器会自动插入这些代码。于是就不需要在程序中到处编写在实例结束时清理这些变量的代码 —— 而且还不会泄露资源。

指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。为了能够看出 Rust 何时调用 drop，让暂时使用 println! 语句实现 drop。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}

```

Drop trait 包含在 prelude 中，所以无需导入它。在 CustomSmartPointer 上实现了 Drop trait，并提供了一个调用 println! 的 drop 方法实现。drop 函数体是放置任何当类型实例离开作用域时期望运行的逻辑的地方。这里选择打印一些文本以展示 Rust 何时调用 drop。

在 main 中，新建了两个 CustomSmartPointer 实例并打印出了 CustomSmartPointer created.。在 main 的结尾，CustomSmartPointer 的实例会离开作用域，而 Rust 会调用放置于 drop 方法中的代码，打印出最后的信息。注意无需显示调用 drop 方法：

#### 通过 std::mem::drop 提早丢弃值

不幸的是，并不能直截了当的禁用 drop 这个功能。通常也不需要禁用 drop ；整个 Drop trait 存在的意义在于其是自动处理的。然而，有时可能需要提早清理某个值。一个例子是当使用智能指针管理锁时；可能希望强制运行 drop 方法来释放锁以便作用域中的其他代码可以获取锁。Rust 并不允许主动调用 Drop trait 的 drop 方法；当希望在作用域结束之前就强制释放变量的话，应该使用的是由标准库提供的 std::mem::drop。

Rust 不允许显式调用 drop 因为 Rust 仍然会在 main 的结尾对值自动调用 drop，这会导致一个 double free 错误，因为 Rust 会尝试清理相同的值两次。

因为不能禁用当值离开作用域时自动插入的 drop，并且不能显示调用 drop，如果需要强制提早清理值，可以使用 std::mem::drop 函数。

std::mem::drop 函数不同于 Drop trait 中的 drop 方法。可以通过传递希望提早强制丢弃的值作为参数。std::mem::drop 位于 prelude，

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

```

#### Rc<T> 引用计数智能指针

大部分情况下所有权是非常明确的：可以准确地知道哪个变量拥有某个值。然而，有些情况单个值可能会有多个所有者。例如，在图数据结构中，多个边可能指向相同的结点，而这个结点从概念上讲为所有指向它的边所拥有。结点直到没有任何边指向它之前都不应该被清理。

为了启用多所有权，Rust 有一个叫做 `Rc<T>` 的类型。其名称为 引用计数（reference counting）的缩写。引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。

可以将其想象为客厅中的电视。当一个人进来看电视时，他打开电视。其他人也可以进来看电视。当最后一个人离开房间时，他关掉电视因为它不再被使用了。如果某人在其他人还在看的时候就关掉了电视，正在看电视的人肯定会抓狂的！

`Rc<T>` 用于当希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候。如果确实知道哪部分是最后一个结束使用的话，就可以令其成为数据的所有者同时，正常的所有权规则就可以在编译时生效。

*注意 `Rc<T>` 只能用于单线程场景；*

##### 使用 Rc<T> 共享数据

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}

```

编译会得出如下错误：

```rust
error[E0382]: use of moved value: `a`
  --> src/main.rs:13:30
   |
12 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
13 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move
   |
   = note: move occurs because `a` has type `List`, which does not implement
   the `Copy` trait

```

Cons 成员拥有其储存的数据，所以当创建 b 列表时，a 被移动进了 b 这样 b 就拥有了 a。接着当再次尝使用 a 创建 c 时，这不被允许因为 a 的所有权已经被移动。

可以改变 Cons 的定义来存放一个引用，不过接着必须指定生命周期参数。通过指定生命周期参数，表明列表中的每一个元素都至少与列表本身存在的一样久。例如，借用检查器不会允许 let a = Cons(10, &Nil); 编译，因为临时值 Nil 会在 a 获取其引用之前就被丢弃了。

相反，修改 List 的定义为使用 Rc<T> 代替 Box<T>，如列表 15-18 所示。现在每一个 Cons 变量都包含一个值和一个指向 List 的 Rc。当创建 b 时，不同于获取 a 的所有权，这里会克隆 a 所包含的 Rc，这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc 中数据的所有权。创建 c 时也会克隆 a，这会将引用计数从 2 增加为 3。每次调用 Rc::clone，Rc 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理。

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

```

需要使用 use 语句将 `Rc<T>` 引入作用域，因为它不在 prelude 中。在 main 中创建了存放 5 和 10 的列表并将其存放在 a 的新的 `Rc<List>`中。接着当创建 b 和 c 时，调用 Rc::clone 函数并传递 a 中 `Rc<List>` 的引用作为参数。

也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。Rc::clone 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。

##### 克隆 Rc<T> 会增加引用计数

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

```

在程序中每个引用计数变化的点，会打印出引用计数，其值可以通过调用 Rc::strong_count 函数获得。这个函数叫做 strong_count 而不是 count 是因为 `Rc<T>` 也有 weak_count；在 “避免引用循环：将 `Rc<T>` 变为 `Weak<T>`” 部分会讲解 weak_count 的用途。

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

能够看到 a 中 `Rc<List>` 的初始引用计数为一，接着每次调用 clone，计数会增加一。当 c 离开作用域时，计数减一。不必像调用 Rc::clone 增加引用计数那样调用一个函数来减少计数；Drop trait 的实现当 `Rc<T>` 值离开作用域时自动减少引用计数。

从这个例子所不能看到的是在 main 的结尾当 b 然后是 a 离开作用域时，此处计数会是 0，同时 Rc 被完全清理。使用 Rc 允许一个值有多个所有者，引用计数则确保只要任何所有者依然存在其值也保持有效。

通过不可变引用， `Rc<T>` 允许在程序的多个部分之间只读地共享数据。如果 `Rc<T>` 也允许多个可变引用，则会违反第四章讨论的借用规则之一：相同位置的多个可变借用可能造成数据竞争和不一致。不过可以修改数据是非常有用的！在下一部分，将讨论内部可变性模式和 `RefCell<T>` 类型，它可以与 `Rc<T>` 结合使用来处理不可变性的限制。

#### RefCell<T> 和内部可变性模式

**内部可变性（Interior mutability）**是 Rust 中的一个设计模式，它允许即使在有不可变引用时改变数据，这通常是借用规则所不允许的。为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则。还未讲到不安全代码；第十九章会学习它们。当可以确保代码在运行时会遵守借用规则，即使编译器不能保证的情况，可以选择使用那些运用内部可变性模式的类型。所涉及的 unsafe 代码将被封装进安全的 API 中，而外部类型仍然是不可变的。

##### 通过 RefCell<T> 在运行时检查借用规则

不同于 `Rc<T>`，`RefCell<T>` 代表其数据的唯一的所有权。

* 在任意给定时间，只能拥有一个可变引用或任意数量的不可变引用 之一（而不是全部）。
* 引用必须总是有效的。

对于引用和 `Box<T>`，借用规则的不可变性作用于编译时。对于 `RefCell<T>`，这些不可变性作用于 运行时。对于引用，如果违反这些规则，会得到一个编译错误。而对于 `RefCell<T>`，如果违反这些规则程序会 panic 并退出。

在编译时检查借用规则的优势是这些错误将在开发过程的早期被捕获同时对没有运行时性能影响，因为所有的分析都提前完成了。为此，在编译时检查借用规则是大部分情况的最佳选择，这也正是其为何是 Rust 的默认行为。

相反在运行时检查借用规则的好处则是允许出现特定内存安全的场景，而它们在编译时检查中是不允许的。静态分析，正如 Rust 编译器，是天生保守的。但代码的一些属性不可能通过分析代码发现：其中最著名的就是 停机问题（Halting Problem），这超出了本书的范畴，不过如果感兴趣的话这是一个值得研究的有趣主题。

因为一些分析是不可能的，如果 Rust 编译器不能通过所有权规则编译，它可能会拒绝一个正确的程序；从这种角度考虑它是保守的。如果 Rust 接受不正确的程序，那么用户也就不会相信 Rust 所做的保证了。然而，如果 Rust 拒绝正确的程序，虽然会给程序员带来不便，但不会带来灾难。`RefCell<T>` 正是用于当确信代码遵守借用规则，而编译器不能理解和确定的时候。

类似于 `Rc<T>`，`RefCell<T>` 只能用于单线程场景。如果尝试在多线程上下文中使用 `RefCell<T>`，会得到一个编译错误。

如下为选择 `Box<T>`，`Rc<T>` 或 `RefCell<T>` 的理由：

* `Rc<T>` 允许相同数据有多个所有者；`Box<T>` 和 `RefCell<T>` 有单一所有者。
* `Box<T>` 允许在编译时执行不可变或可变借用检查；`Rc<T>`仅允许在编译时执行不可变借用检查；`RefCell<T>` 允许在运行时执行不可变或可变借用检查。
* 因为 `RefCell<T>` 允许在运行时执行可变借用检查，所以可以在即便 `RefCell<T>` 自身是不可变的情况下修改其内部的值。
* 
在不可变值内部改变值就是 内部可变性 模式。让看看何时内部可变性是有用的，并讨论这是如何成为可能的。

##### 内部可变性：不可变值的可变借用

内部可变性的用例：mock 对象
测试替身（test double）是一个通用编程概念，它代表一个在测试中替代某个类型的类型。mock 对象 是特定类型的测试替身，它们记录测试过程中发生了什么以便可以断言操作是正确的。

虽然 Rust 没有与其他语言中的对象完全相同的对象，Rust 也没有像其他语言那样在标准库中内建 mock 对象功能，不过确实可以创建一个与 mock 对象有着相同功能的结构体。

如下是一个想要测试的场景：在编写一个记录某个值与最大值的差距的库，并根据当前值与最大值的差距来发送消息。例如，这个库可以用于记录用户所允许的 API 调用数量限额。

该库只提供记录与最大值的差距，以及何种情况发送什么消息的功能。使用此库的程序则期望提供实际发送消息的机制：程序可以选择记录一条消息、发送 email、发送短信等等。库本身无需知道这些细节；只需实现其提供的 Messenger trait 即可。

```rust

#![allow(unused_variables)]
fn main() {
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
             self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}
}
```

这些代码中一个重要部分是拥有一个方法 send 的 Messenger trait，其获取一个 self 的不可变引用和文本信息。这是的 mock 对象所需要拥有的接口。另一个重要的部分是需要测试 LimitTracker 的 set_value 方法的行为。可以改变传递的 value 参数的值，不过 set_value 并没有返回任何可供断言的值。也就是说，如果使用某个实现了 Messenger trait 的值和特定的 max 创建 LimitTracker，当传递不同 value 值时，消息发送者应被告知发送合适的消息。

所需的 mock 对象是，调用 send 不同于实际发送 email 或短息，其只记录信息被通知要发送了。可以新建一个 mock 对象示例，用其创建 LimitTracker，调用 LimitTracker 的 set_value 方法，然后检查 mock 对象是否有期望的消息。下例展示了一个如此尝试的 mock 对象实现，不过借用检查器并不允许：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: vec![] }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}

```

测试代码定义了一个 MockMessenger 结构体，其 sent_messages 字段为一个 String 值的 Vec 用来记录被告知发送的消息。还定义了一个关联函数 new 以便于新建从空消息列表开始的 MockMessenger 值。接着为 MockMessenger 实现 Messenger trait 这样就可以为 LimitTracker 提供一个 MockMessenger。在 send 方法的定义中，获取传入的消息作为参数并储存在 MockMessenger 的 sent_messages 列表中。

在测试中，测试了当 LimitTracker 被告知将 value 设置为超过 max 值 75% 的某个值。首先新建一个 MockMessenger，其从空消息列表开始。接着新建一个 LimitTracker 并传递新建 MockMessenger 的引用和 max 值 100。使用值 80 调用 LimitTracker 的 set_value 方法，这超过了 100 的 75%。接着断言 MockMessenger 中记录的消息列表应该有一条消息。

然而，这个测试是有问题的：

```rust
error[E0596]: cannot borrow immutable field `self.sent_messages` as mutable
  --> src/lib.rs:52:13
   |
51 |         fn send(&self, message: &str) {
   |                 ----- use `&mut self` here to make mutable
52 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ cannot mutably borrow immutable field

```

不能修改 MockMessenger 来记录消息，因为 send 方法获取了 self 的不可变引用。也不能参考错误文本的建议使用 &mut self 替代，因为这样 send 的签名就不符合 Messenger trait 定义中的签名了（可以试着这么改，看看会出现什么错误信息）。

这正是内部可变性的用武之地！将通过 RefCell 来储存 sent_messages，然后 send 将能够修改 sent_messages 并储存消息。

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
             self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(75);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
fn main() {}

```

现在 sent_messages 字段的类型是 `RefCell<Vec<String>>` 而不是 `Vec<String>`。在 new 函数中新建了一个 RefCell 示例替代空 vector。

对于 send 方法的实现，第一个参数仍为 self 的不可变借用，这是符合方法定义的。调用 self.sent_messages 中 RefCell 的 borrow_mut 方法来获取 RefCell 中值的可变引用，这是一个 vector。接着可以对 vector 的可变引用调用 push 以便记录测试过程中看到的消息。

最后必须做出的修改位于断言中：为了看到其内部 vector 中有多少个项，需要调用 RefCell 的 borrow 以获取 vector 的不可变引用。

##### `RefCell<T>` 在运行时记录借用

当创建不可变和可变引用时，分别使用 & 和 &mut 语法。对于 `RefCell<T>` 来说，则是 borrow 和 borrow_mut 方法，这属于 `RefCell<T>` 安全 API 的一部分。borrow 方法返回 Ref 类型的智能指针，borrow_mut 方法返回 RefMut 类型的智能指针。这两个类型都实现了 Deref，所以可以当作常规引用对待。

`RefCell<T>` 记录当前有多少个活动的 `Ref<T>` 和 `RefMut<T>` 智能指针。每次调用 borrow，`RefCell<T>` 将活动的不可变借用计数加一。当 Ref 值离开作用域时，不可变借用计数减一。就像编译时借用规则一样，`RefCell<T>` 在任何时候只允许有多个不可变借用或一个可变借用。

如果尝试违反这些规则，相比引用时的编译时错误，`RefCell<T>` 的实现会在运行时 panic!。下例展示了对上例中 send 实现的修改，这里故意尝试在相同作用域创建两个可变借用以便演示 `RefCell<T>` 不允许在运行时这么做：

```rust
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}

```

这里为 borrow_mut 返回的 RefMut 智能指针创建了 one_borrow 变量。接着用相同的方式在变量 two_borrow 创建了另一个可变借用。这会在相同作用域中创建一个可变引用，这是不允许的。当运行库的测试时，示例 15-23 编译时不会有任何错误，不过测试会失败：

```rust
---- tests::it_sends_an_over_75_percent_warning_message stdout ----
	thread 'tests::it_sends_an_over_75_percent_warning_message' panicked at
'already borrowed: BorrowMutError', src/libcore/result.rs:906:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.

```

##### 结合 `Rc<T>` 和 `RefCell<T>` 来拥有多个可变数据所有者

`RefCell<T>` 的一个常见用法是与 `Rc<T>` 结合。回忆一下 `Rc<T>` 允许对相同数据有多个所有者，不过只能提供数据的不可变访问。如果有一个储存了 `RefCell<T>` 的 `Rc<T>` 的话，就可以得到有多个所有者 并且 可以修改的值了！

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

```

#### 引用循环与内存泄漏

Rust 的内存安全保证使其难以意外地制造永远也不会被清理的内存（被称为 内存泄露（memory leak）），但并不是不可能。与在编译时拒绝数据竞争不同， Rust 并不保证完全地避免内存泄露，这意味着内存泄露在 Rust 被认为是内存安全的。这一点可以通过 `Rc<T>` 和 `RefCell<T>` 看出：创建引用循环的可能性是存在的。这会造成内存泄露，因为每一项的引用计数永远也到不了 0，其值也永远也不会被丢弃。

##### 制造引用循环

```rust
fn main() {}
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

```

这里采用了下例中 List 定义的另一种变体。现在 Cons 成员的第二个元素是 `RefCell<Rc<List>>`，这意味着不同于像上例那样能够修改 i32 的值，希望能够修改 Cons 成员所指向的 List。这里还增加了一个 tail 方法来方便在有 Cons 成员的时候访问其第二项。

在下例中增加了一个 main 函数，其使用了上例中的定义。这些代码在 a 中创建了一个列表，一个指向 a 中列表的 b 列表，接着修改 b 中的列表指向 a 中的列表，这会创建一个引用循环。在这个过程的多个位置有 println! 语句展示引用计数。

```rust
use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

```

这里在变量 a 中创建了一个 `Rc<List>` 实例来存放初值为 5, Nil 的 List 值。接着在变量 b 中创建了存放包含值 10 和指向列表 a 的 List 的另一个 `Rc<List>` 实例。

最后，修改 a 使其指向 b 而不是 Nil，这就创建了一个循环。为此需要使用 tail 方法获取 a 中 `RefCell<Rc<List>>` 的引用，并放入变量 link 中。接着使用 `RefCell<Rc<List>>` 的 borrow_mut 方法将其值从存放 Nil 的 Rc 修改为 b 中的 `Rc<List>`。

##### 避免引用循环：将 Rc<T> 变为 Weak<T>

到目前为止，已经展示了调用 Rc::clone 会增加 `Rc<T>` 实例的 strong_count，和只在其 strong_count 为 0 时才会被清理的 `Rc<T>` 实例。也可以通过调用 Rc::downgrade 并传递 Rc 实例的引用来创建其值的 弱引用（weak reference）。调用 Rc::downgrade 时会得到 `Weak<T>` 类型的智能指针。不同于将 `Rc<T>` 实例的 strong_count 加一，调用 Rc::downgrade 会将 weak_count 加一。`Rc<T>` 类型使用 weak_count 来记录其存在多少个 `Weak<T>` 引用，类似于 strong_count。其区别在于 weak_count 无需计数为 0 就能使 Rc 实例被清理。

强引用代表如何共享 `Rc<T>` 实例的所有权，但弱引用并不属于所有权关系。他们不会造成引用循环，因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。

因为 `Weak<T>` 引用的值可能已经被丢弃了，为了使用 `Weak<T>` 所指向的值，必须确保其值仍然有效。为此可以调用 `Weak<T>` 实例的 upgrade 方法，这会返回 `Option<Rc<T>>`。如果 `Rc<T>` 值还未被丢弃，则结果是 Some；如果 `Rc<T>` 已被丢弃，则结果是 None。因为 upgrade 返回一个 `Option<T>`，确信 Rust 会处理 Some 和 None 的情况，所以它不会返回非法指针。

会创建一个某项知道其子项和父项的树形结构的例子，而不是只知道其下一项的列表。

###### 创建树形数据结构：带有子结点的 Node

```rust

#![allow(unused_variables)]
fn main() {
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
}
```

希望能够 Node 拥有其子结点，同时也希望通过变量来共享所有权，以便可以直接访问树中的每一个 Node，为此 `Vec<T>` 的项的类型被定义为 `Rc<Node>`。还希望能修改其他结点的子结点，所以 children 中 `Vec<Rc<Node>>` 被放进了 `RefCell<T>`。

接下来，使用此结构体定义来创建一个叫做 leaf 的带有值 3 且没有子结点的 Node 实例，和另一个带有值 5 并以 leaf 作为子结点的实例 branch，如下所示：

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
   children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}

```

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
   children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}

```

这里克隆了 leaf 中的 `Rc<Node>` 并储存在了 branch 中，这意味着 leaf 中的 Node 现在有两个所有者：leaf和branch。可以通过 branch.children 从 branch 中获得 leaf，不过无法从 leaf 到 branch。leaf 没有到 branch 的引用且并不知道他们相互关联。希望 leaf 知道 branch 是其父结点。稍后会这么做。

###### 增加从子到父的引用

为了使子结点知道其父结点，需要在 Node 结构体定义中增加一个 parent 字段。问题是 parent 的类型应该是什么。知道其不能包含 `Rc<T>`，因为这样 leaf.parent 将会指向 branch 而 branch.children 会包含 leaf 的指针，这会形成引用循环，会造成其 strong_count 永远也不会为 0.

现在换一种方式思考这个关系，父结点应该拥有其子结点：如果父结点被丢弃了，其子结点也应该被丢弃。然而子结点不应该拥有其父结点：如果丢弃子结点，其父结点应该依然存在。这正是弱引用的例子！

所以 parent 使用 `Weak<T>` 类型而不是 `Rc<T>`，具体来说是 `RefCell<Weak<Node>>`。现在 Node 结构体定义看起来像这样：

```rust

#![allow(unused_variables)]
fn main() {
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
}
```

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

```

创建 leaf 结点类似于示例 15-27 中如何创建 leaf 结点的，除了 parent 字段有所不同：leaf 开始时没有父结点，所以新建了一个空的 Weak 引用实例。

此时，当尝试使用 upgrade 方法获取 leaf 的父结点引用时，会得到一个 None 值。如第一个 println! 输出所示：

```rust
leaf parent = None
```

当创建 branch 结点时，其也会新建一个 `Weak<Node>` 引用，因为 branch 并没有父结点。leaf 仍然作为 branch 的一个子结点。一旦在 branch 中有了 Node 实例，就可以修改 leaf 使其拥有指向父结点的 `Weak<Node>` 引用。这里使用了 leaf 中 parent 字段里的 `RefCell<Weak<Node>>` 的 borrow_mut 方法，接着使用了 Rc::downgrade 函数来从 branch 中的 Rc 值创建了一个指向 branch 的 `Weak<Node>` 引用。

当再次打印出 leaf 的父结点时，这一次将会得到存放了 branch 的 Some 值：现在 leaf 可以访问其父结点了！当打印出 leaf 时，

###### 可视化 strong_count 和 weak_count 的改变

通过创建了一个新的内部作用域并将 branch 的创建放入其中，来观察 `Rc<Node>` 实例的 strong_count 和 weak_count 值的变化。这会展示当 branch 创建和离开作用域被丢弃时会发生什么。

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

```

一旦创建了 leaf，其 `Rc<Node>` 的强引用计数为 1，弱引用计数为 0。在内部作用域中创建了 branch 并与 leaf 相关联，此时 branch 中 `Rc<Node>` 的强引用计数为 1，弱引用计数为 1（因为 leaf.parent 通过 `Weak<Node>` 指向 branch）。这里 leaf 的强引用计数为 2，因为现在 branch 的 branch.children 中储存了 leaf 的 `Rc<Node>` 的拷贝，不过弱引用计数仍然为 0。

当内部作用域结束时，branch 离开作用域，`Rc<Node>` 的强引用计数减少为 0，所以其 Node 被丢弃。来自 leaf.parent 的弱引用计数 1 与 Node 是否被丢弃无关，所以并没有产生任何内存泄露！

如果在内部作用域结束后尝试访问 leaf 的父结点，会再次得到 None。在程序的结尾，leaf 中 `Rc<Node>` 的强引用计数为 1，弱引用计数为 0，因为现在 leaf 又是 `Rc<Node>` 唯一的引用了。

所有这些管理计数和值的逻辑都内建于 `Rc<T>` 和 `Weak<T>` 以及它们的 Drop trait 实现中。通过在 Node 定义中指定从子结点到父结点的关系为一个`Weak<T>`引用，就能够拥有父结点和子结点之间的双向引用而不会造成引用循环和内存泄露。

### 无畏并发

安全并高效的处理并发编程是 Rust 的另一个主要目标。并发编程（Concurrent programming），代表程序的不同部分相互独立的执行，而 并行编程（parallel programming）代表程序不同部分于同时执行，这两个概念随着计算机越来越多的利用多处理器的优势时显得愈发重要。由于历史原因，在此类上下文中编程一直是困难且容易出错的：Rust 希望能改变这一点。

起初，Rust 团队认为确保内存安全和防止并发问题是两个分别需要不同方法应对的挑战。随着时间的推移，团队发现所有权和类型系统是一系列解决内存安全 和 并发问题的强有力的工具！通过利用所有权和类型检查，在 Rust 中很多并发错误都是 编译时 错误，而非运行时错误。因此，相比花费大量时间尝试重现运行时并发 bug 出现的特定情况，Rust 会拒绝编译不正确的代码并提供解释问题的错误信息。因此，可以在开发时修复代码，而不是在部署到生产环境后修复代码。给 Rust 的这一部分起了一个绰号 无畏并发（fearless concurrency）。无畏并发令的代码免于出现诡异的 bug 并可以轻松重构且无需担心会引入新的 bug。

*注意：注意：出于简洁的考虑，将很多问题归类为 并发，而不是更准确的区分 并发和（或）并行。如果这是一本专注于并发和/或并行的书，肯定会更加精确的。对于本章，当谈到 并发 时，请自行脑内替换为 并发和（或）并行。*

很多语言所提供的处理并发问题的解决方法都非常有特色。例如，Erlang 有着优雅的消息传递并发功能，但只有模糊不清的在线程间共享状态的方法。对于高级语言来说，只实现可能解决方案的子集是一个合理的策略，因为高级语言所许诺的价值来源于牺牲一些控制来换取抽象。然而对于底层语言则期望提供在任何给定的情况下有着最高的性能且对硬件有更少的抽象。因此，Rust 提供了多种工具，以符合实际情况和需求的方式来为问题建模。

如下是本章将要涉及到的内容：

* 如何创建线程来同时运行多段代码。
* 消息传递（Message passing）并发，其中通道（channel）被用来在线程间传递消息。
* 共享状态（Shared state）并发，其中多个线程可以访问同一片数据。
* Sync 和 Send trait，他们允许 Rust 的并发保证能被扩展到用户定义的和标准库中提供的类型中。

#### 使用线程同时运行代码

在大部分现代操作系统中，执行中程序的代码运行于一个 进程（process）中，操作系统则负责管理多个进程。在程序内部，也可以拥有多个同时运行的独立部分。这个运行这些独立部分的功能被称为 线程（threads）。

将程序中的计算拆分进多个线程可以改善性能，因为程序可以同时进行多个任务，不过这也会增加复杂性。因为线程是同时运行的，所以无法预先保证不同线程中的代码的执行顺序。这会导致诸如此类的问题：

* 竞争状态（Race conditions），多个线程以不一致的顺序访问数据或资源
* 死锁（Deadlocks），两个线程相互等待对方停止使用其所拥有的资源，这会阻止它们继续运行
* 只会发生在特定情况且难以稳定重现和修复的 bug

Rust 尝试缓和使用线程的负面影响。不过在多线程上下文中编程仍需格外小心，同时其所要求的代码结构也不同于运行于单线程的程序。

编程语言有一些不同的方法来实现线程。很多操作系统提供了创建新线程的 API。这种由编程语言调用操作系统 API 创建线程的模型有时被称为 1:1，一个 OS 线程对应一个语言线程。

很多编程语言提供了自己特殊的线程实现。编程语言提供的线程被称为 绿色（green）线程，使用绿色线程的语言会在不同数量的 OS 线程的上下文中执行它们。为此，绿色线程模式被称为 M:N 模型：M 个绿色线程对应 N 个 OS 线程，这里 M 和 N 不必相同。

每一个模型都有其优势和取舍。对于 Rust 来说最重要的取舍是运行时支持。运行时（Runtime）是一个令人迷惑的概念，其在不同上下文中可能有不同的含义。

在当前上下文中，运行时 代表二进制文件中包含的由语言自身提供的代码。这些代码根据语言的不同可大可小，不过任何非汇编语言都会有一定数量的运行时代码。为此，通常人们说一个语言 “没有运行时”，一般意味着 “小运行时”。更小的运行时拥有更少的功能不过其优势在于更小的二进制输出，这使其易于在更多上下文中与其他语言相结合。虽然很多语言觉得增加运行时来换取更多功能没有什么问题，但是 Rust 需要做到几乎没有运行时，同时为了保持高性能必需能够调用 C 语言，这点也是不能妥协的。

绿色线程的 M:N 模型需要更大的语言运行时来管理这些线程。因此，Rust 标准库只提供了 1:1 线程模型实现。由于 Rust 是较为底层的语言，如果愿意牺牲性能来换取的抽象，以获得对线程运行更精细的控制及更低的上下文切换成本，可以使用实现了 M:N 线程模型的 crate。

#### 使用 spawn 创建新线程

需要调用 thread::spawn 函数并传递一个闭包，并在其中包含希望在新线程运行的代码。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

```

注意这个函数编写的方式，当主线程结束时，新线程也会结束，而不管其是否执行完毕。

thread::sleep 调用强制线程停止执行一小段时间，这会允许其他不同的线程运行。这些线程可能会轮流运行，不过并不保证如此：这依赖操作系统如何调度线程。在这里，主线程首先打印，即便新创建线程的打印语句位于程序的开头，甚至即便告诉新建的线程打印直到 i 等于 9 ，它在主线程结束之前也只打印到了 5。

如果运行代码只看到了主线程的输出，或没有出现重叠打印的现象，尝试增加 range 的数值来增加操作系统切换线程的机会。

#### 使用 join 等待所有线程结束

可以通过将 thread::spawn 的返回值储存在变量中来修复新建线程部分没有执行或者完全没有执行的问题。thread::spawn 的返回值类型是 JoinHandle。JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

```

通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。阻塞（Blocking） 线程意味着阻止该线程执行工作或退出。因为将 join 调用放在了主线程的 for 循环之后.

这两个线程仍然会交替执行，不过主线程会由于 handle.join() 调用会等待直到新建线程执行完毕。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

```

#### 线程与 move 闭包

move 闭包，其经常与 thread::spawn 一起使用，因为它允许在一个线程中使用另一个线程的数据。

通过在闭包之前增加 move 关键字，强制闭包获取其使用的值的所有权，而不是任由 Rust 推断它应该借用值。

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

```

#### 使用消息传递在线程间传送数据

一个日益流行的确保安全并发的方式是 消息传递（message passing），这里线程或 actor 通过发送包含数据的消息来相互沟通。这个思想来源于 Go 编程语言文档中 的口号：“不要共享内存来通讯；而是要通讯来共享内存。”（“Do not communicate by sharing memory; instead, share memory by communicating.”）

编程中的通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）。发送者一端位于上游位置，在这里可以将橡皮鸭放入河中，接收者部分则位于下游，橡皮鸭最终会漂流至此。代码中的一部分调用发送者的方法以及希望发送的数据，另一部分则检查接收端收到到达的消息。当发送者或接收者任一被丢弃时可以认为通道被 关闭（closed）了

这里，将开发一个程序，它会在一个线程生成值向通道发送，而在另一个线程会接收值并打印出来。这里会通过通道在线程间发送简单值来演示这个功能。一旦熟悉了这项技术，就能使用通道来实现聊天系统或利用很多线程进行分布式计算并将部分计算结果发送给一个线程进行聚合。

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}

```

这里使用 mpsc::channel 函数创建一个新的通道；mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写。简而言之，Rust 标准库实现通道的方式意味着一个通道可以有多个产生值的 发送（sending）端，但只能有一个消费这些值的 接收（receiving）端。想象一下多条小河小溪最终汇聚成大河：所有通过这些小河发出的东西最后都会来到大河的下游。目前以单个生产者开始，但是当示例可以工作后会增加多个生产者。

mpsc::channel 函数返回一个元组：第一个元素是发送端，而第二个元素是接收端。由于历史原因，tx 和 rx 通常作为 发送者（transmitter）和 接收者（receiver）的缩写，所以这就是将用来绑定这两端变量的名字。这里使用了一个 let 语句和模式来解构了此元组；如此使用 let 语句是一个方便提取 mpsc::channel 返回的元组中一部分的手段。

将发送端移动到一个新建线程中并发送一个字符串，这样新建线程就可以和主线程通讯了，

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}

```

通道的接收端有两个有用的方法：recv 和 try_recv。这里，使用了 recv，它是 receive 的缩写。这个方法会阻塞主线程执行直到从通道中接收一个值。一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。

try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：可以编写一个循环来频繁调用 try_recv，再有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。

##### 通道与所有权转移

所有权规则在消息传递中扮演了重要角色，其有助于编写安全的并发代码。在并发编程中避免错误是在整个 Rust 程序中必须思考所有权所换来的一大优势。现在做一个试验来看看通道与所有权如何一同协作以避免产生问题：将尝试在新建线程中的通道中发送完 val 值 之后 再使用它。

##### 发送多个值并观察接收者的等待

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

```

在新建线程中有一个字符串 vector 希望发送到主线程。遍历他们，单独的发送每一个字符串并通过一个 Duration 值调用 thread::sleep 函数来暂停一秒。

在主线程中，不再显式调用 recv 函数：而是将 rx 当作一个迭代器。对于每一个接收到的值，将其打印出来。当通道被关闭时，迭代器也将结束。

##### 通过克隆发送者来创建多个生产者

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
// --snip--

let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

// --snip--
}

```

这一次，在创建新线程之前，对通道的发送端调用了 clone 方法。这会给一个可以传递给第一个新建线程的发送端句柄。会将原始的通道发送端传递给第二个新建线程。这样就会有两个线程，每个线程将向通道的接收端发送不同的消息。

#### 共享状态并发

在某种程度上，任何编程语言中的通道都类似于单所有权，因为一旦将一个值传送到通道中，将无法再使用这个值。共享内存类似于多所有权：多个线程可以同时访问相同的内存位置。

#### 互斥器一次只允许一个线程访问数据

互斥器（mutex）是 mutual exclusion 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据。为了访问互斥器中的数据，线程首先需要通过获取互斥器的 锁（lock）来表明其希望访问数据。锁是一个作为互斥器一部分的数据结构，它记录谁有数据的排他访问权。因此，描述互斥器为通过锁系统 保护（guarding）其数据。

mutex的缺点：

* 在使用数据之前尝试获取锁。
* 处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁。

##### `Mutex<T>`的 API 

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

```

像很多类型一样，使用关联函数 new 来创建一个 `Mutex<T>`。使用 lock 方法获取锁，以访问互斥器中的数据。这个调用会阻塞当前线程，直到拥有锁为止。

如果另一个线程拥有锁，并且那个线程 panic 了，则 lock 调用会失败。在这种情况下，没人能够再获取锁，所以这里选择 unwrap 并在遇到这种情况时使线程 panic。

一旦获取了锁，就可以将返回值（在这里是num）视为一个其内部数据的可变引用了。类型系统确保了在使用 m 中的值之前获取锁：`Mutex<i32>` 并不是一个 i32，所以 必须 获取锁才能使用这个 i32 值。是不会忘记这么做的，因为反之类型系统不允许访问内部的 i32 值。

正如所怀疑的，`Mutex<T>` 是一个智能指针。更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针。这个智能指针实现了 Deref 来指向其内部数据；其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁，

##### 多线程和多所有权

通过使用智能指针 `Rc<T>` 来创建引用计数的值，以便拥有多所有者。将例子中的 `Mutex<T>` 封装进 `Rc<T>` 中并在将所有权移入线程之前克隆了 `Rc<T>`。现在理解了所发生的错误，同时也将代码改回使用 for 循环，并保留闭包的 move 关键字：

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
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

```

再一次编译并出现了不同的错误！

```rust
error[E0277]: `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
  --> src/main.rs:11:22
   |
11 |         let handle = thread::spawn(move || {
   |                      ^^^^^^^^^^^^^ `std::rc::Rc<std::sync::Mutex<i32>>`
cannot be sent between threads safely
   |
   = help: within `[closure@src/main.rs:11:36: 14:10
counter:std::rc::Rc<std::sync::Mutex<i32>>]`, the trait `std::marker::Send`
is not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
   = note: required because it appears within the type
`[closure@src/main.rs:11:36: 14:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
   = note: required by `std::thread::spawn`

```

不幸的是，`Rc<T>` 并不能安全的在线程间共享。当 `Rc<T>` 管理引用计数时，它必须在每一个 clone 调用时增加计数，并在每一个克隆被丢弃时减少计数。`Rc<T>` 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断。在计数出错时可能会导致诡异的 bug，比如可能会造成内存泄漏，或在使用结束之前就丢弃一个值。所需要的是一个完全类似 `Rc<T>`，又以一种线程安全的方式改变引用计数的类型。

##### 原子引用计数 Arc<T>

所幸 `Arc<T>` 正是 这么一个类似 `Rc<T>` 并可以安全的用于并发环境的类型。字母 “a” 代表 原子性（atomic），所以这是一个原子引用计数（atomically reference counted）类型。原子性是另一类这里还未涉及到的并发原语：请查看标准库中 std::sync::atomic 的文档来获取更多细节。其中的要点就是：原子性类型工作起来类似原始类型，不过可以安全的在线程间共享。

为什么不是所有标准库中的类型都默认使用 `Arc<T>` 实现？原因在于线程安全带有性能损失。

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
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

```

##### `RefCell<T>`/`Rc<T>` 与 `Mutex<T>`/`Arc<T>` 的相似性

因为 counter 是不可变的，不过可以获取其内部值的可变引用；这意味着 `Mutex<T>` 提供了内部可变性，就像 Cell 系列类型那样。正如中使用 `RefCell<T>` 可以改变 `Rc<T>` 中的内容那样，同样的可以使用 `Mutex<T>` 来改变 `Arc<T>` 中的内容。

另一个值得注意的细节是 Rust 不能避免使用 `Mutex<T>` 的全部逻辑错误。回忆一下使用 `Rc<T>` 就有造成引用循环的风险，这时两个 `Rc<T>` 值相互引用，造成内存泄露。同理，`Mutex<T>` 也有造成 死锁（deadlock） 的风险。这发生于当一个操作需要锁住两个资源而两个线程各持一个锁，这会造成它们永远相互等待。如果对这个主题感兴趣，尝试编写一个带有死锁的 Rust 程序，接着研究任何其他语言中使用互斥器的死锁规避策略并尝试在 Rust 中实现他们。标准库中 `Mutex<T>`和 `MutexGuard` 的 API 文档会提供有用的信息。

#### 使用 Sync 和 Send trait 的可扩展并发

Rust 的并发模型中一个有趣的方面是：语言本身对并发知之 甚少。之前讨论的几乎所有内容，都属于标准库，而不是语言本身的内容。由于不需要语言提供并发相关的基础设施，并发方案不受标准库或语言所限：可以编写自己的或使用别人编写的并发功能。

然而有两个并发概念是内嵌于语言中的：std::marker 中的 Sync 和 Send trait。

##### 通过 Send 允许在线程间转移所有权

Send 标记 trait 表明类型的所有权可以在线程间传递。几乎所有的 Rust 类型都是Send 的，不过有一些例外，包括 `Rc<T>`：这是不能 Send 的，因为如果克隆了 `Rc<T>` 的值并尝试将克隆的所有权转移到另一个线程，这两个线程都可能同时更新引用计数。为此，`Rc<T>` 被实现为用于单线程场景，这时不需要为拥有线程安全的引用计数而付出性能代价。

因此，Rust 类型系统和 trait bound 确保永远也不会意外的将不安全的 `Rc<T>` 在线程间发送。当尝试在示例 16-14 中这么做的时候，会得到错误 `the trait Send is not implemented for Rc<Mutex<i32>>`。而使用标记为 `Send` 的 `Arc<T>` 时，就没有问题了。

任何完全由 `Send` 的类型组成的类型也会自动被标记为 `Send`。几乎所有基本类型都是 `Send` 的，

#### Sync 允许多线程访问

Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。换一种方式来说，对于任意类型 T，如果 &T（T 的引用）是 Send 的话 T 就是 Sync 的，这意味着其引用就可以安全的发送到另一个线程。类似于 Send 的情况，基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的。

通常并不需要手动实现 Send 和 Sync trait，因为由 Send 和 Sync 的类型组成的类型，自动就是 Send 和 Sync 的。因为他们是标记 trait，甚至都不需要实现任何方法。他们只是用来加强并发相关的不可变性的。

### Rust 的面向对象特性

Rust 被很多不同的编程范式影响，包括面向对象编程；函数式编程的特性。

#### 对象包含数据和行为

面向对象的程序是由对象组成的。一个 对象 包含数据和操作这些数据的过程。这些过程通常被称为 方法 或 操作。

Rust 是面向对象的：结构体和枚举包含数据而 impl 块提供了在结构体和枚举之上的方法。虽然带有方法的结构体和枚举并不被 称为 对象，但是他们提供了与对象相同的功能，

#### 封装隐藏了实现细节

另一个通常与面向对象编程相关的方面是 封装（encapsulation）的思想：对象的实现细节不能被使用对象的代码获取到。所以唯一与对象交互的方式是通过对象提供的公有 API；使用对象的代码无法深入到对象内部并直接改变数据或者行为。封装使得改变和重构对象的内部时无需改变使用对象的代码。

Rust可以使用 pub 关键字来决定模块、类型、函数和方法是公有的，而默认情况下其他一切都是私有的。

比如，可以定义一个包含一个 i32 类型 vector 的结构体 AveragedCollection。结构体也可以有一个字段，该字段保存了 vector 中所有值的平均值。这样，希望知道结构体中的 vector 的平均值的人可以随时获取它，而无需自己计算。换句话说，AveragedCollection 会为缓存平均值结果。

```rust

#![allow(unused_variables)]
fn main() {
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
}
```

注意，结构体自身被标记为 pub，这样其他代码就可以使用这个结构体，但是在结构体内部的字段仍然是私有的。这是非常重要的，因为希望保证变量被增加到列表或者被从列表删除时，也会同时更新平均值。可以通过在结构体上实现 add、remove 和 average 方法来做到这一点，

```rust

#![allow(unused_variables)]
fn main() {
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
}
```
公有方法 add、remove 和 average 是修改 AveragedCollection 实例的唯一方式。当使用 add 方法把一个元素加入到 list 或者使用 remove 方法来删除时，这些方法的实现同时会调用私有的 update_average 方法来更新 average 字段。

list 和 average 是私有的，所以没有其他方式来使得外部的代码直接向 list 增加或者删除元素，否则 list 改变时可能会导致 average 字段不同步。average 方法返回 average 字段的值，这使得外部的代码只能读取 average 而不能修改它。

因为已经封装好了 AveragedCollection 的实现细节，将来可以轻松改变类似数据结构这些方面的内容。例如，可以使用 `HashSet<i32>` 代替 `Vec<i32>` 作为 list 字段的类型。只要 add、remove 和 average 公有函数的签名保持不变，使用 AveragedCollection 的代码就无需改变。相反如果使得 list 为公有，就未必都会如此了： `HashSet<i32>` 和 `Vec<i32>` 使用不同的方法增加或移除项，所以如果要想直接修改 list 的话，外部的代码可能不得不做出修改。

如果封装是一个语言被认为是面向对象语言所必要的方面的话，那么 Rust 满足这个要求。在代码中不同的部分使用 pub 与否可以封装其实现细节。

#### 继承，作为类型系统与代码共享

继承（Inheritance）是一个很多编程语言都提供的机制，一个对象可以定义为继承另一个对象的定义，这使其可以获得父对象的数据和行为，而无需重新定义。

如果一个语言必须有继承才能被称为面向对象语言的话，那么 Rust 就不是面向对象的。无法定义一个结构体继承父结构体的成员和方法。然而，如果过去常常在的编程工具箱使用继承，根据最初考虑继承的原因，Rust 也提供了其他的解决方案。

选择继承有两个主要的原因。第一个是为了重用代码：一旦为一个类型实现了特定行为，继承可以对一个不同的类型重用这个实现。相反 Rust 代码可以使用默认 trait 方法实现来进行共享，在之前的例子中见过在 Summary trait 上增加的 summarize 方法的默认实现。任何实现了 Summary trait 的类型都可以使用 summarize 方法而无须进一步实现。这类似于父类有一个方法的实现，而通过继承子类也拥有这个方法的实现。当实现 Summary trait 时也可以选择覆盖 summarize 的默认实现，这类似于子类覆盖从父类继承的方法实现。

第二个使用继承的原因与类型系统有关：表现为子类型可以用于父类型被使用的地方。这也被称为 多态（polymorphism），这意味着如果多种对象共享特定的属性，则可以相互替代使用。

很多人将多态描述为继承的同义词。不过它是一个有关可以用于多种类型的代码的更广泛的概念。对于继承来说，这些类型通常是子类。 Rust 则通过泛型来对不同的可能类型进行抽象，并通过 trait bounds 对这些类型所必须提供的内容施加约束。这有时被称为 bounded parametric polymorphism。

近来继承作为一种语言设计的解决方案在很多语言中失宠了，因为其时常带有共享多于所需的代码的风险。子类不应总是共享其父类的所有特征，但是继承却始终如此。如此会使程序设计更为不灵活，并引入无意义的子类方法调用，或由于方法实际并不适用于子类而造成错误的可能性。某些语言还只允许子类继承一个父类，进一步限制了程序设计的灵活性。

因为这些原因，Rust 选择了一个不同的途径，使用 trait 对象而不是继承。看一下 Rust 中的 trait 对象是如何实现多态的。

#### 为使用不同类型的值而设计的 trait 对象


```rust

#![allow(unused_variables)]
fn main() {
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
}
```

这与定义使用了带有 trait bound 的泛型类型参数的结构体不同。泛型类型参数一次只能替代一个具体类型，而 trait 对象则允许在运行时替代多种具体类型。例如，可以定义 Screen 结构体来使用泛型和 trait bound，如下例所示：

```rust

#![allow(unused_variables)]
fn main() {
pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
}
```

这限制了 Screen 实例必须拥有一个全是 Button 类型或者全是 TextField 类型的组件列表。如果只需要同质（相同类型）集合，则倾向于使用泛型和 trait bound，因为其定义会在编译时采用具体类型进行单态化。

另一方面，通过使用 trait 对象的方法，一个 Screen 实例可以存放一个既能包含 `Box<Button>`，也能包含 `Box<TextField>` 的 `Vec<T>`。看看它是如何工作的，接着会讲到其运行时性能影响。

#### 实现 trait

现在来增加一些实现了 Draw trait 的类型。将提供 Button 类型。再一次重申，真正实现 GUI 库超出了本书的范畴，所以 draw 方法体中不会有任何有意义的实现。为了想象一下这个实现看起来像什么，一个 Button 结构体可能会拥有 width、height 和 label 字段，

```rust

#![allow(unused_variables)]
fn main() {
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 实际绘制按钮的代码
    }
}
}
```

在 Button 上的 width、height 和 label 字段会和其他组件不同，比如 TextField 可能有 width、height、label 以及 placeholder 字段。每一个希望能在屏幕上绘制的类型都会使用不同的代码来实现 Draw trait 的 draw 方法来定义如何绘制特定的类型，像这里的 Button 类型（并不包含任何实际的 GUI 代码，这超出了本章的范畴）。除了实现 Draw trait 之外，比如 Button 还可能有另一个包含按钮点击如何响应的方法的 impl 块。这类方法并不适用于像 TextField 这样的类型。

如果一些库的使用者决定实现一个包含 width、height 和 options 字段的结构体 SelectBox，并且也为其实现了 Draw trait，如下例所示：

```rust
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

```

库使用者现在可以在他们的 main 函数中创建一个 Screen 实例。至此可以通过将 SelectBox 和 Button 放入 `Box<T>` 转变为 trait 对象来增加组件。接着可以调用 Screen 的 run 方法，它会调用每个组件的 draw 方法。如下所示:

```rust
use gui::{Screen, Button};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

```

当编写库的时候，不知道何人会在何时增加 SelectBox 类型，不过 Screen 的实现能够操作并绘制这个新类型，因为 SelectBox 实现了 Draw trait，这意味着它实现了 draw 方法。

这个概念 —— 只关心值所反映的信息而不是其具体类型 —— 类似于动态类型语言中称为 鸭子类型（duck typing）的概念：如果它走起来像一只鸭子，叫起来像一只鸭子，那么它就是一只鸭子！在示例 17-5 中 Screen 上的 run 实现中，run 并不需要知道各个组件的具体类型是什么。它并不检查组件是 Button 或者 SelectBox 的实例。通过指定 `Box<dyn Draw>` 作为 components vector 中值的类型，就定义了 Screen 为需要可以在其上调用 draw 方法的值。

使用 trait 对象和 Rust 类型系统来进行类似鸭子类型操作的优势是无需在运行时检查一个值是否实现了特定方法或者担心在调用时因为值没有实现方法而产生错误。如果值没有实现 trait 对象所需的 trait 则 Rust 不会编译这些代码。

#### trait 对象执行动态分发

当对泛型使用 trait bound 时编译器所进行单态化处理：编译器为每一个被泛型类型参数代替的具体类型生成了非泛型的函数和方法实现。单态化所产生的代码进行 静态分发（static dispatch）。静态分发发生于编译器在编译时就知晓调用了什么方法的时候。这与 动态分发 （dynamic dispatch）相对，这时编译器在编译时无法知晓调用了什么方法。在动态分发的情况下，编译器会生成在运行时确定调用了什么方法的代码。

当使用 trait 对象时，Rust 必须使用动态分发。编译器无法知晓所有可能用于 trait 对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。为此，Rust 在运行时使用 trait 对象中的指针来知晓需要调用哪个方法。动态分发也阻止编译器有选择的内联方法代码，这会相应的禁用一些优化。

#### Trait 对象要求对象安全

只有 对象安全（object safe）的 trait 才可以组成 trait 对象。围绕所有使得 trait 对象安全的属性存在一些复杂的规则，不过在实践中，只涉及到两条规则。如果一个 trait 中所有的方法有如下属性时，则该 trait 是对象安全的：

* 返回值类型不为 Self
* 方法没有任何泛型类型参数
* 
Self 关键字是要实现 trait 或方法的类型的别名。对象安全对于 trait 对象是必须的，因为一旦有了 trait 对象，就不再知晓实现该 trait 的具体类型是什么了。如果 trait 方法返回具体的 Self 类型，但是 trait 对象忘记了其真正的类型，那么方法不可能使用已经忘却的原始具体类型。同理对于泛型类型参数来说，当使用 trait 时其会放入具体的类型参数：此具体类型变成了实现该 trait 的类型的一部分。当使用 trait 对象时其具体类型被抹去了，故无从得知放入泛型参数类型的类型是什么。

一个 trait 的方法不是对象安全的例子是标准库中的 Clone trait。Clone trait 的 clone 方法的参数签名看起来像这样：

```rust

#![allow(unused_variables)]
fn main() {
pub trait Clone {
    fn clone(&self) -> Self;
}
}
```

String 实现了 Clone trait，当在 String 实例上调用 clone 方法时会得到一个 String 实例。类似的，当调用 `Vec<T>` 实例的 clone 方法会得到一个 `Vec<T>` 实例。clone 的签名需要知道什么类型会代替 Self，因为这是它的返回值。

如果尝试做一些违反有关 trait 对象的对象安全规则的事情，编译器会产生提示。如

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Clone>>,
}

```

将会得到如下错误：

```rust
error[E0038]: the trait `std::clone::Clone` cannot be made into an object
 --> src/lib.rs:2:5
  |
2 |     pub components: Vec<Box<dyn Clone>>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone`
  cannot be made into an object
  |
  = note: the trait cannot require that `Self : Sized`

```

#### 面向对象设计模式的实现

状态模式（state pattern）是一个面向对象设计模式。该模式的关键在于一个值有某些内部状态，体现为一系列的 状态对象，同时值的行为随着其内部状态而改变。状态对象共享功能：当然，在 Rust 中使用结构体和 trait 而不是对象和继承。每一个状态对象代表负责其自身的行为和当需要改变为另一个状态时的规则的状态。持有任何一个这种状态对象的值对于不同状态的行为以及何时状态转移毫不知情。

使用状态模式意味着当程序的业务需求改变时，无需改变值持有状态或者使用值的代码。只需更新某个状态对象中的代码来改变其规则，或者是增加更多的状态对象。

为了探索这个概念，将实现一个增量式的发布博文的工作流。这个博客的最终功能看起来像这样：

1. 博文从空白的草案开始。
2. 一旦草案完成，请求审核博文。
3. 一旦博文过审，它将被发表。
4. 只有被发表的博文的内容会被打印，这样就不会意外打印出没有被审核的博文的文本。
5. 
任何其他对博文的修改尝试都是没有作用的。例如，如果尝试在请求审核之前通过一个草案博文，博文应该保持未发布的状态。

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

```

希望允许用户使用 Post::new 创建一个新的博文草案。接着希望能在草案阶段为博文编写一些文本。如果尝试在审核之前立即打印出博文的内容，什么也不会发生因为博文仍然是草案。这里增加的 assert_eq! 出于演示目的。一个好的单元测试将是断言草案博文的 content 方法返回空字符串，不过并不准备为这个例子编写单元测试。

接下来，希望能够请求审核博文，而在等待审核的阶段 content 应该仍然返回空字符串。最后当博文审核通过，它应该被发表，这意味着当调用 content 时博文的文本将被返回。

注意与 crate 交互的唯一的类型是 Post。这个类型会使用状态模式并会存放处于三种博文所可能的状态之一的值 —— 草案，等待审核和发布。状态上的改变由 Post 类型内部进行管理。状态依库用户对 Post 实例调用的方法而改变，但是不能直接管理状态变化。这也意味着用户不会在状态上犯错，比如在过审前发布博文。

##### 定义 Post 并新建一个草案状态的实例

```rust

#![allow(unused_variables)]
fn main() {
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
}
```

State trait 定义了所有不同状态的博文所共享的行为，同时 Draft、PendingReview 和 Published 状态都会实现 State 状态。现在这个 trait 并没有任何方法，同时开始将只定义 Draft 状态因为这是希望博文的初始状态。

当创建新的 Post 时，将其 state 字段设置为一个存放了 Box 的 Some 值。这个 Box 指向一个 Draft 结构体新实例。这确保了无论何时新建一个 Post 实例，它都会从草案开始。因为 Post 的 state 字段是私有的，也就无法创建任何其他状态的 Post 了！。Post::new 函数中将 content 设置为新建的空 String。

##### 存放博文内容的文本

希望能够调用一个叫做 add_text 的方法并向其传递一个 &str 来将文本增加到博文的内容中。选择实现为一个方法而不是将 content 字段暴露为 pub 。这意味着之后可以实现一个方法来控制 content 字段如何被读取。add_text 方法是非常直观的，

```rust

#![allow(unused_variables)]
fn main() {
pub struct Post {
    content: String,
}

impl Post {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
}
```

add_text 获取一个 self 的可变引用，因为需要改变调用 add_text 的 Post 实例。接着调用 content 中的 String 的 push_str 并传递 text 参数来保存到 content 中。这不是状态模式的一部分，因为它的行为并不依赖博文所处的状态。add_text 方法完全不与 state 状态交互，不过这是希望支持的行为的一部分。

##### 确保博文草案的内容是空的

```rust

#![allow(unused_variables)]
fn main() {
pub struct Post {
    content: String,
}

impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}
}
```

##### 请求审核博文来改变其状态

接下来需要增加请求审核博文的功能，这应当将其状态由 Draft 改为 PendingReview。

```rust

#![allow(unused_variables)]
fn main() {
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
}
```

这里为 Post 增加一个获取 self 可变引用的公有方法 request_review。接着在 Post 的当前状态下调用内部的 request_review 方法，并且第二个 request_review 方法会消费当前的状态并返回一个新状态。

这里给 State trait 增加了 request_review 方法；所有实现了这个 trait 的类型现在都需要实现 request_review 方法。注意不同于使用 self、 &self 或者 &mut self 作为方法的第一个参数，这里使用了 self: Box<Self>。这个语法意味着这个方法调用只对这个类型的 Box 有效。这个语法获取了 Box<Self> 的所有权，使老状态无效化以便 Post 的状态值可以将自身转换为新状态。

为了消费老状态，request_review 方法需要获取状态值的所有权。这也就是 Post 的 state 字段中 Option 的来历：调用 take 方法将 state 字段中的 Some 值取出并留下一个 None，因为 Rust 不允许在结构体中存在空的字段。这使得将 state 值移动出 Post 而不是借用它。接着将博文的 state 值设置为这个操作的结果。

这里需要将 state 临时设置为 None，不同于像 self.state = self.state.request_review(); 这样的代码直接设置 state 字段，来获取 state 值的所有权。这确保了当 Post 被转换为新状态后其不再能使用老的 state 值。

Draft 的方法 request_review 的实现返回一个新的，装箱的 PendingReview 结构体的实例，其用来代表博文处于等待审核状态。结构体 PendingReview 同样也实现了 request_review 方法，不过它不进行任何状态转换。相反它返回自身，因为请求审核已经处于 PendingReview 状态的博文应该保持 PendingReview 状态。

现在开始能够看出状态模式的优势了：Post 的 request_review 方法无论 state 是何值都是一样的。每个状态只负责它自己的规则。

将继续保持 Post 的 content 方法不变，返回一个空字符串 slice。现在可以拥有 PendingReview 状态而不仅仅是 Draft 状态的 Post 了，不过希望在 PendingReview 状态下其也有相同的行为。

##### 增加改变 content 行为的 approve 方法

approve 方法将与 request_review 方法类似：它会将 state 设置为审核通过时应处于的状态

```rust

#![allow(unused_variables)]
fn main() {
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
}
```

这里为 State trait 增加了 approve 方法，并新增了一个实现了 State 的结构体，Published 状态。

类似于 request_review，如果对 Draft 调用 approve 方法，并没有任何效果，因为它会返回 self。当对 PendingReview 调用 approve 时，它返回一个新的、装箱的 Published 结构体的实例。Published 结构体实现了 State trait，同时对于 request_review 和 approve 两方法来说，它返回自身，因为在这两种情况博文应该保持 Published 状态。

现在更新 Post 的 content 方法：如果状态为 Published 希望返回博文 content 字段的值；否则希望返回空字符串 slice，

```rust

#![allow(unused_variables)]
fn main() {
trait State {
    fn content<'a>(&self, post: &'a Post) -> &'a str;
}
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // --snip--
}
}
```

因为目标是将所有像这样的规则保持在实现了 State 的结构体中，将调用 state 中的值的 content 方法并传递博文实例（也就是 self）作为参数。接着返回 state 值的 content 方法的返回值。

这里调用 Option 的 as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权。因为 state 是一个 `Option<Box<State>>`，调用 as_ref 会返回一个 `Option<&Box<State>>`。如果不调用 as_ref，将会得到一个错误，因为不能将 state 移动出借用的 &self 函数参数。

接着调用 unwrap 方法，这里知道它永远也不会 panic，因为 Post 的所有方法都确保在他们返回时 state 会有一个 Some 值。这就是一个第十二章 “当比编译器知道更多的情况” 部分讨论过的知道 None 是不可能的而编译器却不能理解的情况。

接着就有了一个 `&Box<State>`，当调用其 content 时，解引用强制多态会作用于 & 和 Box ，这样最终会调用实现了 State trait 的类型的 content 方法。这意味着需要为 State trait 定义增加 content，这也是放置根据所处状态返回什么内容的逻辑的地方，

```rust

#![allow(unused_variables)]
fn main() {
pub struct Post {
    content: String
}
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
}
```

这里增加了一个 content 方法的默认实现来返回一个空字符串 slice。这意味着无需为 Draft 和 PendingReview 结构体实现 content 了。Published 结构体会覆盖 content 方法并会返回 post.content 的值。

注意这个方法需要生命周期注解，如第十章所讨论的。这里获取 post 的引用作为参数，并返回 post 一部分的引用，所以返回的引用的生命周期与 post 参数相关。

##### 状态模式的权衡取舍

展示了 Rust 是能够实现面向对象的状态模式的，以便能根据博文所处的状态来封装不同类型的行为。Post 的方法并不知道这些不同类型的行为。通过这种组织代码的方式，要找到所有已发布博文的不同行为只需查看一处代码：Published 的 State trait 的实现。

如果要创建一个不使用状态模式的替代实现，则可能会在 Post 的方法中，或者甚至于在 main 代码中用到 match 语句，来检查博文状态并在这里改变其行为。这意味着需要查看很多位置来理解处于发布状态的博文的所有逻辑！这在增加更多状态时会变得更糟：每一个 match 语句都会需要另一个分支。

对于状态模式来说，Post 的方法和使用 Post 的位置无需 match 语句，同时增加新状态只涉及到增加一个新 struct 和为其实现 trait 的方法。

这个实现易于扩展增加更多功能。为了体会使用此模式维护代码的简洁性，请尝试如下一些建议：

* 增加 reject 方法将博文的状态从 PendingReview 变回 Draft
* 在将状态变为 Published 之前需要两次 approve 调用
* 只允许博文处于 Draft 状态时增加文本内容。提示：让状态对象负责什么可能会修改内容而不负责修改 Post。

状态模式的一个缺点是因为状态实现了状态之间的转换，一些状态会相互联系。如果在 PendingReview 和 Published 之间增加另一个状态，比如 Scheduled，则不得不修改 PendingReview 中的代码来转移到 Scheduled。如果 PendingReview 无需因为新增的状态而改变就更好了，不过这意味着切换到另一种设计模式。

另一个缺点是会发现一些重复的逻辑。为了消除他们，可以尝试为 State trait 中返回 self 的 request_review 和 approve 方法增加默认实现，不过这会违反对象安全性，因为 trait 不知道 self 具体是什么。希望能够将 State 作为一个 trait 对象，所以需要其方法是对象安全的。

另一个重复是 Post 中 request_review 和 approve 这两个类似的实现。他们都委托调用了 state 字段中 Option 值的同一方法，并在结果中为 state 字段设置了新值。如果 Post 中的很多方法都遵循这个模式，可能会考虑定义一个宏来消除重复

完全按照面向对象语言的定义实现这个模式并没有尽可能地利用 Rust 的优势。看看一些代码中可以做出的修改，来将无效的状态和状态转移变为编译时错误。

### 模式用来匹配值的结构

模式是 Rust 中特殊的语法，它用来匹配类型中的结构，无论类型是简单还是复杂。结合使用模式和 match 表达式以及其他结构可以提供更多对程序控制流的支配权。模式由如下一些内容组合而成：

* 字面值
* 解构的数组、枚举、结构体或者元组
* 变量
* 通配符
* 占位符

#### match 分支

一个模式常用的位置是 match 表达式的分支。在形式上 match 表达式由 match 关键字、用于匹配的值和一个或多个分支构成，这些分支包含一个模式和在值匹配分支的模式时运行的表达式：

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

```

match 表达式必须是 穷尽（exhaustive）的，意为 match 表达式所有可能的值都必须被考虑到。一个确保覆盖每个可能值的方法是在最后一个分支使用捕获所有的模式：比如，一个匹配任何值的名称永远也不会失败，因此可以覆盖所有匹配剩下的情况。

有一个特定的模式 _ 可以匹配所有情况，不过它从不绑定任何变量。这在例如希望忽略任何未指定值的情况很有用。

#### `if let` 条件表达式

下例 展示了也可以组合并匹配 if let、else if 和 else if let 表达式。这相比 match 表达式一次只能将一个值与模式比较提供了更多灵活性；一系列 if let、else if、else if let 分支并不要求其条件相互关联。

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

```

#### while let 条件循环

一个与 if let 结构类似的是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环。

```rust

#![allow(unused_variables)]
fn main() {
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
}
```

这个例子会打印出 3、2 接着是 1。pop 方法取出 vector 的最后一个元素并返回 Some(value)。如果 vector 是空的，它返回 None。while 循环只要 pop 返回 Some 就会一直运行其块中的代码。一旦其返回 None，while 循环停止。可以使用 while let 来弹出栈中的每一个元素。

#### for 循环

如同第三章所讲的，for 循环是 Rust 中最常见的循环结构，不过还没有讲到的是 for 可以获取一个模式。在 for 循环中，模式是 for 关键字直接跟随的值，正如 for x in y 中的 x。

下例中展示了如何使用 for 循环来解构，或拆开一个元组作为 for 循环的一部分：

```rust

#![allow(unused_variables)]
fn main() {
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
}
```

上例的代码会打印出：

```
a is at index 0
b is at index 1
c is at index 2
```

这里使用 enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引，他们位于一个元组中。第一个 enumerate 调用会产生元组 (0, 'a')。当这个值匹配模式 (index, value)，index 将会是 0 而 value 将会是 'a'，并打印出第一行输出。

#### let的模式匹配

为了更清楚的理解 let 的模式匹配方面的内容，考虑下例 中使用 let 和模式解构一个元组：

```rust

#![allow(unused_variables)]
fn main() {
let (x, y, z) = (1, 2, 3);
}
```

#### 函数参数

函数参数也可以是模式。类似于之前对 let 所做的，可以在函数参数中匹配元组。下例将传递给函数的元组拆分为值：

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}

```

#### Refutability（可反驳性）: 模式是否会匹配失效

模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）。一个例子就是 let x = 5; 语句中的 x，因为 x 可以匹配任何值所以不可能会失败。对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）。一个这样的例子便是 if let Some(x) = a_value 表达式中的 Some(x)；如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配。

函数参数、 let 语句和 for 循环只能接受不可反驳的模式，因为通过不匹配的值程序无法进行有意义的工作。if let 和 while let 表达式被限制为只能接受可反驳的模式，因为根据定义他们意在处理可能的失败：条件表达式的功能就是根据成功或失败执行不同的操作。

通常无需担心可反驳和不可反驳模式的区别，不过确实需要熟悉可反驳性的概念，这样当在错误信息中看到时就知道如何应对。遇到这些情况，根据代码行为的意图，需要修改模式或者使用模式的结构。

为了修复在需要不可反驳模式的地方使用可反驳模式的情况，可以修改使用模式的代码：不同于使用 let，可以使用 if let。如此，如果模式不匹配，大括号中的代码将被忽略，其余代码保持有效。

```rust

#![allow(unused_variables)]
fn main() {
let some_option_value: Option<i32> = None;
if let Some(x) = some_option_value {
    println!("{}", x);
}
}
```

匹配分支必须使用可反驳模式，除了最后一个分支需要使用能匹配任何剩余值的不可反驳模式。允许将不可反驳模式用于只有一个分支的 match，不过这么做不是特别有用，并可以被更简单的 let 语句替代。

#### 模式用法：匹配字面值

```rust

#![allow(unused_variables)]
fn main() {
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
}
```

#### 模式用法：匹配命名变量

命名变量是匹配任何值的不可反驳模式，这在之前已经使用过数次。然而当其用于 match 表达式时情况会有些复杂。因为 match 会开始一个新作用域，match 表达式中作为模式的一部分声明的变量会覆盖 match 结构之外的同名变量，与所有变量一样。在下例 中，声明了一个值为 Some(5) 的变量 x 和一个值为 10 的变量 y。接着在值 x 上创建了一个 match 表达式。

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

```

看看当 match 语句运行的时候发生了什么。第一个匹配分支的模式并不匹配 x 中定义的值，所以代码继续执行。

第二个匹配分支中的模式引入了一个新变量 y，它会匹配任何 Some 中的值。因为我们在 match 表达式的新作用域中，这是一个新变量，而不是开头声明为值 10 的那个 y。这个新的 y 绑定会匹配任何 Some 中的值，在这里是 x 中的值。因此这个 y 绑定了 x 中 Some 内部的值。这个值是 5，所以这个分支的表达式将会执行并打印出 Matched, y = 5。

如果 x 的值是 None 而不是 Some(5)，头两个分支的模式不会匹配，所以会匹配下划线。这个分支的模式中没有引入变量 x，所以此时表达式中的 x 会是外部没有被覆盖的 x。在这个假想的例子中，match 将会打印 Default case, x = None。

一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了。最后的 println! 会打印 at the end: x = Some(5), y = 10。

为了创建能够比较外部 x 和 y 的值，而不引入覆盖变量的 match 表达式，我们需要相应地使用带有条件的匹配守卫（match guard）。

#### 模式用法：多个模式

在 match 表达式中，可以使用 | 语法匹配多个模式，它代表 或（or）的意思。例如，如下代码将 x 的值与匹配分支相比较，第一个分支有 或 选项，意味着如果 x 的值匹配此分支的任一个值，它就会运行：

```rust

#![allow(unused_variables)]
fn main() {
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
}
```

#### 通过 `..=` 匹配值的范围

`..=` 语法允许你匹配一个闭区间范围内的值。在如下代码中，当模式匹配任何在此范围内的值时，该分支会执行：

```rust

#![allow(unused_variables)]
fn main() {
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
}
```

如果 x 是 1、2、3、4 或 5，第一个分支就会匹配。这相比使用 | 运算符表达相同的意思更为方便；相比 1..=5，使用 | 则不得不指定 1 | 2 | 3 | 4 | 5。相反指定范围就简短的多，特别是在希望匹配比如从 1 到 1000 的数字的时候！

范围只允许用于数字或 char 值，因为编译器会在编译时检查范围不为空。char 和 数字值是 Rust 仅有的可以判断范围是否为空的类型。

如下是一个使用 char 类型值范围的例子：

```rust

#![allow(unused_variables)]
fn main() {
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
}
```

#### 解构并分解值

也可以使用模式来解构结构体、枚举、元组和引用，以便使用这些值的不同部分。

##### 解构结构体

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

```

这段代码创建了变量 a 和 b 来匹配结构体 p 中的 x 和 y 字段。这个例子展示了模式中的变量名不必与结构体中的字段名一致。不过通常希望变量名与字段名一致以便于理解变量来自于哪些字段。

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

```

这段代码创建了变量 x 和 y，与变量 p 中的 x 和 y 相匹配。其结果是变量 x 和 y 包含结构体 p 中的值。

也可以使用字面值作为结构体模式的一部分进行进行解构，而不是为所有的字段创建变量。这允许我们测试一些字段为特定值的同时创建其他字段的变量。

下例 展示了一个 match 语句将 Point 值分成了三种情况：直接位于 x 轴上（此时 y = 0 为真）、位于 y 轴上（x = 0）或不在任何轴上的点。

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

```

第一个分支通过指定字段 y 匹配字面值 0 来匹配任何位于 x 轴上的点。此模式仍然创建了变量 x 以便在分支的代码中使用。

类似的，第二个分支通过指定字段 x 匹配字面值 0 来匹配任何位于 y 轴上的点，并为字段 y 创建了变量 y。第三个分支没有指定任何字面值，所以其会匹配任何其他的 Point 并为 x 和 y 两个字段创建变量。

在这个例子中，值 p 因为其 x 包含 0 而匹配第二个分支，因此会打印出 On the y axis at 7。

##### 解构枚举

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}

```

##### 解构嵌套的结构体和枚举

```rust
enum Color {
   Rgb(i32, i32, i32),
   Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }
}

```

match 表达式第一个分支的模式匹配一个包含 Color::Rgb 枚举成员的 Message::ChangeColor 枚举成员，然后模式绑定了 3 个内部的 i32 值。第二个分支的模式也匹配一个 Message::ChangeColor 枚举成员， 但是其内部的枚举会匹配 Color::Hsv 枚举成员。可以在一个 match 表达式中指定这些复杂条件，即使会涉及到两个枚举。

##### 解构结构体和元组

甚至可以用复杂的方式来混合、匹配和嵌套解构模式。

```rust

#![allow(unused_variables)]
fn main() {
struct Point {
    x: i32,
    y: i32,
}

let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
}
```

#### 忽略模式中的值

有时忽略模式中的一些值是有用的，比如 match 中最后捕获全部情况的分支实际上没有做任何事，但是它确实对所有剩余情况负责。有一些简单的方法可以忽略模式中全部或部分值：使用 _ 模式，在另一个模式中使用 _ 模式，使用一个以下划线开始的名称，或者使用 .. 忽略所剩部分的值。让我们来分别探索如何以及为什么要这么做。

##### 使用 _ 忽略整个值

已经使用过下划线（_）作为匹配但不绑定任何值的通配符模式了。虽然 _ 模式作为 match 表达式最后的分支特别有用，也可以将其用于任意模式，包括函数参数中

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}

```

这段代码会完全忽略作为第一个参数传递的值 3，并会打印出 This code only uses the y parameter: 4。

大部分情况当你不再需要特定函数参数时，最好修改签名不再包含无用的参数。在一些情况下忽略函数参数会变得特别有用，比如实现 trait 时，当你需要特定类型签名但是函数实现并不需要某个参数时。此时编译器就不会警告说存在未使用的函数参数，就跟使用命名参数一样。

##### 使用嵌套的 _ 忽略部分值

也可以在一个模式内部使用_ 忽略部分值，例如，当只需要测试部分值但在期望运行的代码中没有用到其他部分时。下例  展示了负责管理设置值的代码。业务需求是用户不允许覆盖现有的自定义设置，但是可以取消设置，也可以在当前未设置时为其提供设置。

```rust

#![allow(unused_variables)]
fn main() {
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);
}
```

这段代码会打印出 Can't overwrite an existing customized value 接着是 setting is Some(5)。在第一个匹配分支，我们不需要匹配或使用任一个 Some 成员中的值；重要的部分是需要测试 setting_value 和 new_setting_value 都为 Some 成员的情况。在这种情况，我们打印出为何不改变 setting_value，并且不会改变它。

对于所有其他情况（setting_value 或 new_setting_value 任一为 None），这由第二个分支的 _ 模式体现，这时确实希望允许 new_setting_value 变为 setting_value。

也可以在一个模式中的多处使用下划线来忽略特定值，

```rust

#![allow(unused_variables)]
fn main() {
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
}
```

##### 通过在名字前以一个下划线开头来忽略未使用的变量

如果创建了一个变量却不在任何地方使用它, Rust 通常会给你一个警告，因为这可能会是个 bug。但是有时创建一个还未使用的变量是有用的，比如你正在设计原型或刚刚开始一个项目。这时你希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头。

```rust
fn main() {
    let _x = 5;
    let y = 10;
}

```

这里得到了警告说未使用变量 y，不过没有警告说未使用下划线开头的变量。

注意, 只使用 _ 和使用以下划线开头的名称有些微妙的不同：比如 _x 仍会将值绑定到变量，而 _ 则完全不会绑定。为了展示这个区别的意义。

```rust

#![allow(unused_variables)]
fn main() {
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
}
```

##### 用 .. 忽略剩余值

对于有多个部分的值，可以使用 .. 语法来只使用部分并忽略其它值，同时避免不得不每一个忽略值列出下划线。.. 模式会忽略模式中剩余的任何没有显式匹配的值部分。

```rust

#![allow(unused_variables)]
fn main() {
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
}
```

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}

```

这里用 first 和 last 来匹配第一个和最后一个值。.. 将匹配并忽略中间的所有值。

然而使用 .. 必须是无歧义的。如果期望匹配和忽略的值是不明确的，Rust 会报错。

#### 匹配守卫提供的额外条件

匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。匹配守卫用于表达比单独的模式所能允许的更为复杂的情况。

```rust

#![allow(unused_variables)]
fn main() {
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
}
```

如何使用匹配守卫修复变量覆盖问题。

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

```

现在这会打印出 Default case, x = Some(5)。现在第二个匹配分支中的模式不会引入一个覆盖外部 y 的新变量 y，这意味着可以在匹配守卫中使用外部的 y。相比指定会覆盖外部 y 的模式 Some(y)，这里指定为 Some(n)。此新建的变量 n 并没有覆盖任何值，因为 match 外部没有变量 n。

匹配守卫 if n == y 并不是一个模式所以没有引入新变量。这个 y 正是 外部的 y 而不是新的覆盖变量 y，这样就可以通过比较 n 和 y 来表达寻找一个与外部 y 相同的值的概念了。

也可以在匹配守卫中使用 或 运算符 | 来指定多个模式，同时匹配守卫的条件会作用于所有的模式。下例  展示了结合匹配守卫与使用了 | 的模式的优先级。这个例子中重要的部分是匹配守卫 if y 作用于 4、5 和 6，即使这看起来好像 if y 只作用于 6：

```rust

#![allow(unused_variables)]
fn main() {
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
}
```

这个匹配条件表明此分支值匹配 x 值为 4、5 或 6 同时 y 为 true 的情况。运行这段代码时会发生的是第一个分支的模式因 x 为 4 而匹配，不过匹配守卫 if y 为假，所以第一个分支不会被选择。代码移动到第二个分支，这会匹配，此程序会打印出 no。这是因为 if 条件作用于整个 4 | 5 | 6 模式，而不仅是最后的值 6。换句话说，匹配守卫与模式的优先级关系看起来像这样：

```
(4 | 5 | 6) if y => ...
```

而不是

```
4 | 5 | (6 if y) => ...
```

可以通过运行代码时的情况看出这一点：如果匹配守卫只作用于由 | 运算符指定的值列表的最后一个值，这个分支就会匹配且程序会打印出 yes。

#### @ 绑定

at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式。下例 展示了一个例子，这里我们希望测试 Message::Hello 的 id 字段是否位于 3...7 范围内，同时也希望能将其值绑定到 id_variable 变量中以便此分支相关联的代码可以使用它。可以将 id_variable 命名为 id，与字段同名，不过出于示例的目的这里选择了不同的名称。

```rust

#![allow(unused_variables)]
fn main() {
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
}
```

上例会打印出 Found an id in range: 5。通过在 3...7 之前指定 id_variable @，我们捕获了任何匹配此范围的值并同时测试其值匹配这个范围模式。

第二个分支只在模式中指定了一个范围，分支相关代码代码没有一个包含 id 字段实际值的变量。id 字段的值可以是 10、11 或 12，不过这个模式的代码并不知情也不能使用 id 字段中的值，因为没有将 id 值保存进一个变量。

最后一个分支指定了一个没有范围的变量，此时确实拥有可以用于分支代码的变量 id，因为这里使用了结构体字段简写语法。不过此分支中没有像头两个分支那样对 id 字段的值进行测试：任何值都会匹配此分支。

使用 @ 可以在一个模式中同时测试和保存变量值。

### Rust高级特性

* 不安全 Rust：用于当需要舍弃 Rust 的某些保证并负责手动维持这些保证
高级 trait：与 trait 相关的关联类型，默认类型参数，完全限定语法（fully qualified syntax），超（父）trait（supertraits）和 newtype 模式
* 高级类型：关于 newtype 模式的更多内容，类型别名，never 类型和动态大小类型

* 高级函数和闭包：函数指针和返回闭包
* 宏：定义在编译时定义更多更多代码的方式

#### 不安全 Rust

目前为止讨论过的代码都有 Rust 在编译时会强制执行的内存安全保证。然而，Rust 还隐藏有第二种语言，它不会强制执行这类内存安全保证：这被称为 不安全 Rust（unsafe Rust）。它与常规 Rust 代码无异，但是会提供额外的超级力量。

不安全 Rust 之所以存在，是因为静态分析本质上是保守的。当编译器尝试确定一段代码是否支持某个保证时，拒绝一些有效的程序比接受无效程序要好一些。这必然意味着有时代码可能是合法的，但是 Rust 不这么认为！在这种情况下，可以使用不安全代码告诉编译器，“相信我，我知道我在干什么。”这么做的缺点就是你只能靠自己了：如果不安全代码出错了，比如解引用空指针，可能会导致不安全的内存使用。

另一个 Rust 存在不安全一面的原因是：底层计算机硬件固有的不安全性。如果 Rust 不允许进行不安全操作，那么有些任务则根本完成不了。Rust 需要能够进行像直接与操作系统交互，甚至于编写你自己的操作系统这样的底层系统编程！这也是 Rust 语言的目标之一。

##### 不安全的超级力量

可以通过 unsafe 关键字来切换到不安全 Rust，接着可以开启一个新的存放不安全代码的块。这里有四类可以在不安全 Rust 中进行而不能用于安全 Rust 的操作，它们称之为 “不安全的超级力量。” 这些超级力量是：

* 解引用裸指针
* 调用不安全的函数或方法
* 访问或修改可变静态变量
* 实现不安全 trait
* 访问 union 的字段

有一点很重要，unsafe 并不会关闭借用检查器或禁用任何其他 Rust 安全检查：如果在不安全代码中使用引用，它仍会被检查。unsafe 关键字只是提供了那四个不会被编译器检查内存安全的功能。你仍然能在不安全块中获得某种程度的安全。

再者，unsafe 不意味着块中的代码就一定是危险的或者必然导致内存安全问题：其意图在于作为程序员你将会确保 unsafe 块中的代码以有效的方式访问内存。

人是会犯错误的，错误总会发生，不过通过要求这四类操作必须位于标记为 unsafe 的块中，就能够知道任何与内存安全相关的错误必定位于 unsafe 块内。保持 unsafe 块尽可能小，如此当之后调查内存 bug 时就会感谢你自己了。

为了尽可能隔离不安全代码，将不安全代码封装进一个安全的抽象并提供安全 API 是一个好主意，当我们学习不安全函数和方法时会讨论到。标准库的一部分被实现为在被评审过的不安全代码之上的安全抽象。这个技术防止了 unsafe 泄露到所有你或者用户希望使用由 unsafe 代码实现的功能的地方，因为使用其安全抽象是安全的。

让我们按顺序依次介绍上述四个超级力量，同时我们会看到一些提供不安全代码的安全接口的抽象。

###### 解引用裸指针

那里提到了编译器会确保引用总是有效的。不安全 Rust 有两个被称为 裸指针（raw pointers）的类似于引用的新类型。和引用一样，裸指针是可变或不可变的，分别写作 *const T 和 *mut T。这里的星号不是解引用运算符；它是类型名称的一部分。在裸指针的上下文中，不可变 意味着指针解引用之后不能直接赋值。

与引用和智能指针的区别在于，记住裸指针

* 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
* 不保证指向有效的内存
* 允许为空
* 不能实现任何自动清理功能

通过去掉 Rust 强加的保证，你可以放弃安全保证以换取性能或使用另一个语言或硬件接口的能力，此时 Rust 的保证并不适用。

下例展示了如何从引用同时创建不可变和可变裸指针。

```rust

#![allow(unused_variables)]
fn main() {
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
}
```

注意这里没有引入 unsafe 关键字。可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针，稍后便会看到。

这里使用 as 将不可变和可变引用强转为对应的裸指针类型。因为直接从保证安全的引用来创建他们，可以知道这些特定的裸指针是有效，但是不能对任何裸指针做出如此假设。

接下来会创建一个不能确定其有效性的裸指针，示例 19-2 展示了如何创建一个指向任意内存地址的裸指针。尝试使用任意内存是未定义行为：此地址可能有数据也可能没有，编译器可能会优化掉这个内存访问，或者程序可能会出现段错误（segmentation fault）。通常没有好的理由编写这样的代码，不过却是可行的：

```rust

#![allow(unused_variables)]
fn main() {
let address = 0x012345usize;
let r = address as *const i32;
}
```

说过可以在安全代码中创建裸指针，不过不能 解引用 裸指针和读取其指向的数据。现在我们要做的就是对裸指针使用解引用运算符 *，这需要一个 unsafe 块，如下例所示：

```rust

#![allow(unused_variables)]
fn main() {
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
}
```

创建一个指针不会造成任何危险；只有当访问其指向的值时才有可能遇到无效的值。

还需注意示例 19-1 和 19-3 中创建了同时指向相同内存位置 num 的裸指针 *const i32 和 *mut i32。相反如果尝试创建 num 的不可变和可变引用，这将无法编译因为 Rust 的所有权规则不允许拥有可变引用的同时拥有不可变引用。通过裸指针，就能够同时创建同一地址的可变指针和不可变指针，若通过可变指针修改数据，则可能潜在造成数据竞争。请多加小心！

既然存在这么多的危险，为何还要使用裸指针呢？一个主要的应用场景便是调用 C 代码接口，这在下一部分 “调用不安全函数或方法” 中会讲到。另一个场景是构建借用检查器无法理解的安全抽象。

###### 调用不安全函数或方法

第二类要求使用不安全块的操作是调用不安全函数。不安全函数和方法与常规函数方法十分类似，除了其开头有一个额外的 unsafe。unsafe 表明我们作为程序需要满足其要求，因为 Rust 不会保证满足这些要求。通过在 unsafe 块中调用不安全函数，我们表明已经阅读过此函数的文档并对其是否满足函数自身的契约负责。

```rust

#![allow(unused_variables)]
fn main() {
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
}
```

###### 创建不安全代码的安全抽象

仅仅因为函数包含不安全代码并不意味着整个函数都需要标记为不安全的。事实上，将不安全代码封装进安全函数是一个常见的抽象。作为一个例子，标准库中的函数，split_at_mut，它需要一些不安全代码，让我们探索如何可以实现它。这个安全函数定义于可变 slice 之上：它获取一个 slice 并从给定的索引参数开始将其分为两个 slice。split_at_mut 的用法如下例 所示：

```rust

#![allow(unused_variables)]
fn main() {
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
}
```

Rust 的借用检查器不能理解我们要借用这个 slice 的两个不同部分：它只知道我们借用了同一个 slice 两次。本质上借用 slice 的不同部分是可以的，因为结果两个 slice 不会重叠，不过 Rust 还没有智能到能够理解这些。当我们知道某些事是可以的而 Rust 不知道的时候，就是触及不安全代码的时候了

下例 展示了如何使用 unsafe 块，裸指针和一些不安全函数调用来实现 split_at_mut：

```rust

#![allow(unused_variables)]
fn main() {
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
}
```

slice 是一个指向一些数据的指针，并带有该 slice 的长度。可以使用 len 方法获取 slice 的长度，使用 as_mut_ptr 方法访问 slice 的裸指针。在这个例子中，因为有一个 i32 值的可变 slice，as_mut_ptr 返回一个 *mut i32 类型的裸指针，储存在 ptr 变量中。

我们保持索引 mid 位于 slice 中的断言。接着是不安全代码：slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice。这里使用此函数从 ptr 中创建了一个有 mid 个项的 slice。之后在 ptr 上调用 offset 方法并使用 mid 作为参数来获取一个从 mid 开始的裸指针，使用这个裸指针并以 mid 之后项的数量为长度创建一个 slice。

slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针，并必须确信这个指针是有效的。裸指针上的 offset 方法也是不安全的，因为其必须确信此地址偏移量也是有效的指针。因此必须将 slice::from_raw_parts_mut 和 offset 放入 unsafe 块中以便能调用它们。通过观察代码，和增加 mid 必然小于等于 len 的断言，我们可以说 unsafe 块中所有的裸指针将是有效的 slice 中数据的指针。这是一个可以接受的 unsafe 的恰当用法。

注意无需将 split_at_mut 函数的结果标记为 unsafe，并可以在安全 Rust 中调用此函数。我们创建了一个不安全代码的安全抽象，其代码以一种安全的方式使用了 unsafe 代码，因为其只从这个函数访问的数据中创建了有效的指针。

与此相对，下例 中的 slice::from_raw_parts_mut 在使用 slice 时很有可能会崩溃。这段代码获取任意内存地址并创建了一个长为一万的 slice：

```rust

#![allow(unused_variables)]
fn main() {
use std::slice;

let address = 0x01234usize;
let r = address as *mut i32;

let slice: &[i32] = unsafe {
    slice::from_raw_parts_mut(r, 10000)
};
}
```

并不拥有这个任意地址的内存，也不能保证这段代码创建的 slice 包含有效的 i32 值。试图使用臆测为有效的 slice 会导致未定义的行为。

###### 使用 extern 函数调用外部代码

有时 Rust 代码可能需要与其他语言编写的代码交互。为此 Rust 有一个关键字，extern，有助于创建和使用 外部函数接口（Foreign Function Interface， FFI）。外部函数接口是一个编程语言用以定义函数的方式，其允许不同（外部）编程语言调用这些函数。

示例 19-8 展示了如何集成 C 标准库中的 abs 函数。extern 块中声明的函数在 Rust 代码中总是不安全的。因为其他语言不会强制执行 Rust 的规则且 Rust 无法检查它们，所以确保其安全是程序员的责任：

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

```

在 extern "C" 块中，列出了我们希望能够调用的另一个语言中的外部函数的签名和名称。"C" 部分定义了外部函数所使用的 应用程序接口（application binary interface，ABI） —— ABI 定义了如何在汇编语言层面调用此函数。"C" ABI 是最常见的，并遵循 C 编程语言的 ABI。

##### 访问或修改可变静态变量

目前为止都尽量避免讨论 全局变量（global variables），Rust 确实支持他们，不过这对于 Rust 的所有权规则来说是有问题的。如果有两个线程访问相同的可变全局变量，则可能会造成数据竞争。

全局变量在 Rust 中被称为 静态（static）变量。示例 19-9 展示了一个拥有字符串 slice 值的静态变量的声明和应用：

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}

```

static 变量类似于 “变量和常量的区别” 部分讨论的常量。通常静态变量的名称采用 SCREAMING_SNAKE_CASE 写法，并 必须 标注变量的类型，在这个例子中是 &'static str。静态变量只能储存拥有 'static 生命周期的引用，这意味着 Rust 编译器可以自己计算出其生命周期而无需显式标注。访问不可变静态变量是安全的。

常量与不可变静态变量可能看起来很类似，不过一个微妙的区别是静态变量中的值有一个固定的内存地址。使用这个值总是会访问相同的地址。另一方面，常量则允许在任何被用到的时候复制其数据。

常量与静态变量的另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是 不安全 的。下例 展示了如何声明、访问和修改名为 COUNTER 的可变静态变量：

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

```

就像常规变量一样，我们使用 mut 关键来指定可变性。任何读写 COUNTER 的代码都必须位于 unsafe 块中。这段代码可以编译并如期打印出 COUNTER: 3，因为这是单线程的。拥有多个线程访问 COUNTER 则可能导致数据竞争。

拥有可以全局访问的可变数据，难以保证不存在数据竞争，这就是为何 Rust 认为可变静态变量是不安全的。任何可能的情况，请优先使用第十六章讨论的并发技术和线程安全智能指针，这样编译器就能检测不同线程间的数据访问是否是安全的。

##### 实现不安全 trait

最后一个只能用在 unsafe 中的操作是实现不安全 trait。当至少有一个方法中包含编译器不能验证的不变量时 trait 是不安全的。可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe，

```rust

#![allow(unused_variables)]
fn main() {
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
}
```

#### 高级 trait

##### 关联类型在 trait 定义中指定占位符类型

关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型。trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型。如此可以定义一个使用多种类型的 trait，直到实现此 trait 时都无需知道这些类型具体是什么。

本章所描述的大部分内容都非常少见。关联类型则比较适中；它们比本书其他的内容要少见，不过比本章中的很多内容要更常见。

一个带有关联类型的 trait 的例子是标准库提供的 Iterator trait。它有一个叫做 Item 的关联类型来替代遍历的值的类型。

```rust

#![allow(unused_variables)]
fn main() {
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
}
```

Item 是一个占位类型，同时 next 方法定义表明它返回 Option<Self::Item> 类型的值。这个 trait 的实现者会指定 Item 的具体类型，然而不管实现者指定何种类型, next 方法都会返回一个包含了此具体类型值的 Option。

关联类型看起来像一个类似泛型的概念，因为它允许定义一个函数而不指定其可以处理的类型。那么为什么要使用关联类型呢？

让我们通过一个在第十三章中出现的 Counter 结构体上实现 Iterator trait 的例子来检视其中的区别。在下例中，指定了 Item 的类型为 u32：

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--

```

##### 默认泛型类型参数和运算符重载

当使用泛型类型参数时，可以为泛型指定一个默认的具体类型。如果默认类型就足够的话，这消除了为具体类型实现 trait 的需要。为泛型类型指定默认类型的语法是在声明泛型类型时使用 `<PlaceholderType=ConcreteType>`。

这种情况的一个非常好的例子是用于运算符重载。运算符重载（Operator overloading）是指在特定情况下自定义运算符（比如 +）行为的操作。

Rust 并不允许创建自定义运算符或重载任意运算符，不过 std::ops 中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载。例如，下例 中展示了如何在 Point 结构体上实现 Add trait 来重载 + 运算符，这样就可以将两个 Point 实例相加了：

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}

```

add 方法将两个 Point 实例的 x 值和 y 值分别相加来创建一个新的 Point。Add trait 有一个叫做 Output 的关联类型，它用来决定 add 方法的返回值类型。

这里默认泛型类型位于 Add trait 中。这里是其定义：

```rust

#![allow(unused_variables)]
fn main() {
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
}
```

这看来应该很熟悉，这是一个带有一个方法和一个关联类型的 trait。比较陌生的部分是尖括号中的 RHS=Self：这个语法叫做 默认类型参数（default type parameters）。RHS 是一个泛型类型参数（“right hand side” 的缩写），它用于定义 add 方法中的 rhs 参数。如果实现 Add trait 时不指定 RHS 的具体类型，RHS 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型。

当为 Point 实现 Add 时，使用了默认的 RHS，因为我们希望将两个 Point 实例相加。让我们看看一个实现 Add trait 时希望自定义 RHS 类型而不是使用默认类型的例子

这里有两个存放不同单元值的结构体，Millimeters 和 Meters。我们希望能够将毫米值与米值相加，并让 Add 的实现正确处理转换。可以为 Millimeters 实现 Add 并以 Meters 作为 RHS，如下面所示

```rust

#![allow(unused_variables)]
fn main() {
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
}
```

为了使 Millimeters 和 Meters 能够相加，我们指定 `impl Add<Meters>` 来设定 RHS 类型参数的值而不是使用默认的 Self。

默认参数类型主要用于如下两个方面：

* 扩展类型而不破坏现有代码。
* 在大部分用户都不需要的特定情况进行自定义。

标准库的 Add trait 就是一个第二个目的例子：大部分时候你会将两个相似的类型相加，不过它提供了自定义额外行为的能力。在 Add trait 定义中使用默认类型参数意味着大部分时候无需指定额外的参数。换句话说，一小部分实现的样板代码是不必要的，这样使用 trait 就更容易了。

第一个目的是相似的，但过程是反过来的：如果需要为现有 trait 增加类型参数，为其提供一个默认类型将允许我们在不破坏现有实现代码的基础上扩展 trait 的功能。

###### 完全限定语法与消歧义：调用相同名称的方法

Rust 既不能避免一个 trait 与另一个 trait 拥有相同名称的方法，也不能阻止为同一类型同时实现这两个 trait。甚至直接在类型上实现开始已经有的同名方法也是可能的！

```rust

#![allow(unused_variables)]
fn main() {
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
}
```

 两个 trait 定义为拥有 fly 方法，并在直接定义有 fly 方法的 Human 类型上实现这两个 trait

当调用 Human 实例的 fly 时，编译器默认调用直接是现在类型上的方法

然而，关联函数是 trait 的一部分，但没有 self 参数。当同一作用域的两个类型实现了同一 trait，Rust 就不能计算出我们期望的是哪一个类型，除非使用 完全限定语法（fully qualified syntax）。例如，拿下例 中的 Animal trait 来说，它有关联函数 baby_name，结构体 Dog 实现了 Animal，同时有关联函数 baby_name 直接定义于 Dog 之上：

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}

```

这段代码用于一个动物收容所，他们将所有的小狗起名为 Spot，这实现为定义于 Dog 之上的关联函数 baby_name。Dog 类型还实现了 Animal trait，它描述了所有动物的共有的特征。小狗被称为 puppy，这表现为 Dog 的 Animal trait 实现中与 Animal trait 相关联的函数 baby_name。

在 main 调用了 Dog::baby_name 函数，它直接调用了定义于 Dog 之上的关联函数。这段代码会打印出：

```
A baby dog is called a Spot
```

对于关联函数，其没有一个 receiver，故只会有其他参数的列表。可以选择在任何函数或方法调用处使用完全限定语法。然而，允许省略任何 Rust 能够从程序中的其他信息中计算出的部分。只有当存在多个同名实现而 Rust 需要帮助以便知道我们希望调用哪个实现时，才需要使用这个较为冗长的语法。

###### 父 trait 用于在另一个 trait 中使用某 trait 的功能

有时我们可能会需要某个 trait 使用另一个 trait 的功能。在这种情况下，需要能够依赖相关的 trait 也被实现。这个所需的 trait 是我们实现的 trait 的 父（超） trait（supertrait）。

```rust

#![allow(unused_variables)]
fn main() {
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
}
```

因为指定了 OutlinePrint 需要 Display trait，则可以在 outline_print 中使用 to_string， 其会为任何实现 Display 的类型自动实现。如果不在 trait 名后增加 : Display 并尝试在 outline_print 中使用 to_string，则会得到一个错误说在当前作用域中没有找到用于 &Self 类型的方法 to_string。

让我们看看如果尝试在一个没有实现 Display 的类型上实现 OutlinePrint 会发生什么，比如 Point 结构体：

```rust

#![allow(unused_variables)]
fn main() {
trait OutlinePrint {}
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
}
```

```rust

#![allow(unused_variables)]
fn main() {
struct Point {
    x: i32,
    y: i32,
}

use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
}
```

##### newtype 模式用以在外部类型上实现外部 trait

提到了孤儿规则（orphan rule），它说明只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait。一个绕开这个限制的方法是使用 newtype 模式（newtype pattern），它涉及到在一个元组结构体（第五章 “用没有命名字段的元组结构体来创建不同的类型” 部分介绍了元组结构体）中创建一个新类型。这个元组结构体带有一个字段作为希望实现 trait 的类型的简单封装。接着这个封装类型对于 crate 是本地的，这样就可以在这个封装上实现 trait。Newtype 是一个源自（U.C.0079，逃）Haskell 编程语言的概念。使用这个模式没有运行时性能惩罚，这个封装类型在编译时就被省略了。

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

```

Display 的实现使用 self.0 来访问其内部的 `Vec<T>`，因为 Wrapper 是元组结构体而 `Vec<T>` 是结构体总位于索引 0 的项。接着就可以使用 Wrapper 中 Display 的功能了。

此方法的缺点是，因为 Wrapper 是一个新类型，它没有定义于其值之上的方法；必须直接在 Wrapper 上实现 `Vec<T>` 的所有方法，这样就可以代理到self.0 上 —— 这就允许我们完全像 `Vec<T>` 那样对待 Wrapper。如果希望新类型拥有其内部类型的每一个方法，为封装类型实现 Deref trait（第十五章 “通过 Deref trait 将智能指针当作常规引用处理” 部分讨论过）并返回其内部类型是一种解决方案。如果不希望封装类型拥有所有内部类型的方法 —— 比如为了限制封装类型的行为 —— 则必须只自行实现所需的方法。

#### 高级类型
