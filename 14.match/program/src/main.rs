enum Coin {
    Penny,
    Nickel, 
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let cents = value_in_cents(Coin::Penny);
    println!("cents: {}", cents);
}