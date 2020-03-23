
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
let hello = String::from("你好");
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

该`#[cfg(test)]`测试模块上的注解告诉锈编译只有当你运行运行测试代码cargo test，而不是当你运行cargo build。当只想构建库时，这样可以节省编译时间，并且由于不包括测试，因此可以节省生成的编译工件中的空间。会看到，由于集成测试位于不同的目录中，因此它们不需要`#[cfg(test)]`注释。但是，由于单元测试与代码位于相同的文件中，因此将用于`#[cfg(test)]`指定不应将其包含在编译结果中。

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

*注意：请注意，std::env::args如果任何参数包含无效的Unicode ，将感到异常。如果您的程序需要接受包含无效Unicode的参数，请std::env::args_os改用。该函数返回一个迭代器，该迭代器生成OsString值而不是String值。我们之所以选择std::env::args此处为简单起见，因为OsString每个平台的值不同，并且使用起来String比值更复杂。*

#### 将参数值保存在变量中

打印参数向量的值说明该程序能够访问指定为命令行参数的值。现在我们需要将两个参数的值保存在变量中，以便可以在程序的其余部分中使用这些值。

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

正如看到的那样，在打印矢量时，程序的名称占据了矢量中的第一个值`args[0]`，因此我们从index开始1。第一个参数minigrep是我们要搜索的字符串，因此我们在变量中引用了第一个参数query。第二个参数是文件名，因此我们在变量中引用了第二个参数filename。

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

在此程序中，使用了以前没有涉及的方法： unwrap_or_else，它是`Result<T, E>`由标准库定义的。使用unwrap_or_else允许我们定义一些自定义的，非panic!错误的处理。如果Result是一个Ok值，则此方法的行为类似于unwrap：它返回Ok包装的内部值。但是，如果值是一个Err值，则此方法将调用闭包中的代码，该闭包是我们定义的匿名函数，并作为参数传递给unwrap_or_else。

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

将其余的程序逻辑分离到run函数中，我们可以像Config::new清单12-9中那样改进错误处理。当出现问题时expect，run 函数将返回a ，而不是通过调用使程序异常`Result<T, E>`。这将使我们进一步main以用户友好的方式整合到处理错误的逻辑中。

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

我们使用if let而不是unwrap_or_else检查是否run返回Err值，然后调用返回 值process::exit(1)。该run函数不会以 返回实例unwrap的相同方式Config::new返回我们想要的值Config。因为在成功情况下run返回()，所以我们只关心检测错误，因此我们不需要unwrap_or_else返回展开的值，因为它只会是()。

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

1. 编写一个失败的测试并运行它，以确保由于您期望的原因而失败。
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

目前，我们的测试失败了，因为我们总是返回一个空向量。要解决此问题并实施search，我们的程序需要遵循以下步骤：

* 遍历内容的每一行。
* 检查该行是否包含我们的查询字符串。
* 如果是这样，请将其添加到我们要返回的值列表中。
* 如果没有，则什么也不做。
* 返回匹配的结果列表。

让我们完成每个步骤，从遍历行开始。

##### 通过该lines方法迭代线

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

##### 在每一行中搜索查询

接下来，我们将检查当前行是否包含查询字符串。幸运的是，字符串有一个有用的名为的方法contains，可以为我们做到这一点！contains在search函数中添加对方法的调用

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

现在 search 函数是可以工作并测试通过了的，我们需要实际在 run 函数中调用 search。需要将 config.query 值和 run 从文件中读取的 contents 传递给 search 函数。接着 run 会打印出 search 返回的每一行：

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

我们希望增加一个新函数 search_case_insensitive，并将会在设置了环境变量时调用它。这里将继续遵循 TDD 过程，其第一步是再次编写一个失败测试。我们将为新的大小写不敏感搜索函数新增一个测试函数，并将老的测试函数从 one_result 改名为 case_sensitive 来更清楚的表明这两个测试的区别，

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

注意我们也改变了老测试中 contents 的值。还新增了一个含有文本 "Duct tape." 的行，它有一个大写的 D，这在大小写敏感搜索时不应该匹配 "duct"。我们修改这个测试以确保不会意外破坏已经实现的大小写敏感搜索功能；这个测试现在应该能通过并在处理大小写不敏感搜索时应该能一直通过。

