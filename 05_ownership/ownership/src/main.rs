// stack領域は固定サイズなデータに向いていて高速、ポインタを必要としない、ローカリティが高い
// heap領域は可変サイズなデータに向いている。OSにheap領域を動的に要求する、stackに比べると低速でローカリティが低い
// 所有権という概念はheapデータを管理するのに意味がある
// 値は所有者と呼ばれる変数がもつ。いかなる時も所有者は一つの変数であり、所有者(変数)がスコープから外れた時に値は破棄される。

fn about_scope() {
    // sは有効ではない、まだ宣言されていない
    let s = "s have this"; // sは有効、 "s have this"はハードコードされた不変値(internされた文字列という奴か、スタック領域にある？)
    println!("{}", s); //sを参照できる。
} // 関数の終わり、sはスコープから外れる、"s have this"はスタックに確保されているのであればメモリ解放する意味がないのでされないかも？

fn heap_object() { // Stringはヒープに格納される。長さを動的に決められる。
    let mut s = String::from("hello"); // String型のスタティックメソッドfrom、OSにヒープを要求している。
    s.push_str("world"); // 末尾にworldを追加、Stringはヒープにあるので長さも含めて可変、メモリが不足する場合新たにヒープを要求するだろう。
    println!("{}", s); // 副作用が適用されていることを確認
} // sがスコープから外れる。所有者sを通じてヒープに確保されたメモリは返還される、Cで言う所のfreeをRustではdrop関数を呼ぶ

fn move_semantics() {
    let x = 5; // 5をスタックに用意し、xに束縛する。
    let y = x; // xは5なので5を新たにスタックに用意し、yに束縛する。2つの5ができる。
    // つまりスタック領域に確保される値の場合、コピーセマンティクスが適用される。
    // スタック領域に確保されているのでディープコピーもシャローコピーも同じなので区別する必要はない。
    // Javaで言う所のプリミティブ型
    println!("{}", y);   
    let s1 = String::from("hello"); // Stringは文字の実体へのポインタ・文字列長・既にメモリ確保した長さからなる
    let s2 = s1; // シャローコピーのセマンティクスで考えるとポインタ・文字列長・メモリ確保した長さをコピーするが、文字の実体をコピーするわけではない。ポインタはs1と同じ実体を指す。
    // s1とs2を通じて文字の実体を指せるので文字の実体の所有者はこのままではs1とs2の2つという事になる。
    println!("{}, world!", s2); //s2のみが呼び出せる。s1はs2にムーブされたため使用不可
    // println!("{}, world!", s1); エラー、Rustではlet s2=s1;とした時点でs1は利用できなくなる。C++で言う所のムーブセマンティクス
    // s1が指しているがヒープ(いわゆる参照型、あるいはオブジェクト型)
    // Rustは自動的にディープコピーをすることはない。
    let s3 = String::from("hello");
    let s4 = s3.clone(); // cloneでは文字の実体も含めてディープコピーする。s4はディープコピーされた文字の実体を所有しているし、s3がもつ文字の実体ともはや無関係。
    println!("s3 = {}, s4 = {}", s3, s4); // s3は文字の実体に対する所有権を放棄していないので、呼び出せる。
    // スタックに配置される型(リテラルとか)は、Copyトレイトに適合している。
    // ある型がその一部分でもDropトレイトを実装している場合、Copyトレイトによる注釈はできない。(型の値がスコープを外れた時に何かする必要がある場合Copy注釈を与えられない。)
    // タプル(i32, i32)はCopyトレイトを満たすが、タプル(i32, String)はStringがCopyトレイトを満たさないので、Copyトレイトを満たさない。
} 

fn passes_ownership() {
    let s1 = String::from("moved");  // sがStringを所有する
    takes_ownership(s1); // s1の値が関数呼び出しを通じてsome_stringにムーブされるためsは使えない。
    // println!("{}", s1); エラー、s1は関数呼び出しによって所有権が関数の仮引数にムーブされたので使えない。
    let s2 = String::from("moved_but_returned");  // s2がStringを所有する
    let s3 = print_and_return_ownership(s2);
    println!("{}", s3);// s3は関数からStringの所有権を返されたので所有権を持っている
} 

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_stringを所有しているのでここでdropされる、つまりヒープに確保されたStringのメモリが解放される、参照カウントがゼロになるとも言える

fn print_and_return_ownership(some_string: String)->String{
    println!("{}", some_string);
    some_string
} // some_stringをreturnしているので呼び出し元に所有権がムーブする

fn work_without_reference() { // 参照を使わないでプログラムを書くと？
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_without_ref(s1); // s1がムーブするので、このままでは使えない
    println!("The length of '{}' is {}.", s2, len); // 関数からムーブし直したのでこの呼び出しは正当
}

fn calculate_length_without_ref(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // ムーブしたsをムーブし直さないと呼び出し元でsを使えない。参照を使わなくてもプログラムはかけるが端的な記述にならない。
}

fn work_with_reference() {
    let s1 = String::from("hello");
    let len = calculate_length_with_ref(&s1); // s1を指す参照(&s1はs1を指しているポインタと考えればいい)
    // Stringヒープオブジェクトの所有権はs1にある、&s1は所有権を持たない
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_with_ref(s: &String) -> usize { // 参照を通じてsのデータにアクセスできる、sを借用しているが所有はしていない
    // s.push_str(", world"); &はイミュータブルな借用のためデータを勝手に変更できない。
    s.len()
} // sを所有していないためsはdropされない。呼び出し元でsは使用できる。


fn work_with_mut_reference() {
    let mut s1 = String::from("hello"); // s1自体がmutじゃないと文字列を変更できない。
    let s1_ref1 = &mut s1; // mutableな参照、もしもs1がイミュータブルならmutableではないので無意味です。
    // let s1_ref2 = &mut s1; エラー、同一スコープでmutableな参照を複数作ることはできない。これによりRustはデータ競合を防ぐ。
    // let s1_ref3 = & s1; // エラー、イミュータブルな参照は不変性を期待しているので、mutableな参照とは共存できない。
    let len = calculate_length_with_mut_ref(s1_ref1); 
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_with_mut_ref(s: &mut String) -> usize { // mutableな借用なのでsがmutなら変更できる。
    s.push_str(", world");
    s.len()
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//} エラー、sは関数スコープを抜けるとdropする、&sは意味あるものをささないダングリングポインタになるためコンパイルエラーになる。

fn slicing() {
    let s = String::from("hello world");
    //スライスは参照先の最初の要素への参照と参照する長さをもつイミュータブルな参照だと考えられる
    let hello:&str = &s[..5]; // 文字列のslice &str型　[0..5]と同じ
    let world:&str = &s[6..]; // 省略できる [6..s.len()]と同じ
    println!("hello: {} ,world: {}.", hello, world);
    let word1 = first_word(&s[..]); // スライスを渡す
    println!("first_word is {}.", word1);
    let literal:&str = "hello world"; // 文字列リテラルは(バイナリの)スライスである
    let word2 = first_word(&literal); // first_wordは&strにも使える
    println!("first_word2 is {}.", word2);
    let arr = [1,2,3,4,5];
    let _slice_arr:&[i32] = &arr[2..]; // 配列もスライスできる

}

fn first_word(s: &str) -> &str { // 引数を&strにすると&strとStringのスライス両方で使える
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }
    &s[..] // [0..s.len()]と同じ
}


fn main() {
    about_scope();
    heap_object();
    move_semantics();
    passes_ownership();
    work_without_reference();
    work_with_reference();
    work_with_mut_reference();
    slicing();
}
