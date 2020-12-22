use std::thread;
use std::time::Duration;

struct Cacher<T:Fn(u32) -> u32>{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> { Cacher { calculation, value: None } }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}
fn capturing() {
    let x = 4;
    let equal_to_x = |z| z == x; // クロージャは外部スコープのxをキャプチャできる Fn Fnmut FnOnce
    // fn equal_to_x(z: i32) -> bool { z == x } // エラー。fnはキャプチャできない
    let y = 4;
    assert!(equal_to_x(y));
}

fn force_move() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x; // ムーブセマンティクスの強制
    // println!("can't use x here: {:?}", x); // エラー、xはzにムーブした
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn iterate() {
    let v1 = vec![1, 2, 3];
    for val in v1.iter() { //不変借用でイテレーション、v1.iter()自体の所有権はforが奪うので内部で可変にしている
        println!("Got: {}", val);
    }
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // mapは遅延評価、collectでイテレータがコレクションに変換される。
    assert_eq!(v2, vec![2, 3, 4]);
    println!("{:?}",v2);

    let mut v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter_mut().map(|x| {*x=*x+1; *x}).collect(); // こう書くこともできるが、
    assert_eq!(v2, vec![2, 3, 4]);
    println!("{:?}",v2);
    println!("{:?}",v1); // v1は副作用を受ける、ただこう書くことのメリットは無いと思われる。
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect() // into_iterは所有権を奪ってイテレーションする
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // sumはv1_iterの参照を引数に取るメソッドではない、v1_iterの所有権を奪いイテレータを消費する
    // v1_iterはもう呼び出せない。
    assert_eq!(total, 6);
}


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); // nextを自分で呼ぶ場合はmutが必要()
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

struct Counter { count: u32 } // 独自型の実装

impl Counter {
    fn new() -> Counter { Counter { count: 0 } }
}

impl Iterator for Counter { // 独自型のIteratorトレイトを実装する方法
    type Item = u32; // Item型は何であるか(今回はu32)を指定する
    fn next(&mut self) -> Option<Self::Item> { // nextを実装する
        self.count += 1;
        if self.count < 6 { Some(self.count) } else { None }
    }
}


#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}



fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
    capturing();
    force_move();
    iterate();
    let counter = Counter::new();
    for e in counter { // counterはイテレータである
        println!("{}",e); 
    }
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    println!("{}",sum); 
}