大小写 不敏感 搜索的新测试使用 "rUsT" 作为其查询字符串。在我们将要增加的 search_case_insensitive 函数中，"rUsT" 查询应该包含带有一个大写 R 的 "Rust:" 还有 "Trust me." 这两行，即便他们与查询的大小写都不同。这个测试现在会编译失败因为还没有定义 search_case_insensitive 函数。请随意增加一个总是返回空 vector 的骨架实现，

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

首先我们将 query 字符串转换为小写，并将其覆盖到同名的变量中。对查询字符串调用 to_lowercase 是必需的，这样不管用户的查询是 "rust"、"RUST"、"Rust" 或者 "rUsT"，我们都将其当作 "rust" 处理并对大小写不敏感。

注意 query 现在是一个 String 而不是字符串 slice，因为调用 to_lowercase 是在创建新数据，而不是引用现有数据。如果查询字符串是 "rUsT"，这个字符串 slice 并不包含可供我们使用的小写的 u 或 t，所以必需分配一个包含 "rust" 的新 String。现在当我们将 query 作为一个参数传递给 contains 方法时，需要增加一个 & 因为 contains 的签名被定义为获取一个字符串 slice。

接下来在检查每个 line 是否包含 search 之前增加了一个 to_lowercase 调用将他们都变为小写。现在我们将 line 和 query 都转换成了小写，这样就可以不管查询的大小写进行匹配了。

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

这里增加了 case_sensitive 字符来存放一个布尔值。接着我们需要 run 函数检查 case_sensitive 字段的值并使用它来决定是否调用 search 函数或 search_case_insensitive 函数，

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

最后需要实际检查环境变量。处理环境变量的函数位于标准库的 env 模块中，所以我们需要在 src/lib.rs 的开头增加一个 use std::env; 行将这个模块引入作用域中。接着在 Config::new 中使用 env 模块的 var 方法来检查一个叫做 CASE_INSENSITIVE 的环境变量，

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

这里创建了一个新变量 case_sensitive。为了设置它的值，需要调用 env::var 函数并传递我们需要寻找的环境变量名称，CASE_INSENSITIVE。env::var 返回一个 Result，它在环境变量被设置时返回包含其值的 Ok 成员，并在环境变量未被设置时返回 Err 成员。

我们使用 Result 的 is_err 方法来检查其是否是一个 error（也就是环境变量未被设置的情况），这也就意味着我们 需要 进行一个大小写敏感搜索。如果CASE_INSENSITIVE 环境变量被设置为任何值，is_err 会返回 false 并将进行大小写不敏感搜索。我们并不关心环境变量所设置的 值，只关心它是否被设置了，所以检查 is_err 而不是 unwrap、expect 或任何我们已经见过的 Result 的方法。

我们将变量 case_sensitive 的值传递给 Config 实例，这样 run 函数可以读取其值并决定是否调用 search 或者示例 12-22 中实现的 search_case_insensitive。

#### 检查错误应该写入何处

首先，让我们观察一下目前 minigrep 打印的所有内容是如何被写入标准输出的，包括那些应该被写入标准错误的错误信息。可以通过将标准输出流重定向到一个文件同时有意产生一个错误来做到这一点。我们没有重定向标准错误流，所以任何发送到标准错误的内容将会继续显示在屏幕上。

命令行程序被期望将错误信息发送到标准错误流，这样即便选择将标准输出流重定向到文件中时仍然能看到错误信息。目前我们的程序并不符合期望；相反我们将看到它将错误信息输出保存到了文件中。

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

本章我们不会讨论函数式编程是或不是什么的问题，而是展示 Rust 的一些在功能上与其他被认为是函数式语言类似的特性。

更具体的，我们将要涉及：

* **闭包（Closures）**，一个可以储存在变量里的类似函数的结构
* **迭代器（Iterators）**，一种处理元素序列的方式

#### 闭包：可以捕获环境的匿名函数

Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算。不同于函数，闭包允许捕获调用者作用域中的值。我们将展示闭包的这些功能如何复用代码和自定义行为。

#### 使用闭包创建行为的抽象

让我们来看一个存储稍后要执行的闭包的示例。其间我们会讨论闭包的语法、类型推断和 trait。

考虑一下这个假想的情况：我们在一个通过 app 生成自定义健身计划的初创企业工作。其后端使用 Rust 编写，而生成健身计划的算法需要考虑很多不同的因素，比如用户的年龄、身体质量指数（Body Mass Index）、用户喜好、最近的健身活动和用户指定的强度系数。本例中实际的算法并不重要，重要的是这个计算只花费几秒钟。我们只希望在需要时调用算法，并且只希望调用一次，这样就不会让用户等得太久。

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

