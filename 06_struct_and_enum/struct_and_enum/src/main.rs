#[derive(Debug)] // Rust標準のデバッグ用出力実装でstd::fmt::Debugトレイトを実装することを宣言する
                 // これをしないとDebug出力すらできない。Rustは構造体のデフォルトの出力形式を規定しない
                 // std::fmt::Displayではヒューマンリーダブルな出力を実装すべきである、pythonで言う所の__str__だ
struct User { // 構造体の定義
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

struct LikeUnit { // ユニット様の構造体の定義、トレイトの実装などに使うらしい。型自体はデータを持たない。
}

struct Color(i32, i32, i32); // タプル構造体の定義、構造体としての名前をつけるがプロパティの名前はつけない時に使う
struct Point(i32, i32, i32); // Colorと同じシグニチャだけど違う構造体、明確に区別する以外の利点は少なそう


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
}
