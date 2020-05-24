use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

pub struct NetWork {
    first: String,
    second: String,
}

impl Future for NetWork {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        thread::sleep(Duration::from_secs(2));
        return Poll::Ready(self.second.clone());
    }
}

pub fn test_future() {
    let ne = NetWork {
        first: "first".to_string(),
        second: "second".to_string(),
    };

    println!("{}", "begin...");
//    futures::executor::block_on(
//        async {
//            println!("{}", ne.await)
//        }
//    );
    async_std::task::spawn(
        async {
            println!("{}", ne.await)
        }
    );
    println!("{}", "end");
    for i in 0..20 {
        thread::sleep(Duration::from_secs(1));
        println!("{}", i)
    }
}

pub fn test_mpsc() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}