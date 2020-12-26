pub trait Draw { fn draw(&self); }

pub struct Screen {
    // DrawだけではStructのサイズが不明なのでBoxポインタを挟む必要がある
    pub components: Vec<Box<Draw>>, // Box<Draw>がトレイトオブジェクト(トレイトのポインタ)
    // 型パラメータTと違いDrawトレイトを実装していれば異なる型を含んでも良い。
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() { component.draw(); }
    }
}

// こう定義すると、Drawを実装した特定の具体型1つしか含むことができない(パラメータ多相ではなくコンパイル時に単相化される)
pub struct TypedScreen<T: Draw> { pub components: Vec<T> } 
impl<T> TypedScreen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() { component.draw();}
    }
}

pub struct Button { pub width: u32, pub height: u32, pub label: String }
impl Draw for Button { fn draw(&self) { println!("button"); } }
struct SelectBox { width: u32, height: u32, options: Vec<String> }
impl Draw for SelectBox { fn draw(&self) { println!("selectbox"); } }

pub struct DraftPost { content: String } // 下手にStateパターン使うよりこっちの方がはるかにメリットがある。
impl DraftPost {
    pub fn add_text(&mut self, text: &str) { self.content.push_str(text); }
    pub fn request_review(self) -> PendingReviewPost { PendingReviewPost { content: self.content } }
}

pub struct PendingReviewPost { content: String }
impl PendingReviewPost {
    pub fn approve(self) -> Post { Post { content: self.content } }
}

pub struct Post { content: String }
impl Post {
    pub fn new() -> DraftPost { DraftPost { content: String::new() } }
    pub fn content(&self) -> &str { &self.content }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("Yes"),String::from("Maybe"),String::from("No")]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();

    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    println!("{}", post.content());
}