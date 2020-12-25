use std::thread;
use std::time::Duration;

fn hello_threading() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // spawned threadの終了を待たずにdropする
}

fn thread_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread_join {} from spawned", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("thread_join {} from main", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();  // handleしたthreadが終わるまでブロックする
}

fn move_value_to_spawned_thread(){
    let v = vec![1, 2, 3];
    // クロージャと同じく、別スレッドに所有権を渡すには、moveが必要
    let handle = thread::spawn(move || { println!("Here's a vector: {:?}", v); }); 
    handle.join().unwrap();
}

use std::sync::mpsc;

fn channeling() {
    let (tx, rx) = mpsc::channel(); // 送信と受信
    thread::spawn(move || {
        tx.send(String::from("hi")).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

use std::sync::Mutex;

fn hello_mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

use std::sync::Arc;

fn hello_mutex_with_arc() {
    let counter = Arc::new(Mutex::new(0)); // counterの参照を増やすにはArcが必要
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles { handle.join().unwrap(); }
    println!("Result: {}", *counter.lock().unwrap());
}
fn main() {
    hello_threading();
    thread_join();
    channeling();
    hello_mutex();
    hello_mutex_with_arc();
}