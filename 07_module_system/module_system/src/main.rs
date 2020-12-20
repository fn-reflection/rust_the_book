mod front_of_house { // front_of_houseはpubがなくても兄弟のeat_at_restaurant関数から見える(モジュールレベルは同じ)
    pub mod hosting { // front_of_houseモジュール外部に公開するためにpubをつける
        pub fn add_to_waitlist() {println!("add waitlist")} // hostingモジュール外部に公開するためにpubをつける
        fn seat_at_table() {}
        pub mod sub_hosting {
            pub fn add_to_waitlist2() {super::add_to_waitlist()} // superで上位モジュールの要素にアクセスできる
        }
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // 公開されてないのでプロパティにアクセスできない
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer { // enumはpubをここにつけておけば全部に要素にアクセスできる
        Soup,
        Salad,
    }
}

fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();// 絶対パス参照
    front_of_house::hosting::add_to_waitlist(); //相対パス参照
    crate::front_of_house::hosting::sub_hosting::add_to_waitlist2();
}

fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); エラー、seasonal_fruitは公開されていない
    // println!("{}", meal.seasonal_fruit);  //エラー、seasonal_fruitは公開されていない
}

use crate::front_of_house::hosting; //hostingが直接参照できる、シンボリックリンクと似ている。
// pub use crate::front_of_house::hosting; //useしたものを公開する、デフォルトではuseしたものは非公開
use crate::front_of_house::hosting::add_to_waitlist; // 関数を直接的にuseするのは可能だが、Rustの慣習には沿わない。上位モジュールを参照する。
use std::collections::HashMap; // 構造体やenumなど関数以外をuseするときは直接的にuseするのが慣習
use std::fmt::Result;
use std::io::Result as IoResult; // 名前がかぶる場合はエイリアスをつけて回避できる。
use std::cmp::{Ordering, Reverse}; // useはnestできる。
use std::io::{self, Write}; // selfを使う例 use std::io; use std::io::Write;と同じ
use std::collections::*; // glob演算子
mod module_a; // modの後にブロックじゃなく;とするとmodの実装をhostingというファイルから探す、__init__.pyと似ていると考える
use module_a::sub_module_b; // 
fn main() {
    eat_at_restaurant();
    eat_at_restaurant2();
    add_to_waitlist();
    hosting::add_to_waitlist();
    let mut map = HashMap::new(); // HashMapを参照する
    map.insert(1, 2);
    println!("{:?}", map);
    let mut map = std::collections::HashMap::new(); // stdをuseしなくても使えはする。
    map.insert(1, 2);
    println!("{:?}", map);
    println!("{}", sub_module_b::func_c(1,2));
}
