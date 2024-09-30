# cargo

cargo new hello_cargo

で cargo プロジェクトを作成できる

既に git レポジトリ下であれば、プロジェクトは git レポジトリとして作成されない

# log

rust_tutorial on  main
❯ cargo new hello_cargo
    Creating binary (application) `hello_cargo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

rust_tutorial on  main
❯

rust_tutorial/02.cargo on  main
❯ cd hello_cargo/    

rust_tutorial/02.cargo/hello_cargo on  main is 📦 v0.1.0 via 🦀 v1.81.0        
❯ cargo build
   Compiling hello_cargo v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/02.cargo/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.42s

rust_tutorial/02.cargo/hello_cargo on  main is 📦 v0.1.0 via 🦀 v1.81.0 took 3s

rust_tutorial/02.cargo/hello_cargo on  main is 📦 v0.1.0 via 🦀 v1.81.0 took 3s

❯ ./target/debug/hello_cargo                                                    
Hello, world!

rust_tutorial/02.cargo/hello_cargo on  main is 📦 v0.1.0 via 🦀 v1.81.0        
❯ cargo run
   Compiling hello_cargo v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/02.cargo/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.30s
     Running `target/debug/hello_cargo`
Hello, world!

rust_tutorial/02.cargo/hello_cargo on  main is 📦 v0.1.0 via 🦀 v1.81.0 took 3s

❯ cargo check
    Checking hello_cargo v0.1.0 (/mnt/c/Users/cashico/Desktop/rust_tutorial/02.cargo/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s

rust_tutorial/02.cargo/hello_cargo on  main is 📦 v0.1.0 via 🦀 v1.81.0 took 7s
