#[derive(Debug)] // Rust標準のデバッグ用出力実装でstd::fmt::Debugトレイトを実装することを宣言する
                 // これをしないとDebug出力すらできない。Rustは構造体のデフォルトの出力形式を規定しない
                 // std::fmt::Displayではヒューマンリーダブルな出力を実装すべきである、pythonで言う所の__str__だ
struct User { // 構造体の定義r
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User { // Userのインスタンスに紐づく関数、すなわち(インスタンス)メソッドを実装
    fn double_count(&self) -> u64 { // selfを使えばメソッド
        2 * self.sign_in_count // 普通はselfを&で借用する。selfの所有権を奪うのは、構造体を変えてselfを捨てたいという特殊な場合のみのはず
    }
}

impl User { // implは複数回書いても問題ない、C#のpartial classっぽい、今回の事例では分けて書くメリットはない。トレイトなどの実装時に役立つ？
    fn test_account() -> User { // selfを使わないと関連関数、特定の構造体にてグルーピングされた関数、Javaで言う所のスタティックメソッド
        User {
            email:String::from("test@example.com"), username:String::from("test"), active: true, sign_in_count: 1
        }
    } // 別にcreate_test_user_accountという関数でこれを実装していいと思うが、折角Userという括りがあるのでそのグループに入れとくイメージ
}

// struct User {  
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// } // エラー &strはスライス参照なので所有権はない、つまりUser自体がusernameの値の所有権を持たないためライフタイムを定義する必要がある

struct LikeUnit { // haskellでいうところユニット型の構造体の定義、トレイトの実装などに使うらしい。型自体はデータを持たない。
}

struct Color(i32, i32, i32); // タプル構造体の定義、構造体としての名前をつけるがプロパティの名前はつけない時に使う
struct Point(i32, i32, i32); // Colorと同じシグニチャだけど違う構造体、明確に区別する以外の利点は少なそう

#[derive(Debug)]
enum IpAddrKind { V4, V6 }
enum IpAddr {
    V4(String),
    V6(String),
} // enumにStringを紐づけて定義する。あえてIpAddrKindとStringの構造体を作る必要はない。

enum Coin { Penny, Nickel, Dime, Quarter }

fn value_in_cents(coin: Coin) -> u32 {
    match coin { // パターンマッチング
        Coin::Penny => {
            println!("Lucky penny!");
            1 // ブロックを使える。
        },
        Coin::Nickel => 5, //値を返すだけならブロックはいらない
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> { // nullなら何もせず数値なら1を足すといったこともできる。
    match x {
        Some(i) => Some(i + 1),
        None => None, // もしこのパターンを書き忘れたらコンパイルエラー、パターンマッチングは包括的である必要がある
    }
}


fn instantiate(){
    let mut user = User {// Userのミュータブルなインスタンスを作る
        email: String::from("someone@example.com"), username: String::from("someusername123"),
        active: true, sign_in_count: 1,
    };
    println!("{}", user.email);
    println!("{}", user.username);
    println!("{}", user.sign_in_count);
    println!("{}", user.active);
    user.active=false; // mutなstructのプロパティは変更できる 特定のプロパティのみをmutとする記法はない。
    println!("{}", user.active);
}


fn build_user_by_omitted_syntax(email: String, username: String) -> User {
    User {
        email, // 便利な省略記法、email:emailと同じ、es6みたいだ
        username, // username:usernameと同じ
        active: true, sign_in_count: 1,
    }
}

fn instantiate_with_other_struct() {
    let user1 = build_user_by_omitted_syntax(String::from("someone@example.com"), String::from("someusername123"));
    let user2 = User { //user1のデータを元に新しい構造体を作る
        username: String::from("anotherusername567"),
        ..user1 // 指定しなかったプロパティはuser1のプロパティを引き継ぐ
    };
    println!("{}", user2.email); // user1と同じ
}


fn main() {
    instantiate();
    instantiate_with_other_struct();
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    let _like_unit = LikeUnit{};
    let user1 = build_user_by_omitted_syntax(String::from("someone@example.com"), String::from("someusername123"));
    println!("{:?}", user1); // std::fmt::Debugによるprint出力
    println!("{:#?}", user1); // std::fmt::Debugによるprettyなprint出力
    println!("{}", user1.double_count()); // メソッド呼び出し
    println!("{:#?}", User::test_account()); // 関連関数の呼び出し

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}", four); // enumの出力
    let home = IpAddr::V4(String::from("127.0.0.1")); // enumに特定の値を紐づける場合
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5); //
    let some_string = Option::Some("a string"); // SomeはOption型のenum、この場合型はOpsion<&str>と推論できる
    let absent_number: Option<i32> = None; // NoneはOption型のenum、NoneではOptionのジェネリクス型を推論できないので型注釈が必要

    let x = 0u8;
    match x {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _は1,3,5,7以外の全てのパターン、いわゆるelseでありRustでは包括値と呼ぶ
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    } // Some(3)の時のみ何かしたい時でもパターンを包括する必要がある

    if let Some(3) = some_u8_value { println!("three"); } // if letで略記できる。
}
