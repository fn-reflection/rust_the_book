use std::io;

fn main() {
    println!("数当てゲーム");
    println!("数字を入力してね");
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");  
    println!("あなたが入力したのは: {}", guess);
}