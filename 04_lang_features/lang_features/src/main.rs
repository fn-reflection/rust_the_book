fn mutable_variable() {
    let mut x = 5; // mutable変数はlet mutで宣言する
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn constant() {
    const PI:f64 = 3.1415926535; // constは型注釈必須
    const TWO_POWERED_BY_16: u32 = 65_536; // 数値は_で区切ることができる
    println!("The value of PI is: {}", PI);
    println!("The value of 2**16 is: {}", TWO_POWERED_BY_16);
}

fn shadowing() {
    let x = 5;
    let x = x + 1; //右辺のxは上で束縛されたx、この文により上のxは隠(シャドーイング)される。
    let x = x * 2; //右辺値が毎回新規で作られ、識別子xで参照できるようになると考えたほうがいい。
    let spaces = "   "; // String型変数spaces
    let spaces = spaces.len(); // 右辺値はspacesの長さ、つまり3したがってspacesの型は数値
    println!("The value of (5+1)*2 is: {}", x);
    println!("The space length is: {}", spaces);
}

fn tuple() {
    let tup = (500, 6.4, 1); // tupleは型注釈なしでも宣言できる
    let (x, y, z) = tup; // 分配束縛もできる
    // println!("The tup is: {}", tup); これはできないらしい。要検討
    println!("The x is: {}", x);
    println!("The y is: {}", y);
    println!("The z is: {}", z);
    println!("The x is: {}", tup.0); // tup.0で最初の要素にアクセス
}

fn array() {
    let arr = [1,2,3,4,5]; // 配列は同一型からなり固定長で伸縮不可、スタック領域にメモリ確保する、柔軟な奴はベクタ型
    // let arr = [1,2.5]; // これは不可
    // println!("The arr is: {}", arr); これは不可
    println!("The arr[0] is: {}", arr[0]);
}

fn double(x:i32)->i32 { // 関数シグニチャは型注釈必須
    return 2*x;
}

fn expression()->i32 { 
    let y = {
        let x = 3;
        x + 1 // x+1は式なので;をつけない。;をつけるのは文で副作用を示し値を返さない。
    }; // {}ブロックは式を返すのでこう書ける。
    y // returnは実は必要ない、rubyと同じく最後に書いた式が返される
}

fn expression_scope()->i32 { 
    let x = 5;
    let y = {
        let z = 1;
        x + 1
    }; // 外部スコープからxを引っ張ってこれる。
    // println!("The z is: {}", z); エラー、zは内部スコープに閉じているので呼び出せない
    return y;
}

fn if_expression(x:i32)->&'static str { 
    let res = if x>5 { // ifは式なので結果を代入可能、rubyと同じ
        "greater than 5" // 条件式に紐づいたこのブロックのことをアームと呼ぶことがある
    } else if x==5 { // ちなみに条件式はbool型でなければならない
        "equal 5"
    } else {
        "less than 5" // if式が返す型は基本的に揃える、揃えない場合は型に工夫が必要そう
    };
    res 
}

fn iterations() { 
    let arr = [10, 20, 30, 40, 50];

    for elem in arr.iter() { // イテレータによるforループ
        println!("the value is: {}", elem);
    }
    
    for number in (1..4).rev() { // rangeオブジェクト、reverse
        println!("{}!", number);
    }

    let mut y = 5;
    while y > 0 { // 条件を満たさなくなるまでループ
        println!("in the while");
        y -= 1;
    }

    let mut x = 5;
    loop { // breakするまで無限ループ
        if x<=0 { break; }
        println!("in the loop");
        x -= 1;
    }
}



fn main() {
    mutable_variable();
    constant();
    shadowing();
    tuple();
    array();
    println!("The double(2) is: {}", double(2));
    println!("The expression() is: {}", expression());
    println!("The expression_scope() is: {}", expression_scope());
    println!("The if_expression(5) is: {}", if_expression(5));
    println!("The if_expression(10) is: {}", if_expression(10));
    iterations();
}