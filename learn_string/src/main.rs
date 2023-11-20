use std::ops::Add;

fn main() {
    let S = String::from("hello world");
    // println!("s = {}\n", S);
    borrow_String(&S);
    // S.push_str(" world"); // 出错原因是因为S不是mutable的变量
    

    let s = "hello world";
    // print!("s = {}\n", s);
    // borrow_String(s); // 出错原因是因为类型不一样,String和str是不同的类型,String是可变的，str是不可变的
    borrow_str(&S); // 但是可以通过&String强制转换成&str
    borrow_str(s);

    let mut ms = String::from("hello");
    ms.push_str(" world");
    // print!("ms = {}\n", ms);
    borrow_mut_String(&mut ms);

    let s1 = String::from("hello");
    let s2 = String::from(" world");
    // let s3 = s1 + &s2;
    let s3 = s1.add(&s2); // 两种写法是一样的
    println!("s1 + &s2 = s3:{}\n",s3);

    // 切片
    println!("s3[0..5] = {}\n", &s3[0..5]);
    let len = s3.len();
    println!("s3[0..len] = {}\n", &s3[0..len]);
}

fn borrow_String(s: &String) {
    println!("borrow String = {}\n", s);
    // s.push_str("append_something_here"); // 出错原因是因为s是不可变的，不能修改
}

fn borrow_str(s: &str) {
    println!("borrow str = {}\n", s);
}

fn borrow_mut_String(s: &mut String) {
    s.push_str(" append_something_here");
    println!("borrow mut String = {}\n", s)
}
