// unsafe superpower (この4つの機能の使用はコンパイラによって捕捉されない)
// 生ポインタを参照外しすること
// unsafeな関数やメソッドを呼ぶこと
// 可変で静的な変数にアクセスしたり変更すること
// unsafeなトレイトを実装すること
use std::slice;

fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    // (&mut slice[..mid], &mut slice[mid..]) エラー sliceの別の部分をそれぞれ可変借用しているが、sliceを2回借用しているとコンパイラはみなしてしまう
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle] // 関数名をmangleさせない必要がある
pub extern "C" fn call_from_c() {
    // Cから呼べるようにする、共有ライブラリにコンパイルしリンクする
    println!("Just called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world!"; // グローバル変数 staticを使う
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub trait Iterator {
    type Item; // ジェネリクスではなく関連型を使うのは、型注釈をしないため
    fn next(&mut self) -> Option<Self::Item>;
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters { // RHSにSelf(Millimeters)とは違う型を適用する場合
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}


trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display { // Displayトレイトを実装した場合に使えるトレイト
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// impl fmt::Display for Vec<String> {} // エラーオーファンルールにより外部で規定されたかたのトレイトを実装できない
struct Wrapper(Vec<String>); // ニュータイプルールを使う、オーバーヘッドはない
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

type Kilometers = i32; // 型エイリアス
// type Result<T> = Result<T, std::io::Error>; std::ioにある便利なエイリアス



fn generic<T: ?Sized>(t: &T) { // ジェネリクス関数のトレイト境界には暗黙的にSizedが追加される、?Sizedで条件を緩和できるが、Tはコンパイル時にサイズが確定できないのでポインタを挟む必要がある
    // --snip--
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { // fnが関数ポインタ型
    f(arg) + f(arg)
}

// fn returns_closure() -> Fn(i32) -> i32 { |x| x + 1 } // エラー、クロージャを直接返そうにもクロージャのサイズが確定しない
fn returns_closure() -> Box<Fn(i32) -> i32> { // BoxでラップしたこれはOK
    Box::new(|x| x + 1)
}

#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32; // const生ポインタ
    let r2 = &mut num as *mut i32; // mut生ポインタ、ポインタの宣言自体はunsafeではない
                                   // ポインタでなく参照ならエラー、不変参照と可変参照は共存できない。
    let address = 0x01234544usize;
    let _dangling = address as *const i32; // ダングリングであろうconst生ポインタ

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("dangling is: {}", *_dangling); // segfault
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    println!("{:?}", a);
    println!("{:?}", b);
    let (a, b) = my_split_at_mut(r, 3);
    println!("{:?}", a);
    println!("{:?}", b);

    let address = 0x012345usize;
    let r = address as *mut i32;
    // let slice = unsafe { slice::from_raw_parts_mut(r, 10000) }; // segfault
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    } // 可変な静的変数のアクセスはunsafe
    println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 }); // +演算子のオーバーロード
    println!("{:?}", Millimeters(3) + Meters(1));
    let person = Human;
    Pilot::fly(&person); // Pilotトレイトに実装されたfly実装を呼ぶ
    Wizard::fly(&person); // Wizardトレイトに実装されたfly実装を呼ぶ
    person.fly(); // Humanに実装されたfly実装を呼ぶ
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // フルパス記法、関連関数をキャストして呼ぶ
    let p = Point { x: 1, y: 0 }; // +演算子のオーバーロード
    p.outline_print();
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w); // wrapperはラッパーでしかないのでVecのメソッドを使えない、Derefトレイトなどを実装してVecを返すなどする必要がある
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y); // Kilometersはi32と全く同じなので足し算可能
    loop {  // loopやcontinue、panic!などはnever(!)型を返す、never型は型強制される。
        print!("and ever ");
        break;
    }
    let answer = do_twice(add_one, 5); // 高階関数
    println!("The answer is: {}", answer);
    println!("The answer is: {:?}", vec2![1,2,3]);

    
}