接下来，main 函数中将会包含本例的健身 app 中的重要部分。这代表当用户请求健身计划时 app 会调用的代码。因为与 app 前端的交互与闭包的使用并不相关，所以我们将硬编码代表程序输入的值并打印输出。

所需的输入有这些：

* 一个来自用户的 intensity 数字，请求健身计划时指定，它代表用户喜好低强度还是高强度健身。
* 一个随机数，其会在健身计划中生成变化。

程序的输出将会是建议的锻炼计划。示例 13-2 展示了我们将要使用的 main 函数：

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

现在有了执行上下文，让我们编写算法。示例 13-3 中的 generate_workout 函数包含本例中我们最关心的 app 业务逻辑。本例中余下的代码修改都将在这个函数中进行：

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

现在这份代码能够应对我们的需求了，但数据科学部门的同学告知我们将来会对调用 simulated_expensive_calculation 的方式做出一些改变。为了在要做这些改动的时候简化更新步骤，我们将重构代码来让它只调用 simulated_expensive_calculation 一次。同时还希望去掉目前多余的连续两次函数调用，并不希望在计算过程中增加任何其他此函数的调用。也就是说，我们不希望在完全无需其结果的情况调用函数，不过仍然希望只调用函数一次。

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

不同于总是在 if 块之前调用 simulated_expensive_calculation 函数并储存其结果，我们可以定义一个闭包并将其储存在变量中，如示例 13-5 所示。实际上可以选择将整个 simulated_expensive_calculation 函数体移动到这里引入的闭包中：

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

注意这个 let 语句意味着 expensive_closure 包含一个匿名函数的 定义，不是调用匿名函数的 返回值。回忆一下使用闭包的原因是我们需要在一个位置定义代码，储存代码，并在之后的位置实际调用它；期望调用的代码现在储存在 expensive_closure 中。

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

仍然在第一个 if 块中调用了闭包两次，这调用了慢计算代码两次而使得用户需要多等待一倍的时间。可以通过在 if 块中创建一个本地变量存放闭包调用的结果来解决这个问题，不过闭包可以提供另外一种解决方案。我们稍后会讨论这个方案，不过目前让我们首先讨论一下为何闭包定义中和所涉及的 trait 中没有类型注解。

#### 闭包类型推断和注解

闭包不要求像 fn 函数那样在参数和返回值上注明类型。函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分。严格的定义这些接口对于保证所有人都认同函数使用和返回值的类型来说是很重要的。但是闭包并不用于这样暴露在外的接口：他们储存在变量中并被使用，不用命名他们或暴露给库的用户调用。

闭包通常很短并只与对应相对任意的场景较小的上下文中。在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型，类似于它是如何能够推断大部分变量的类型一样。

强制在这些小的匿名函数中注明类型是很恼人的，并且与编译器已知的信息存在大量的重复。

类似于变量，如果相比严格的必要性你更希望增加明确性并变得更啰嗦，可以选择增加类型注解；

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

幸运的是，还有另一个可用的方案。可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。你可能见过这种模式被称 memoization 或 lazy evaluation。

为了让结构体存放闭包，我们需要指定闭包的类型，因为结构体定义需要知道其每一个字段的类型。每一个闭包实例有其自己独有的匿名类型：也就是说，即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。为了定义使用闭包的结构体、枚举或函数参数，需要像第十章讨论的那样使用泛型和 trait bound。

Fn 系列 trait 由标准库提供。所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个。在 “闭包会捕获其环境” 部分我们会讨论这些 trait 的区别；在这个例子中可以使用 Fn trait。

为了满足 Fn trait bound 我们增加了代表闭包所必须的参数和返回值类型的类型。在这个例子中，闭包有一个 u32 的参数并返回一个 u32，这样所指定的 trait bound 就是 Fn(u32) -> u32。

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

结构体 Cacher 有一个泛型 T 的字段 calculation。T 的 trait bound 指定了 T 是一个使用 Fn 的闭包。任何我们希望储存到 Cacher 实例的 calculation 字段的闭包必须有一个 u32 参数（由 Fn 之后的括号的内容指定）并必须返回一个 u32（由 -> 之后的内容）。

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

Cacher 结构体的字段是私有的，因为我们希望 Cacher 管理这些值而不是任由调用代码潜在的直接改变他们。

