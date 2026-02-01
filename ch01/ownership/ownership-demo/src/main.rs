fn main() {
    // 作用域
    // 1.块作用域
    {
        let s1 = "hello";
        println!("s1:{}",s1);
    } // 这一行执行时，s1变量的作用范围就会自动失效，也就是说会自动清理s1
    // println!("s1={}",s1); // 这里就不能使用块级作用域中的s1

    // 2.所有权转移
    let s = String::from("hello");
    let s2 = s;
    // 这里就不能继续使用s，运行会报错：^ value borrowed here after move
    // 因为s的所有者已转移到了s2，因此s不能再使用了，Rust 会自动调用 drop 函数并清理变量的堆内存
    // println!("s: {}", s);
    println!("s2: {}", s2);

    // 3.rust中变量默认是不可变的
    // let greeter = String::from("hello");
    // greeter.push_str(", world!"); // ^^^^^^^ cannot borrow as mutable
    // 为了解决上述问题，需要将greeter变成可变类型，通过关键字mut来修饰变量
    let mut greeter = String::from("hello"); // 此时greeter是可变类型的变量
    greeter.push_str(", world"); // 通过push_str追加字面量字符串
    println!("{}", greeter);

    // 4. 可变变量修改
    let mut s = String::from("hello");
    println!("s: {}", s);
    // 当你给一个已有的变量赋一个全新的值时，Rust 将会立即调用 drop 并释放原始值的内存
    // 也就是说，原有分配的内存s就会自动释放hello存放空间，s新存储的值是daheige
    s = String::from("daheige");
    println!("{}, world!",s);

    // 5. 深度拷贝 deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 这里通过调用clone方法，克隆出一个新的变量s2
    println!("s1 = {s1}, s2 = {s2}"); // 这里可以继续使用s1

    // 6.函数一般来说，默认会发生所有权转移
    let s1 = String::from("hello");
    takes_ownership(s1);// 将s1的所有权转移到了函数内部，当函数执行完毕后，s1的所有权就会自动被释放了，也就是s1变量就会失效
    // println!("s1 is {}", s1);// 这里s1就不能再使用，会报错：^^ value borrowed here after move
    let x = 5;
    makes_copy(x);
    println!("x = {}", x);

    // 将s2所有权转移到函数内部，然后返回
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {s3}");

    // 通过不可变引用方式（借用）传递参数
    let s2 = String::from("hello");
    borrow_ownership(&s2);
    println!("s2 = {}", s2);
}

// 将所有权转移到了函数内部
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// 基本类型x这里发生是栈上数据拷贝操作，因此函数调用完毕后，这里的x还可以继续使用
fn makes_copy(x: i32) {
    println!("{}", x);
}

// 这里传递的是字符串类型的引用，借用方式
fn borrow_ownership(s: &str) {
    println!("{}", s);
}

// 该函数将传入字符串并返回该值
fn takes_and_gives_back(s: String) -> String {
    // s 进入作用域
    println!("s ={}",s);

    s  // 返回 s 字符串类型的所有权，移出给调用的函数
}