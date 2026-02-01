# borrow checker
    不可变引用和可变引用

# reference
    引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。与指针不同，引用在其生命周期内保证指向某个特定类型的有效值。

## 不变引用
```rust
let s1 = String::from("hello,world");
let len = get_str_len(&s1); // 通过&运算符获取变量的不可变引用
println!("The length of {} is {}.", s1, len);

// 获取字符串的长度，这里的s是一个不可变的引用，它没有获取值的所有权
// 函数签名使用 & 来表明参数 s 的类型是一个引用
fn get_str_len(s: &String) -> usize {
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生
```
- 上述代码，定义了s1字符串对象，然后调用 get_str_len 获取其长度，这里参数s是一个不可变的引用类型；s变量我们可以理解为指向s1字符串对象的ptr指针。
- s1 是一个字符串对象，可以这么理解，它包含3部分：
  - 1. ptr指针（内存布局的指针变量）
  - 2. len 表示字符串s1对象的长度 
  - 3. capacity 表示s1字符串的容量；

## 可变引用
以下代码先定义了一个字符串，然后尝试修改它
```rust
let s = String::from("hello");
change(&s);
println!("s = {}",s);

fn change(s:&String) {
    s.push_str(", world");
}
```
当我们运行时，会报错：s.push_str(", world"); // ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
说明不能对一个不可变引用的变量进行修改。那如何解决它呢？
- 首先，我们必须将 s 改为 mut。然后在调用 change 函数的地方创建一个可变引用 &mut s。
- 然后，更新函数签名以接受一个可变引用 s: &mut String。这就非常清楚地表明，change 函数将改变它所借用的值。
改动后的代码如下：
```rust
 let mut s = String::from("hello"); // 这里声明的变量s需要使用mut关键字修饰，让其可变，也就是可以修改
change(&mut s); // 注意这里需要&mut s方式传递s的可变引用，&mut 表示可变引用；
println!("s = {}",s);

// 此时change函数签名如下，它接收一个可变引用的参数s
fn change(s:&mut String) {
    s.push_str(", world"); // ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}
```

## 可变和不可变互斥
可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。具体规则如下：
- 同一时刻，可变引用和不可变引用不能同时存在；
- 可以允许多个不可变引用，但不能同时存在可变引用；
- 同一时刻，不能存在多个可变引用；

具体示例代码如下：
```rust
let mut s1 = String::from("hello");
let s2 = &mut s1; // ------- first mutable borrow occurs here
let s3 = &mut s1; // ^^^^^^^ second mutable borrow occurs here 这里再次将s1的可变引用赋值给s3变量，是不允许的
println!("s1 = {}, s2 = {}",s1, s2); // -- first borrow later used here
```
当我们运行时，就会提示上述报错信息。这些限制，能避免数据竞争，发生竞争发生的情况如下：
- 两个或更多指针同时访问同一数据。
- 至少有一个指针被用来写入数据。
- 没有同步数据访问的机制。

数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 通过拒绝编译存在数据竞争的代码来避免此问题！

如果需要定义多个可变引用，可以通过块级作用域实现，不过实际开发过程中，不建议这么使用。
```rust
// 通过块级作用域定义多个可变引用
let mut s = String::from("hello");
{
    let r1 = &mut s;
    println!("r1 = {}",r1);
} // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

let r2 = &mut s;
println!("r2 = {}",r2);
```
那是否可以多个不可变引用，一个可变引用呢？
```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题

println!("{}, {}, and {}", r1, r2, r3);
```
上述代码，无法编译运行，因为r1，r2是不可变引用，而r3是可变引用，这种违反了`rust的可变引用和不可变引用不能同时存在的原则`。
但是我们，在使用r1,r2输出后，就可以使用s的可变引用，代码如下：
```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
println!("{r1} and {r2}");

// 此位置之后 r1 和 r2 不再使用
let r3 = &mut s; // 没问题
println!("{r3}");
```
此时r3可以正常输出，因为上下文中没有同时存在可变引用和不可变引用。那是否可以返回变量的引用呢？
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    // 这里返回了函数局部变量的引用，这是不允许的，因为s一旦离开作用域后，就不能再使用了
    &s
} // 这里 s 离开作用域并被丢弃。其内存被释放。
```
- 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个悬垂指针（dangling pointer）—— 指向可能已被分配给其他用途的内存位置的指针。
- 相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂引用：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
- 上面代码 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放。不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String，这可不对！Rust 不会允许我们这么做。

不发生悬挂引用的方式，可以返回变量的所有权
```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    // 所有权被移动出去，所以没有值被释放。
    s
}
```
关于引用的规则如下：
- 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。 
- 变量的引用必须总是有效的，不能离开作用域后，还继续使用变量的引用，这是rust不允许的，借用检查器在编译时会阻止编译运行。
