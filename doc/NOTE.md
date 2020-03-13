
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

第一行`[package]`是节标题，指示以下语句正在配置程序包。当我们向该文件添加更多信息时，我们将添加其他部分。

接下来的四行设置了Cargo编译程序所需的配置信息：名称，版本，编写者以及要使用的Rust版本。Cargo从的环境中获取的姓名和电子邮件信息，因此，如果该信息不正确，请立即修复此信息，然后保存文件。我们将edition在附录E中讨论密钥。

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

Cargo具有一种机制，可确保您或您其他任何人每次构建代码时都可以重建相同的工件：Cargo将仅使用您指定的依赖项版本，除非您另有说明。例如，如果下周发布的版本为0.5.6的rand包

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

默认情况下变量是不可变的。这是Rust推动您编写代码的一种方式，它可以利用Rust提供的安全性和易并发性。但是，您仍然可以选择使变量可变。让我们探讨一下Rust如何以及为什么鼓励您支持不变性，以及为什么有时您可能想要退出。

当变量是不可变的时，将值绑定到名称后，就无法更改该值。为了说明这一点，我们使用来 在项目目录中生成一个名为变量的新项目。

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;   //error!
    println!("The value of x is: {}", x);
}
```

#### 变量和常量之间的差异

无法更改变量的值可能使您想起了大多数其他语言都具有的另一个编程概念：常量。像不可变变量一样，常量是绑定到名称且不允许更改的值，但是常量和变量之间存在一些差异。

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

Rust中的每个值都具有特定的数据类型，它告诉Rust指定了哪种数据，因此知道如何使用该数据。我们将研究两个数据类型子集：标量和复合。

请记住，Rust是一种静态类型的语言，这意味着它必须在编译时知道所有变量的类型。编译器通常可以根据值以及如何使用它来推断我们要使用的类型。在当许多类型是可能的，

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

#### 标量类型

标量类型表示一个单一的值。Rust具有四种主要的标量类型：整数，浮点数，布尔值和字符。您可能会从其他编程语言中识别这些。让我们跳到他们在Rust中的工作方式。

##### 整数类型

一个整数是没有小数部分的数。在第2章中，我们使用了一种整数`u32`类型。此类型声明表示与之关联的值应为占用32位空间的无符号整数（有符号整数类型以开头`i`，而不是`u`）。表3-1显示了Rust中的内置整数类型。`Signed`和`Unsigned`列中的每个变体（例如`i16`）都可以用来声明整数值的类型。

长度|	符号|	无符号
-|-|-
8位|	i8|	u8
16位|	i16|	u16
32位|	i32|	u32
64位|	i64|	u64
128位|	i128|	u128

每个变体可以是有符号的也可以是无符号的，并且具有明确的大小。 带符号和无符号表示数字是负数还是正数，换句话说，数字是否需要带有正负号（带正负号），或者数字将永远是正数，因此可以不带正负号表示（未签名）。这就像在纸上写数字一样：当符号很重要时，数字上会显示加号或减号。但是，可以放心地假设数字为正数时，它不会显示任何符号。带符号的数字使用二进制补码表示存储。

每个带符号的变量可以存储\[-2^{n - 1}\]到\[ 2^{n-1}-1 \]之间的数字，其中n是变量使用的位数。因此，一个 `i8`可从存储-128至127，其等于-128到127无符号的变体可以存储从0编号.

此外，`isize`和`usize`类型取决于您的程序所运行的计算机类型：如果您使用的是64位体系结构，则为64位；如果您使用的是32位体系结构，则为32位。

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

Rust支持您期望所有数字类型的基本数学运算：加，减，乘，除和余数。以下代码显示了如何在let语句中使用每个代码

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

到目前为止，我们仅处理数字，但是Rust也支持字母。Rust的 char类型是该语言最原始的字母类型，下面的代码显示了一种使用它的方式。（请注意，char文字是用单引号指定的，而字符串文字是使用双引号的。）

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Rust的`char`类型为4个字节，代表Unicode标量值，这意味着它可以代表的不仅仅是ASCII。重音字母；中文，日文和韩文字符；表情符号 和零宽度空格char在Rust 中都是有效值。Unicode标值的范围从`U+0000`到 `U+D7FF`和`U+E000`到`U+10FFFF`包容性。但是，“字符”在Unicode中并不是真正的概念，因此您对“字符”是什么的直觉可能与`char`Rust中的a 不一致。

##### 复合类型

复合类型可以将多个值组合为一种类型。Rust有两种原始的复合类型：元组和数组。

###### 元组类型

元组是一种将多种类型的值组合为一个复合类型的一般方法。元组的长度是固定的：声明后，它们的大小就无法增长或缩小。

我们通过在括号内编写逗号分隔的值列表来创建元组。元组中的每个位置都有一个类型，并且元组中不同值的类型不必相同。在此示例中，我们添加了可选的类型注释：

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```

程序首先创建一个元组并将其绑定到变量`tup`。然后，它使用带有`let`采取`tup`并把它变成三个独立的变量`x`，`y`和`z`。这称为解构，因为它将单个元组分为三部分。最后，程序将输出的值 `y`，即`6.4`。

除了通过模式匹配进行结构分解之外，我们还可以通过使用句点（.）和要访问的值的索引直接访问元组元素。例如：

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

当您希望将数据分配在堆栈而不是堆上时（当我们在第4章中讨论堆栈和堆时），或者要确保始终拥有固定数量的元素时，数组很有用。但是，数组不像矢量类型那样灵活。载体是由标准库提供一个类似集合类型是允许生长或尺寸的缩小。如果不确定使用数组还是向量，则可能应该使用向量。第8章将更详细地讨论向量。

一个程序可能需要使用数组而不是向量，例如，该程序需要知道一年中各个月份的名称。这样的程序不太可能需要添加或删除月份，因此可以使用数组，因为您知道它将始终包含12个元素：

```rust

#![allow(unused_variables)]
fn main() {
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
}
```

您将使用方括号编写数组的类型，并且在方括号内包括每个元素的类型，分号，然后是数组中元素的数量，如下所示

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

数组是在堆栈上分配的单个内存块。您可以使用索引访问数组的元素，如下所示：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

```

在此示例中，名为的变量`first`将获得值`1`，因为这是`[0]`数组中index处的值。名为的变量`second`将从数组中的`2`索引获取值`[1]`。


### Rust函数
