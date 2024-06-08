fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    //インスタンス全体が可変でなければならない
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

////他のインスタンスからインスタンスを生成する
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        //明示的にセットされていない残りのフィールドは省略可能
        ..user1
    };

}


fn build_user(email: String, username: String) -> User {
//仮引数名と構造体のフィールド名が同じなので省略可能
    User {
        //email: email,
        email,
        //username: username,
        username,
        active: true,
        sign_in_count: 1,
    }
}
