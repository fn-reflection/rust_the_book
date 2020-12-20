
// RUST_BACKTRACE=1 cargo run // stack backtraceを少し出す
// RUST_BACKTRACE=full cargo run //  stack backtraceを完全に出す

// enum Result<T, E> {
//    Ok(T),
//    Err(E),
// }

fn incurable_error() {
    // panic!("crash and burn"); // panicを引き起こす。
    let v = vec![1, 2, 3];
    // v[99]; // index out of bounds
    println!("{:?}",v);
}

use std::fs::File;

fn curable_error() {
    let f = File::open("hello.txt");
    println!("{:?}",f);  // Result型を返す。
}

use std::io::ErrorKind;

// パターンの文脈において、&は参照にマッチしその値を返す。
// refは値にマッチしそれへの参照を返す。
fn error_handling() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => { // refはerrorがガード式に所有権を奪われないように必要
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => { panic!("Tried to create file but cannot create: {:?}", e)},
            }
        },
        Err(error) => {panic!("There was a problem opening the file: {:?}", error)},
    };
    println!("{:?}",f);
}


fn use_unwrap() {
    // let f = File::open("hello3.txt").unwrap(); // hello.txtがないとpanic!を呼ぶシンタックスシュガー。
    // let f = File::open("hello.txt").expect("Failed to open hello.txt"); // expectはエラー出力を上書きするイメージ
    // prototypingには有用
}

use std::io::Read;
fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


fn read_username_from_file2() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s) // ?演算子は内部でreturn Err(e)を返すので Result型である必要がある。
}

use std::net::IpAddr;

pub struct Guess { value: u32 } // 数当てゲーム用の構造体、valueはprivateなのでアクセスできない。
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 { panic!("Guess value must be between 1 and 100, got {}.", value); }
        Guess { value } // 公開されたnewメソッドを通じてのみGuessを作れる。
    }
    pub fn value(&self) -> u32 { self.value } // valueへの公開アクセサメソッド(ゲッター)の定義
}

fn main() {
    incurable_error();
    curable_error();
    error_handling();
    use_unwrap();
    let f = read_username_from_file();
    println!("{:?}", f);
    let f = read_username_from_file2();
    println!("{:?}", f);
    let f = read_username_from_file3();
    println!("{:?}", f);
    let home: IpAddr = "127.0.0.1".parse().unwrap(); // この場合parseに失敗することはないのでunwrapするのはあり
    println!("{}", home);

}
