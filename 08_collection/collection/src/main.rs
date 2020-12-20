#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_type() {
    let v: Vec<i32> = Vec::new(); // ベクターはジェネリクス型である
    let v2 = vec![1, 2, 3]; // vec!マクロを使えば型注釈不要、右辺値からi32を推論する
    println!("{:?}",v);
    println!("{:?}",v2);

    let mut v3 = Vec::new(); // ベクタの旨みを出すためには普通mutで定義するはず
    v3.push(5); // この文からv3の型を推論できてしまう。
    v3.push(6);
    println!("{:?}",v3);

    let v = vec![1, 2, 3, 4, 5];
//  let does_not_exist = &v[100]; // ndex out of bounds、panicになる。
    let does_not_exist = v.get(100); // getはOption型を返すのでエラーにならない。
    println!("{:?}", does_not_exist); // Option::None
    
    let mut v4 = vec![1, 2, 3, 4, 5];
    let first = &v4[0];
    // v4.push(6); // firstで不変借用しているので変更できない。
    // 仮にv4.push(6)によってv4のメモリ位置がリアロケートされたとするとfirstは無効なメモリ位置を参照しかねない。
    println!("The first element is: {}", first);

    let v5 = vec![100, 32, 57];
    for i in &v5 { println!("{}", i); } // 不変な参照でループ、&をつけないと所有権をforに持っていかれる。
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 { *i += 50;} // iは参照(ポインタ)なので、参照はがしが必要、不変参照とは微妙に振る舞い違うので注意
    println!("{:?}", v6); 

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]; // enumを使うことで色々な型をまとめて放り込める。
    println!("{:?}", row); 
    // このままだと実行時にプログラマーが全ての型を知らないとうまくいかない。
    // スキーマが規定されていないJSONからデータをconstructするシチュエーションなどはそうだ。
    // トレイトオブジェクトを使用することで対応できる。
}

fn string_types(){
    // &str: 所有権を持たない、プログラムバイナリに格納される、伸縮不可能な不変なデータ
    // String: 所有権を持つ、ヒープに格納される、伸縮可能な可変なデータ
    // OsString、OsStr、CString、CStrなどもある。
    let mut s1 = String::new(); // Stringの作成
    println!("{}", s1);
    let mut s2 = "initial contents".to_string();
    // let mut s2 = String::from("initial contents"); こちらでも同じ
    s2.push('a'); // pushだとchar型
    let bar ="bar";
    s2.push_str(bar); // push_strだと&str型を使える
    println!("{}", s2);
    println!("{}", bar); // barは&str型なのでpush_strでも所有権が奪われていないので使える

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // add(self, s: &str)なので&をつけないと型に合わない、s1の所有権はs3にムーブされる
      // &s3は&Stringだが&strに型強制されるので動く。参照外し型強制
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // こうもかけるが
    println!("{}", s);

    let s1 = String::from("tic"); 
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // こう書いた方が直感的
    println!("{}", s);
    println!("{}", s1); // format!ならs1の所有権が奪われることはない。
    println!("{}", String::from("あいうえお").len()); // 15 lenはバイト値を返す。

    let s1 = String::from("あいうえお");
    // let h = s1[0]; エラー、文字列型はUTF-8のバイトバッファであり、インデックス0の値はおそらくユーザーが求めるものでない。
    let s = &s1[0..3];
    println!("{}", s); // あが出力される。 もし0..2とかするとrustはpanicを起こしてしまう。
    for c in s1[..].chars() { println!("{}", c); } // ユニコード文字のリストとして扱える。
    println!("{:?}", s1[..].chars().last()); // Some('お')が出力される。
    for b in "नमस्ते".bytes() { println!("{}", b);} // バイト列とみなした時の数値が出力される。
}

use std::collections::HashMap;

fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("{:?}", scores);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect(); // collectはイテレータをコレクションに変換する。
    println!("{:?}", teams); // .iter()はteamsを借用するだけなので引き続き参照できる．
    println!("{:?}", initial_scores);
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // ベクタの型を元にある程度型推論できる。
    println!("{:?}", scores);
    let score = scores.get(&String::from("Blue"));
    println!("{:?}", score); // getは失敗しうるのでOption型になる
    let score = scores[&String::from("Blue")];
    println!("{:?}", score); // 失敗するとpanic
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // Stringの所有権はmapにムーブする


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 上書きする。
    println!("{:?}", scores);   

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // ないので50をYellowに紐づける
    scores.entry(String::from("Blue")).or_insert(50); // あるのでBlueには何もしない。
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // wordにあればその値を可変参照(&mut V)として返す、無ければ0を登録して可変参照として返す。
        *count += 1;
    }
    println!("{:?}", map);
}


fn main() {
    vector_type();
    string_types();
    hash_maps(); // Hashmapのhash関数はセキュリティ耐性があるが遅いので書き換えても良い。
}