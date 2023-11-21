enum Message{
    Quit {id:i32},
    Login {user:String, password:String},
    Send {to:i32, msg:String},
}

// 使用枚举的好处是不用针对每个类型都写一个函数，而是可以把所有的类型都放到一个函数里面。
// 假如使用结构体的话，就需要针对每个结构体都写一个函数。
fn handle_message(msg:Message){
    match msg {
        Message::Quit{id} => println!("Quit with id {}", id),
        Message::Login{user, password} => println!("Login with user {} and password {}", user, password),
        Message::Send{to, msg} => println!("Send message to {} with content {}", to, msg),
    }
}

fn main() {
    let quit = Message::Quit{id:0};
    let login = Message::Login{user:String::from("user"), password:String::from("password")};
    let send = Message::Send{to:0, msg:String::from("Hello")};
    handle_message(quit);
    handle_message(login);
    handle_message(send);
}