Cacher::new 函数获取一个泛型参数 T，它定义于 impl 块上下文中并与 Cacher 结构体有着相同的 trait bound。Cacher::new 返回一个在 calculation 字段中存放了指定闭包和在 value 字段中存放了 None 值的 Cacher 实例，因为我们还未执行闭包。

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

值缓存是一种更加广泛的实用行为，我们可能希望在代码中的其他闭包中也使用他们。然而，目前 Cacher 的实现存在两个小问题，这使得在不同上下文中复用变得很困难。

```rust
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

```

这个测试使用返回传递给它的值的闭包创建了一个新的 Cacher 实例。使用为 1 的 arg 和为 2 的 arg 调用 Cacher 实例的 value 方法，同时我们期望使用为 2 的 arg 调用 value 会返回 2。

这里的问题是第一次使用 1 调用 c.value，Cacher 实例将 Some(1) 保存进 self.value。在这之后，无论传递什么值调用 value，它总是会返回 1。

尝试修改 Cacher 存放一个哈希 map 而不是单独一个值。哈希 map 的 key 将是传递进来的 arg 值，而 value 则是对应 key 调用闭包的结果值。相比之前检查 self.value 直接是 Some 还是 None 值，现在 value 函数会在哈希 map 中寻找 arg，如果找到的话就返回其对应的值。如果不存在，Cacher 会调用闭包并将结果值保存在哈希 map 对应 arg 值的位置。

当前 Cacher 实现的第二个问题是它的应用被限制为只接受获取一个 u32 值并返回一个 u32 值的闭包。比如说，我们可能需要能够缓存一个获取字符串 slice 并返回 usize 值的闭包的结果。请尝试引入更多泛型参数来增加 Cacher 功能的灵活性。

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

编译器甚至会提示我们这只能用于闭包！

当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。这会使用内存并产生额外的开销，在更一般的场景中，当我们不需要闭包来捕获环境时，我们不希望产生这些开销。因为函数从未允许捕获环境，定义和使用函数也就从不会有这些额外开销。

闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 Fn trait：

* FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
* FnMut 获取可变的借用值所以可以改变其环境
* Fn 从其环境获取不可变的借用值

当创建一个闭包时，Rust 根据其如何使用环境中变量来推断我们希望如何引用环境。由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce 。那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut ，而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn 。

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

大部分需要指定一个 Fn 系列 trait bound 的时候，可以从 Fn 开始，而编译器会根据闭包体中的情况告诉你是否需要 FnMut 或 FnOnce。

为了展示闭包作为函数参数时捕获其环境的作用，让我们继续下一个主题：迭代器。

#### 使用迭代器处理元素序列

迭代器模式允许你对一个项的序列进行某些处理。迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。

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

在标准库中没有提供迭代器的语言中，我们可能会使用一个从 0 开始的索引变量，使用这个变量索引 vector 中的值，并循环增加其值直到达到 vector 的元素数量。

迭代器为我们处理了所有这些逻辑，这减少了重复代码并消除了潜在的混乱。另外，迭代器的实现方式提供了对多种不同的序列使用相同逻辑的灵活性，而不仅仅是像 vector 这样可索引的数据结构.让我们看看迭代器是如何做到这些的。

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

注意这里有一下我们还未讲到的新语法：type Item 和 Self::Item，他们定义了 trait 的 关联类型（associated type）。第十九章会深入讲解关联类型，不过现在只需知道这段代码表明实现 Iterator trait 要求同时定义一个 Item 类型，这个 Item 类型被用作 next 方法的返回值类型。换句话说，Item 类型将是迭代器返回元素的类型。

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

另外需要注意到从 next 调用中得到的值是 vector 的不可变引用。iter 方法生成一个不可变引用的迭代器。如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。类似的，如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter。

#### 消费迭代器的方法

Iterator trait 有一系列不同的由标准库提供默认实现的方法；你可以在 Iterator trait 的标准库 API 文档中找到所有这些方法。一些方法在其定义中调用了 next 方法，这也就是为什么在实现 Iterator trait 时要求实现 next 方法的原因。

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

Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），他们允许我们将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。

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

在 下例 中，我们将遍历由 map 调用生成的迭代器的结果收集到一个 vector 中，它将会含有原始 vector 中每个元素加 1 的结果：

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

现在介绍了迭代器，让我们展示一个通过使用 filter 迭代器适配器和捕获环境的闭包的常规用例。迭代器的 filter 方法获取一个使用迭代器的每一个项并返回布尔值的闭包。如果闭包返回 true，其值将会包含在 filter 提供的新迭代器中。如果闭包返回 false，其值不会包含在结果迭代器中。

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

