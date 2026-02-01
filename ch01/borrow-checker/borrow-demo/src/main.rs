fn main() {
    let s1 = String::from("hello,world");
    let len = get_str_len(&s1); // 通过&运算符获取变量的不可变引用
    println!("The length of {} is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("s = {}",s);

    // let mut s1 = String::from("hello");
    // let s2 = &mut s1;
    // let s3 = &mut s1; // 这里再次将s1的可变引用赋值给s3变量，是不允许的
    // println!("s1 = {}, s2 = {}",s1, s2);

    // 通过块级作用域定义多个可变引用
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 = {}",r1);
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    println!("r2 = {}",r2);

    // let mut s = String::from("hello");
    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题，这里可变引用不允许，因为上面已有不可变引用存在
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("r1:{r1} and r2:{r2}"); // 这里会输出r1,r2

    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s; // 没问题，因为上下文中没有同时存在可变引用和不可变引用
    println!("r3:{r3}");
}

// 获取字符串的长度，这里的s是一个不可变的引用，它没有获取值的所有权
// 函数签名使用 & 来表明参数 s 的类型是一个引用
fn get_str_len(s: &String) -> usize {
    s.len()
}

fn change(s:&mut String) {
    s.push_str(", world"); // ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}