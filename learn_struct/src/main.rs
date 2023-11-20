#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 对于可变的结构体，必须指明mutable，你可能会想在结构体中设置一些变量是可变的，而另一些是不可变的，但是目前Rust还不支持这种语法。
    // 结构体的值在赋值的时候，所有权也会被转移。
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("email@user1.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("user1 is {:?}", user1);

    // user1.username = "new name".to_string();
    user1.username = String::from("new name");

    println!("user1 is {:?}", user1);
}