已经展示了可以通过在 vector 上调用 iter、into_iter 或 iter_mut 来创建一个迭代器。也可以用标准库中其他的集合类型创建迭代器，比如哈希 map。另外，可以实现 Iterator trait 来创建任何我们希望的迭代器。正如之前提到的，定义中唯一要求提供的方法就是 next 方法。一旦定义了它，就可以使用所有其他由 Iterator trait 提供的拥有默认实现的方法来创建自定义迭代器了！

作为展示，让我们创建一个只会从 1 数到 5 的迭代器。首先，创建一个结构体来存放一些值，接着实现 Iterator trait 将这个结构体放入迭代器中并在此实现中使用其值。

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

Counter 结构体有一个字段 count。这个字段存放一个 u32 值，它会记录处理 1 到 5 的迭代过程中的位置。count 是私有的因为我们希望 Counter 的实现来管理这个值。new 函数通过总是从为 0 的 count 字段开始新实例来确保我们需要的行为。

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

我们希望迭代器对其内部状态加一，这也就是为何将 count 初始化为 0：我们希望迭代器首先返回 1。如果 count 值小于 6，next 会返回封装在 Some 中的当前值，不过如果 count 大于或等于 6，迭代器会返回 None。

#### 使用 Counter 迭代器的 next 方法

一旦实现了 Iterator trait，我们就有了一个迭代器！下例 展示了一个测试用来演示使用 Counter 结构体的迭代器功能，通过直接调用 next 方法，

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

这个测试在 counter 变量中新建了一个 Counter 实例并接着反复调用 next 方法，来验证我们实现的行为符合这个迭代器返回从 1 到 5 的值的预期。

#### 使用自定义迭代器中其他 Iterator trait 方法

通过定义 next 方法实现 Iterator trait，我们现在就可以使用任何标准库定义的拥有默认实现的 Iterator trait 方法了，因为他们都使用了 next 方法的功能。

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

所有这些方法调用都是可能的，因为我们指定了 next 方法如何工作，而标准库则提供了其它调用 next 的方法的默认实现。

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

起初这里需要 clone 的原因是参数 args 中有一个 String 元素的 slice，而 new 函数并不拥有 args。为了能够返回 Config 实例的所有权，我们需要克隆 Config 中字段 query 和 filename 的值，这样 Config 实例就能拥有这些值。

在学习了迭代器之后，我们可以将 new 函数改为获取一个有所有权的迭代器作为参数而不是借用 slice。我们将使用迭代器功能之前检查 slice 长度和索引特定位置的代码。这会明确 Config::new 的工作因为迭代器会负责访问这些值。

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

env::args 函数返回一个迭代器！不同于将迭代器的值收集到一个 vector 中接着传递一个 slice 给 Config::new，现在我们直接将 env::args 返回的迭代器的所有权传递给 Config::new。

接下来需要更新 Config::new 的定义。在 I/O 项目的 src/lib.rs 中，将 Config::new 的签名改为如 下例 所示。这仍然不能编译因为我们还需更新函数体：

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

请记住 env::args 返回值的第一个值是程序的名称。我们希望忽略它并获取下一个值，所以首先调用 next 并不对返回值做任何操作。之后对希望放入 Config 中字段 query 调用 next。如果 next 返回 Some，使用 match 来提取其值。如果它返回 None，则意味着没有提供足够的参数并通过 Err 值提早返回。对 filename 值进行同样的操作。

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

可以通过使用迭代器适配器方法来编写更简明的代码。这也避免了一个可变的中间 results vector 的使用。函数式编程风格倾向于最小化可变状态的数量来使代码更简洁。去掉可变状态可能会使得将来进行并行搜索的增强变得更容易，因为我们不必管理 results vector 的并发访问。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

