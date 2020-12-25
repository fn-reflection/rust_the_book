#[derive(Debug)]
enum ListI32 { Cons(i32, Box<ListI32>), Nil } //本で紹介されたi32向けのLinkedList

#[derive(Debug)]
enum List<T> { Cons(T, Box<List<T>>), Nil } // ジェネリクスタイプのLinkedList

struct MyBox<T>(T); // Boxは1要素のタプルとみなせる
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { MyBox(x) }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

struct CustomSmartPointer { data: String }

impl Drop for CustomSmartPointer {
    // dropの実装、変数がスコープから抜けた時に発動する
    fn drop(&mut self) { println!("Dropping CustomSmartPointer with data `{}`!", self.data); }
}

use std::ops::Deref;
fn hello(name: &str) { println!("Hello, {}!", name); }

fn deref() {
    let x = 5;
    let y = &x; // ref operator
    assert_eq!(5, x);
    assert_eq!(5, *y); // deref operator
    //assert_eq!(5, y); //コンパイルエラー、自動的にderefされない
    let y = Box::new(x);
    assert_eq!(5, *y); // BoxはDerefトレイトを実装している、Derefがあることで&参照以外の参照剥がしができる
    let y = MyBox::new(x);
    assert_eq!(5, *y); // Derefを実装することでderefできるようになる、*(y.deref())
    let m = MyBox::new("Rust"); // 文字列スライスが引数の型なので当然動く
    hello(&m);
    let m = &String::from("Rust"); // 参照外し型強制により&String -> &strに強制される
    hello(&m);
    let m = MyBox::new(String::from("Rust")); // Derefトレイト実装により&MyBox<String> -> &Stringに解釈でき&String -> &strに強制される
    hello(&m);
    let m = MyBox::new(String::from("Rust")); // Derefトレイト実装により&MyBox<String> -> &Stringに解釈でき&String -> &strに強制される
    hello(&(*m)[..]); // mを参照剥がししてStringを出してそれの文字列スライスの参照なのでこれも動くけどこうは書かなくて済む

    //T: Deref<Target=U>の時、&Tから&U
    // T: DerefMut<Target=U>の時、&mut Tから&mut U
    // T: Deref<Target=U>の時、&mut Tから&U

}

fn drop_trait(){
    let c = CustomSmartPointer { data: String::from("one") };
    let d = CustomSmartPointer { data: String::from("two") };
    println!("exit drop_trait"); 
} // d.drop(); c.drop();

fn force_drop(){
    let c = CustomSmartPointer { data: String::from("one") };
    let d = CustomSmartPointer { data: String::from("two") };
    println!("CustomSmartPointers created."); 
    // c.drop(); // エラーデストラクタの明示的呼び出しはできない、このままでは二重のメモリ解放になる
    drop(c); // これはOK std::mem::drop
    println!("exit force_drop"); 
}


#[derive(Debug)]
enum RcList<T> { Cons(T, Rc<RcList<T>>), Nil } // ジェネリクスタイプのLinkedList
use std::rc::Rc;

fn reference_counting(){ //Rc型はマルチスレッドでは使用できない。不変参照のみ認める。 マルチスレッドではArcを使用する
    let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    let b = List::Cons(3, Box::new(a));
    // let c = List::Cons(4, Box::new(a)); // エラー、aがムーブしてしまうので
    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    let b = RcList::Cons(3, Rc::clone(&a)); // a.clone()でもいいが、Rcを使うのが慣習、参照カウントを増やしていることを明示する
    let c = RcList::Cons(4, Rc::clone(&a));
    println!("{:?}",b);
    println!("{:?}",c);
}

fn reference_counting2() {
    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // strong_countの他にweak_countもある。
    let b = RcList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcList::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    drop(b);
    println!("count after force dropping b = {}", Rc::strong_count(&a));
}
fn ref_cell(){ //RefCell型はマルチスレッドでは使用できない。
    let x = 5;
    // let y = &mut x; //エラー、不変値を可変で借用できない。

}

// Rc<T>は、同じデータに複数の所有者を持たせる; 
// Box<T>とRefCell<T>は単独の所有者。
// Box<T>では、不変借用も可変借用もコンパイル時に精査できる
// Rc<T>では不変借用のみがコンパイル時に精査できる; RefCell<T>では、不変借用も可変借用も実行時に精査される。
// RefCell<T>は実行時に精査される可変借用を許可するので、RefCell<T>が不変でも、 RefCell<T>内の値を可変化できる。

#[derive(Debug)]
enum ListMut { Cons(Rc<RefCell<i32>>, Rc<ListMut>), Nil,}
use std::cell::RefCell;

fn share_data_with_refcell() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(ListMut::Cons(Rc::clone(&value), Rc::new(ListMut::Nil)));
    let b = ListMut::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = ListMut::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}


#[derive(Debug)]
enum CirculatedList { Cons(i32, RefCell<Rc<CirculatedList>>), Nil }

impl CirculatedList {
    fn rest(&self) -> Option<&RefCell<Rc<CirculatedList>>> { // 最初の要素を除いた全て、lispで言う所のcdr
        match *self {
            CirculatedList::Cons(_, ref item) => Some(item),
            CirculatedList::Nil => None,
        }
    }
}

fn circular() {
    let a = Rc::new(CirculatedList::Cons(5, RefCell::new(Rc::new(CirculatedList::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.rest());

    let b = Rc::new(CirculatedList::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.rest());

    if let Some(a_rest) = a.rest() { *a_rest.borrow_mut() = Rc::clone(&b); } // aのcdrをbへの参照に変更するa -> b -> a ...
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // println!("{:?}", a); // infinite loop
}

use std::rc::Weak;
#[derive(Debug)]
struct Node { 
    value: i32, 
    parent: RefCell<Weak<Node>>, // parentは循環参照を防ぐためWeakRefになる。
    children: RefCell<Vec<Rc<Node>>>  // childrenは共有可能なNodeのVectorをRefCell参照したもの
}


fn weak_ref(){
    let leaf = Rc::new(Node { value: 3, parent: RefCell::new(Weak::new()), children: RefCell::new(vec![]) });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); 
    let branch = Rc::new(Node { value: 5, parent: RefCell::new(Weak::new()), children: RefCell::new(vec![Rc::clone(&leaf)]) });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("{:?}", branch);
    drop(branch); // ブランチをdropすると
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // branchは消えたのでNone
}


fn main() {
    let b = Box::new(5); // Javaで言う所のボクシングでヒープにデータに保存する
    println!("b = {}", b);
    let list = ListI32::Cons(1, Box::new(ListI32::Cons(2, Box::new(ListI32::Cons(3, Box::new(ListI32::Nil))))));
    println!("b = {:?}", list);
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("b = {:?}", list);
    let list = List::Cons(1.5, Box::new(List::Cons(2.5, Box::new(List::Cons(3.5, Box::new(List::Nil))))));
    println!("b = {:?}", list);
    deref();
    drop_trait();
    force_drop();
    reference_counting();
    reference_counting2();
    share_data_with_refcell();
    circular();
    weak_ref();
}
