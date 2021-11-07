use crossbeam::sync::Parker;
use futures_lite::pin;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use waker_fn::waker_fn;
struct Shared<T> {
    value: Option<T>,
    waker: Option<Waker>,
}
pub struct BlockingFuture<T>(Arc<Mutex<Shared<T>>>);

impl<T: Send> Future for BlockingFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut guard = self.0.lock().unwrap();
        if let Some(value) = guard.value.take() {
            return Poll::Ready(value);
        }
        guard.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

pub fn spawn_blocking<T, F>(closure: F) -> BlockingFuture<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let shared = Shared {
        value: None,
        waker: None,
    };
    let inner = Arc::new(Mutex::new(shared));

    std::thread::spawn({
        let cloned_inner = inner.clone();
        move || {
            let value = closure();
            let maybe_waker = {
                let mut guard = cloned_inner.lock().unwrap();
                guard.value = Some(value);
                guard.waker.take()
            };
            if let Some(waker) = maybe_waker {
                waker.wake();
            }
        }
    });

    BlockingFuture(inner)
}

fn block_on<F: Future>(future: F) -> F::Output {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = waker_fn(move || unparker.unpark());
    let mut context = Context::from_waker(&waker);
    pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => parker.park(),
        }
    }
}

async fn dispatch() {
    let futures = vec![
        spawn_blocking(|| println!("abc")),
        spawn_blocking(|| println!("def")),
        spawn_blocking(|| println!("ghi")),
    ];
    for fut in futures {
        fut.await;
    }
}

fn main() {
    block_on(dispatch());
}
