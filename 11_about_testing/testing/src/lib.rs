#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool { self.width > other.width && self.height > other.height }
}

pub struct Guess { value: i32 }
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 { panic!("Guess value must be greater than or equal to 1, got {}.", value); } 
        else if value > 100 { panic!("Guess value must be less than or equal to 100, got {}.", value); 
    }
    Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 { internal_adder(a, 2) }
fn internal_adder(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)] // cargo testを呼んだ時にのみコンパイルする、build時には無視される
mod tests { // Rustは慣習的にテストを実装と同じファイルに入れるらしい
    use super::*; // 実装をテスト関数内でも参照できるようにする

    #[test]
    fn exploration() { assert_eq!(2 + 2, 4); } // assert_eq!は同じならOK

    #[test]
    #[should_panic] // panicするとOK
    fn another() { panic!("Make this test fail"); }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")] // panicした時のエラーメッセージがこれであればOK
    fn greater_than_100() { Guess::new(200); }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller)); // assert!はtrueならOK
    }

    #[test]
    fn it_works() -> Result<(), String> { // Result型を返す関数として書いてもOK 例えばテスト関数内で?を使える
        if 2 + 2 == 4 { Ok(()) } 
        else { Err(String::from("two plus two does not equal four")) }
    }

    #[test]
    fn test_private_func() {
        assert_eq!(internal_adder(2, 2),4); // これならprivate scopeの関数も問題なくテストできる
    }
}
