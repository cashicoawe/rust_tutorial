fn main() {
    let hello = String::from("السلام عليكم");
    println!("hello: {}", hello);
    let hello = String::from("Dobrý den");
    println!("hello: {}", hello);
    let hello = String::from("Hello");
    println!("hello: {}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("hello: {}", hello);
    let hello = String::from("नमस्ते");
    println!("hello: {}", hello);
    let hello = String::from("こんにちは");
    println!("hello: {}", hello);
    let hello = String::from("안녕하세요");
    println!("hello: {}", hello);
    let hello = String::from("你好");
    println!("hello: {}", hello);
    let hello = String::from("Olá");
    println!("hello: {}", hello);
    let hello = String::from("Здравствуйте");
    println!("hello: {}", hello);
    let hello = String::from("Hola");
    println!("hello: {}", hello);

    println!("==========================");

    // push 系関数が所有権を奪わないことを確認できる
    // 実装？
    let mut s = String::from("foo");
    println!("s: {}", s);
    s.push_str("bar");
    println!("s: {}", s);
    s.push('m');
    println!("s: {}", s);

    println!("==========================");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{} - {} - {}", s1, s2, s3);
    println!("s: {}", s);
}