```

#### 性能对比：循环 VS 迭代器

为了决定使用哪个实现，我们需要知道哪个版本的 search 函数更快一些：是直接使用 for 循环的版本还是使用迭代器的版本。

运行了一个性能测试，通过将阿瑟·柯南·道尔的“福尔摩斯探案集”的全部内容加载进 String 并寻找其中的单词 “the”。如下是 for 循环版本和迭代器版本的 search 函数的性能测试结果：

结果迭代器版本还要稍微快一点！这里我们将不会查看性能测试的代码，我们的目的并不是为了证明他们是完全等同的，而是得出一个怎样比较这两种实现方式性能的基本思路。

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

像音频解码器这样的程序通常最看重计算的性能。这里，我们创建了一个迭代器，使用了两个适配器，接着消费了其值。Rust 代码将会被编译为什么样的汇编代码呢？好吧，在编写本书的这个时候，它被编译成与手写的相同的汇编代码。遍历 coefficients 的值完全用不到循环：Rust 知道这里会迭代 12 次，所以它“展开”（unroll）了循环。展开是一种移除循环控制代码的开销并替换为每个迭代中的重复代码的优化。

所有的系数都被储存在了寄存器中，这意味着访问他们非常快。这里也没有运行时数组访问边界检查。所有这些 Rust 能够提供的优化使得结果代码极为高效。现在知道这些了，请放心大胆的使用迭代器和闭包吧！他们使得代码看起来更高级，但并不为此引入运行时性能损失。

### 只能指针

指针 （pointer）是一个包含内存地址的变量的通用概念。这个地址引用，或 “指向”（points at）一些其他数据。Rust 中最常见的指针是第四章介绍的 引用（reference）。引用以 & 符号为标志并借用了他们所指向的值。除了引用数据没有任何其他特殊功能。它们也没有任何额外开销，所以应用的最多。

另一方面，智能指针（smart pointers）是一类数据结构，他们的表现类似指针，但是也拥有额外的元数据和功能。智能指针的概念并不为 Rust 所独有；其起源于 C++ 并存在于其他语言中。Rust 标准库中不同的智能指针提供了多于引用的额外功能。本章将会探索的一个例子便是 引用计数 （reference counting）智能指针类型，其允许数据有多个所有者。引用计数智能指针记录总共有多少个所有者，并当没有任何所有者时负责清理数据。

在 Rust 中，普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针；相反，在大部分情况下，智能指针 拥有 他们指向的数据。

实际上本书中已经出现过一些智能指针，比如第八章的 String 和 Vec<T>，虽然当时我们并不这么称呼它们。这些类型都属于智能指针因为它们拥有一些数据并允许你修改它们。它们也带有元数据（比如他们的容量）和额外的功能或保证（String 的数据总是有效的 UTF-8 编码）。

智能指针通常使用结构体实现。智能指针区别于常规结构体的显著特性在于其实现了 Deref 和 Drop trait。Deref trait 允许智能指针结构体实例表现的像引用一样，这样就可以编写既用于引用、又用于智能指针的代码。Drop trait 允许我们自定义当智能指针离开作用域时运行的代码。本章会讨论这些 trait 以及为什么对于智能指针来说他们很重要。

考虑到智能指针是一个在 Rust 经常被使用的通用设计模式，本章并不会覆盖所有现存的智能指针。很多库都有自己的智能指针而你也可以编写属于你自己的智能指针。这里将会讲到的是来自标准库中最常用的一些：

* `Box<T>`，用于在堆上分配值
* `Rc<T>`，一个引用计数类型，其数据可以有多个所有者
* `Ref<T>` 和 `RefMut<T>`，通过 `RefCell<T>` 访问，一个在运行时而不是在编译时执行借用规则的类型。

#### 使用Box <T>指向堆上的数据

最简单直接的智能指针是 box，其类型是 `Box<T>`。 box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。如果你想回顾一下栈与堆的区别请参考第四章。

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

这里定义了变量 b，其值是一个指向被分配在堆上的值 5 的 Box。这个程序会打印出 b = 5；在这个例子中，我们可以像数据是储存在栈上的那样访问 box 中的数据。正如任何拥有数据所有权的值那样，当像 b 这样的 box 在 main 的末尾离开作用域时，它将被释放。这个释放过程作用于 box 本身（位于栈上）和它所指向的数据（位于堆上）。

将一个单独的值存放在堆上并不是很有意义，所以像示例 15-1 这样单独使用 box 并不常见。将像单个 i32 这样的值储存在栈上，也就是其默认存放的地方在大部分使用场景中更为合适。让我们看看一个不使用 box 时无法定义的类型的例子。

#### Box 允许创建递归类型

Rust 需要在编译时知道类型占用多少空间。一种无法在编译时知道大小的类型是 递归类型（recursive type），其值的一部分可以是相同类型的另一个值。这种值的嵌套理论上可以无限的进行下去，所以 Rust 不知道递归类型需要多少空间。不过 box 有一个已知的大小，所以通过在循环类型定义中插入 box，就可以创建递归类型了。

##### cons list 的更多内容

cons list 是一个来源于 Lisp 编程语言及其方言的数据结构。在 Lisp 中，cons 函数（“construct function" 的缩写）利用两个参数来构造一个新的列表，他们通常是一个单独的值和另一个列表。

cons 函数的概念涉及到更常见的函数式编程术语；“将 x 与 y 连接” 通常意味着构建一个新的容器而将 x 的元素放在新容器的开头，其后则是容器 y 的元素。

cons list 的每一项都包含两个元素：当前项的值和下一项。其最后一项值包含一个叫做 Nil 的值且没有下一项。cons list 通过递归调用 cons 函数产生。代表递归的终止条件（base case）的规范名称是 Nil，它宣布列表的终止。注意这不同于第六章中的 “null” 或 “nil” 的概念，他们代表无效或缺失的值。 

注意虽然函数式编程语言经常使用 cons list，但是它并不是一个 Rust 中常见的类型。大部分在 Rust 中需要列表的时候，`Vec<T>` 是一个更好的选择。其他更为复杂的递归数据类型 确实 在 Rust 的很多场景中很有用，不过通过以 cons list 作为开始，我们可以探索如何使用 box 毫不费力的定义一个递归数据类型。

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

这个错误表明这个类型 “有无限的大小”。其原因是 List 的一个成员被定义为是递归的：它直接存放了另一个相同类型的值。这意味着 Rust 无法计算为了存放 List 值到底需要多少空间。让我们一点一点来看：首先了解一下 Rust 如何决定需要多少空间来存放一个非递归类型。

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

因为 `Box<T>` 是一个指针，我们总是知道它需要多少空间：指针的大小并不会根据其指向的数据量而改变。这意味着可以将 Box 放入 Cons 成员中而不是直接存放另一个 List 值。Box 会指向另一个位于堆上的 List 值，而不是存放在 Cons 成员中。从概念上讲，我们仍然有一个通过在其中 “存放” 其他列表创建的列表，不过现在实现这个概念的方式更像是一个项挨着另一项，而不是一项包含另一项。

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

Cons 成员将会需要一个 i32 的大小加上储存 box 指针数据的空间。Nil 成员不储存值，所以它比 Cons 成员需要更少的空间。现在我们知道了任何 List 值最多需要一个 i32 加上 box 指针数据的大小。通过使用 box ，打破了这无限递归的连锁，这样编译器就能够计算出储存 List 值需要的大小了。

box 只提供了间接存储和堆分配；他们并没有任何其他特殊的功能，比如我们将会见到的其他智能指针。它们也没有这些特殊功能带来的性能损失，所以他们可以用于像 cons list 这样间接存储是唯一所需功能的场景。

`Box<T>` 类型是一个智能指针，因为它实现了 Deref trait，它允许 `Box<T>` 值被当作引用对待。当 `Box<T>` 值离开作用域时，由于 `Box<T>` 类型 Drop trait 的实现，box 所指向的堆数据也会被清除。让我们更详细的探索一下这两个 trait。这两个 trait 对于在本章余下讨论的其他智能指针所提供的功能中，将会更为重要。

#### 通过 Deref trait 将智能指针当作常规引用处理

实现 Deref trait 允许我们重载 解引用运算符（dereference operator）*（与乘法运算符或 glob 运算符相区别）。通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。

让我们首先看看解引用运算符如何处理常规引用，接着尝试定义我们自己的类似 `Box<T>` 的类型并看看为何解引用运算符不能像引用一样工作。我们会探索如何实现 Deref trait 使得智能指针以类似引用的方式工作变为可能。最后，我们会讨论 Rust 的 解引用强制多态（deref coercions）功能和它是如何一同处理引用或智能指针的。

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

为了体会默认智能指针的行为不同于引用，让我们创建一个类似于标准库提供的 `Box<T>` 类型的智能指针。接着会学习如何增加使用解引用运算符的功能。

从根本上说，`Box<T>` 被定义为包含一个元素的元组结构体，所以下例以相同的方式定义了 `MyBox<T>` 类型。我们还定义了 new 函数来对应定义于 `Box<T>` 的 new 函数：

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

这里定义了一个结构体 MyBox 并声明了一个泛型参数 T，因为我们希望其可以存放任何类型的值。MyBox 是一个包含 T 类型元素的元组结构体。MyBox::new 函数获取一个 T 类型的参数并返回一个存放传入值的 MyBox 实例。

尝试将下例中的代码加入示例 15-8 中并修改 main 使用我们定义的 `MyBox<T>` 类型代替 `Box<T>`。示例 15-9 中的代码不能编译，因为 Rust 不知道如何解引用 MyBox：

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

`MyBox<T>` 类型不能解引用我们并没有为其实现这个功能。为了启用 * 运算符的解引用功能，需要实现 Deref trait。

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

Rust 将 * 运算符替换为先调用 deref 方法再进行直接引用的操作，如此我们便不用担心是不是还需要手动调用 deref 方法了。Rust 的这个特性可以让我们写出行为一致的代码，无论是面对的是常规引用还是实现了 Deref 的类型。

deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有权。如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。在这里以及大部分使用解引用运算符的情况下我们并不希望获取 `MyBox<T>` 内部值的所有权。

注意，每次当我们在代码中使用 * 时， * 运算符都被替换成了先调用 deref 方法再接着使用 * 解引用的操作，且只会发生一次，不会对 * 操作符无限递归替换，解引用出上面 i32 类型的值就停止了，这个值与 下例 中 assert_eq! 的 5 相匹配。

#### 函数和方法的隐式解引用强制多态

解引用强制多态（deref coercions）是 Rust 表现在函数或方法传参上的一种便利。其将实现了 Deref 的类型的引用转换为原始类型通过 Deref 所能够转换的类型的引用。当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，解引用强制多态将自动发生。这时会有一系列的 deref 方法被调用，把我们提供的类型转换成了参数所需的类型。

解引用强制多态的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用。这个功能也使得我们可以编写更多同时作用于引用或智能指针的代码。

作为展示解引用强制多态的实例，让我们使用上例中定义的 `MyBox<T>`，以及下例中增加的 Deref 实现。下例展示了一个有着字符串 slice 参数的函数定义：

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

(*m) 将 `MyBox<String>` 解引用为 String。接着 `&` 和 `[..]` 获取了整个 String 的字符串 slice 来匹配 hello 的签名。没有解引用强制多态所有这些符号混在一起将更难以读写和理解。解引用强制多态使得 Rust 自动的帮我们处理这些转换。

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

对于智能指针模式来说第二个重要的 trait 是 Drop，其允许我们在值要离开作用域时执行一些代码。可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源。我们在智能指针上下文中讨论 Drop 是因为其功能几乎总是用于实现智能指针。例如，Box<T> 自定义了 Drop 用来释放 box 所指向的堆空间。

在其他一些语言中，我们不得不记住在每次使用完智能指针实例后调用清理内存或资源的代码。如果忘记的话，运行代码的系统可能会因为负荷过重而崩溃。在 Rust 中，可以指定一些代码应该在值离开作用域时被执行，而编译器会自动插入这些代码。于是我们就不需要在程序中到处编写在实例结束时清理这些变量的代码 —— 而且还不会泄露资源。

指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。为了能够看出 Rust 何时调用 drop，让我们暂时使用 println! 语句实现 drop。

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

Drop trait 包含在 prelude 中，所以无需导入它。我们在 CustomSmartPointer 上实现了 Drop trait，并提供了一个调用 println! 的 drop 方法实现。drop 函数体是放置任何当类型实例离开作用域时期望运行的逻辑的地方。这里选择打印一些文本以展示 Rust 何时调用 drop。

在 main 中，我们新建了两个 CustomSmartPointer 实例并打印出了 CustomSmartPointer created.。在 main 的结尾，CustomSmartPointer 的实例会离开作用域，而 Rust 会调用放置于 drop 方法中的代码，打印出最后的信息。注意无需显示调用 drop 方法：

#### 通过 std::mem::drop 提早丢弃值

不幸的是，并不能直截了当的禁用 drop 这个功能。通常也不需要禁用 drop ；整个 Drop trait 存在的意义在于其是自动处理的。然而，有时你可能需要提早清理某个值。一个例子是当使用智能指针管理锁时；你可能希望强制运行 drop 方法来释放锁以便作用域中的其他代码可以获取锁。Rust 并不允许我们主动调用 Drop trait 的 drop 方法；当我们希望在作用域结束之前就强制释放变量的话，我们应该使用的是由标准库提供的 std::mem::drop。

Rust 不允许我们显式调用 drop 因为 Rust 仍然会在 main 的结尾对值自动调用 drop，这会导致一个 double free 错误，因为 Rust 会尝试清理相同的值两次。

因为不能禁用当值离开作用域时自动插入的 drop，并且不能显示调用 drop，如果我们需要强制提早清理值，可以使用 std::mem::drop 函数。

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


