// 型パラメータを持つジェネリクスメソッドはコンパイル時に型が解決される(単相化)されるため
// 型パラメータを使用しないメソッドと同等の実行速度を持つ。

fn largest_i32(list: &[i32]) -> i32 { // i32のパターン
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest { largest = item; }
    }
    largest
}

fn largest_char(list: &[char]) -> char { // charのパターン
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest { largest = item; }
    }
    largest
}
// Tは std::cmp::PartialOrd + Copyという2つのトレイト境界を実装していなければならない。
fn largest1<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T { // Tという型パラメータを使用し、Tのスライス型のlistを引数としTを返す関数
    let mut largest = list[0]; // Copyトレイトがないとlistの所有権がムーブしてしまう。
    for &item in list.iter() {
        if item > largest { largest = item; } // 比較可能であるという型制約が必要になる
    }
    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { // Tという型パラメータを使用し、Tのスライス型のlistを引数としTを返す関数
    let mut arg_largest = 0; // copyトレイト不要
    for (i,item) in list.iter().enumerate() {
        if item > &list[arg_largest] { arg_largest = i; } // 比較可能であるという型制約が必要になる
    }
    &list[arg_largest]
}

//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

//    fn some_function<T, U>(t: &T, u: &U) -> i32
//    where T: Display + Clone, // where句も糖衣構文
//          U: Clone + Debug
//{

// fn returns_summarizable() -> impl Summary { Summaryを実装する型を返す関数
// ifによりNewsArticleかTweetのどっちかを返すといった関数はこれだけでは書けない

struct Point<T> { x: T, y: T } // 任意の型に対応した構造体

impl<T> Point<T> { // ジェネリクスメソッドの実装
    fn get_x(&self) -> &T { &self.x }
}

impl Point<f32> { // 特定の型に対するメソッドを定義
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub trait Summary { // Summaryトレイトを宣言する、いわゆるインタフェースに似ている
    //Summaryトレイトを保持する型はsummerizeメソッドを実装せねばならない。
    // fn summarize(&self) -> String; デフォルト実装を与えない場合
    fn summarize(&self) -> String { //トレイトはデフォルト実装を持てる
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {} // デフォルト実装を採用する

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet { //トレイトの実装
    fn summarize(&self) -> String { format!("{}: {}", self.username, self.content) }
}

pub fn notify(item: &impl Summary) { // Summaryを実装している型をもつitemを引数とする関数
    println!("Breaking news! {}", item.summarize());
}
pub fn notify2<T: Summary>(item: &T) { // 上記のnotifyのimplはこの文の構文糖衣。(こっちの方がしっくりくるかも)
    println!("Breaking news! {}", item.summarize());
}

use std::fmt::Display;

struct Pair<T> { x: T, y: T }
impl<T> Pair<T> { // 任意のTについてPair<T>のnewを実装する
    fn new(x: T, y: T) -> Self { Self { x, y } }
}

impl<T: Display + PartialOrd> Pair<T> { // Display + PartialOrdというトレイト境界を満たすTについてcmp_displayを実装する
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


fn lifetime() {
    let r = 1;
    {
        let _x = 5;
        //r = &_x; エラー、rはxを参照するが
    } // x(&xの実体)はここでdropするので、rはダングリングポインタになってしまう。
    println!("r: {}", r);
}

// fn longest(x: &str, y: &str) -> &str { // エラー、xとyどちらのライフタイムを参照すればいいか確定しない。
//    if x.len() > y.len() { x } else { y }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { 
    //ライフタイム'aを宣言し、x,y,返り値が同じライフタイム'aを持つことを注釈する。
    // 'aはxとyのうち短い方のライフタイムになる。
    // 返り値はライフタイム'aと同じだけ生きる
    // ライフタイムは関数外からの参照、関数外への参照があるとき解決困難になる
    // 呼び出しごとに参照のライフタイムは異なる可能性がある。
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_word(s: &str) -> &str { // この関数はライフタイム省略規則を満たしているので書かなくて良くなった
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }
    &s[..]
}
// ライフタイム省略規則
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32) n引数の場合nつの入力ライフタイムを得る
// fn foo<'a>(x: &'a i32) -> &'a i32 入力ライフタイムが1つならそれが出力ライフタイム引数の全てに代入される
// メソッドで引数が&selfや&mut selfだったら、 selfのライフタイムが全出力ライフタイム引数に代入される

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str, //所有権を持たない参照はライフタイムを決める必要がある。
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { // この関数は明らかにライフタイムを省略できる &selfと同じ
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str { // 入力ライフタイムは'aと'b 出力ライフタイムは'aになる
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str // 全部のせパターン、型もライフタイムもジェネリクスなので<>に一緒に入れる。
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let result = largest1(&number_list);
    println!("The largest char is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let result = largest(&number_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let x = integer.get_x();
    println!("{}",x);    
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 }; これは型Tがi32だったりf64だったりするのでエラー
    println!("{}",float.distance_from_origin());
    // println!("{}",integer.distance_from_origin()); エラーinteger型の場合にはメソッド定義されていない。
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false, retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize()); // summarizeは実装したので当然呼び出せる。
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
    lifetime();
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");  // '.'が見つかりませんでした
    let i = ImportantExcerpt { part: first_sentence };
    println!("{:?}",i);
    let s: &'static str = "I have a static lifetime."; // リテラルはstaticな、つまりプログラムの生存期間と同じライフタイムを持つ
    println!("{}",s);
}