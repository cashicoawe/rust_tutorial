fn main() {
    println!("Hello, world!");
}

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
};

let user1 = User {
    email: String::from("someone@example.com"),
    name: String::from("someone"),
    active: true,
    sign_in_count: 1,
};

// インスタンス全体が可変でなければならないことに注意してください; 
// Rustでは、一部のフィールドのみを可変にすることはできないのです.
// また、あらゆる式同様、構造体の新規インスタンスを関数本体の最後の式として生成して、 そのインスタンスを返すことを暗示できます。

let mut user1 = User {
    name: String::from("mut"),
    email: String::from("mut@address.com"),
    sign_in_count: 1,
    active: false,
};

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}

let user2 = User {
    email: String::from("changed"),
    name: String::from("changed"),
    ..user1
};