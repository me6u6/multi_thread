use std::time::Duration;
use tokio::{signal::ctrl_c, spawn, task::JoinHandle, time::sleep};
use tokio_util::sync::CancellationToken;

fn ctrl_c_handler(token: CancellationToken) -> JoinHandle<()> {
    spawn(async move {
        ctrl_c().await.unwrap();
        println!("received ctrl-c");
        token.cancel();
    })
}

fn make_thread(
    token: CancellationToken,
    start: usize,
    end: usize,
    key: &'static str,
) -> JoinHandle<()> {
    spawn(async move {
        for i in start..end {
            if token.is_cancelled() {
                println!("graceful stop handle {}", key);
                break;
            }
            println!("{}", i);
            sleep(Duration::from_secs(1)).await;
        }
    })
}

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let handles = vec![
        make_thread(token.clone(), 0, 10, "1"),
        make_thread(token.clone(), 10, 20, "2"),
        make_thread(token.clone(), 100, 101, "3"),
    ];
    ctrl_c_handler(token);
    for handle in handles {
        handle.await.unwrap();
    }
}
