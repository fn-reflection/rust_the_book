fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


enum Message2 {
    Hello { id: i32 },
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let point = (3, 5);
    print_coordinates(&point);

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // ここで引っかかる、このyはmatchスコープ内の自由変数で上のyではない
        //Some(z) => println!("Matched, y = {:?}", z), // これと同じ
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"), // OR記法
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"), // 1, 2, 3, 4, 5
        // 1...5 => println!("one through five"), // この記法はdeprecated
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        // charまたはint
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // こういう分配束縛もできる
    let Point { x, y } = p; // 省略記法
    println!("{},{}", a, b);
    println!("{},{}", x, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
    let _x = 5; // 未使用変数だが、コンパイラは_から始まった変数を無視する

    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    // println!("{:?}", s); // エラー if letにsの所有権が取られる _sは有効である

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        // _ の場合値を束縛しないのでsの所有権はムーブしない
        println!("found a string");
    }
    println!("{:?}", s); // 呼べる
    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            // 真ん中をすっ飛ばす
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(name) => println!("Found a name: {}", name),
        None => (),
    }
    // println!("robot_name is: {:?}", robot_name); // エラー 値の所有権がname変数にムーブしている

    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name), // refを宣言するとムーブしない
        None => (),
    } // &を使いたいところだが、match式は&にもmatchしてしまうのでrefを宣言して参照を生成する必要がある
    println!("robot_name is: {:?}", robot_name);

    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"), // ref mutで可変参照を生成
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), // 追加の条件式をいれられる
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n), // 条件式内では外部変数yを評価できる
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), // x=4,5,6かつy=trueなので、y=falseだからマッチしない
        _ => println!("no"),
    }



let msg = Message2::Hello { id: 5 };

match msg {
    Message2::Hello { id: id_var @ 3..=7 } => {
        println!("Found an id in range: {}", id_var) // id_varに束縛した値を使用できる
    },
    Message2::Hello { id: 10..=12 } => {
        println!("Found an id in another range") // ここでidを使うことはできない
    },
    Message2::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
}
