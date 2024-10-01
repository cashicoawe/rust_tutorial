fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("COUNT = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}


// ❯ cargo run --bin loop_label
//    Compiling flow v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/07.flow/flow)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.09s
//      Running `target/debug/loop_label`
// COUNT = 0
// remaining = 10
// remaining = 9
// COUNT = 1
// remaining = 10
// remaining = 9
// COUNT = 2
// remaining = 10
// End count = 2