use std::{thread, time::Duration};
use tokio::sync::mpsc::channel;

// 可以通过 cargo run >> result.log 方式运行，将输出的日志打印到文件中
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // 执行异步闭包函数
    print_num(3).await;

    // 1. 线程使用
    // 开启线程模式执行任务，spawn的签名是一个FnOnce()->T类型，执行一次返回结果是T
    let mut handlers = Vec::new();
    println!("spawn funcs begin");
    for i in 0..9 {
        // 开启线程执行
        // 通过move关键字将i所有权移动到闭包中
        let handler = thread::spawn(move || {
            println!("current i = {}", i);
        });
        handlers.push(handler);
    }

    // 等待线程执行完毕
    for handler in handlers {
        handler.join().expect("failed to exec handler");
    }

    println!("spawn funcs end");

    // 2. tokio::spawn异步任务执行，它的参数F是一个Future
    // F: Future + Send + 'static, 表示Future类型，能在线程中安全转移所有权
    // F::Output: Send + 'static, 返回结果Output是一个关联类型，type Output;
    // 这里的static表示静态生命周期，在程序执行过程中一直有效
    // 通过tokio::spawn执行异步任务
    let handler = tokio::spawn(async move {
        print_num(3).await;
    });
    handler.await.expect("failed to exec async fn");

    // 3. tokio mpsc channel通道使用
    // 定义有缓冲通道
    // tokio::sync::mpsc::channel
    // 有缓冲通道，其中tx表示发送通道，tx.send方法签名是send(&self, value: T)，因此它不需要定义为mut类型；
    // rx表示接收通道，用于接收tx发送者通道过来的值，相当于水管一样的机制；
    let (tx, mut rx) = channel(100);

    // 4. tokio::sync::broadcast::channel 广播模式使用
    // 通过广播模式，创建channel
    // 允许一个发送者向多个接收者广播消息，非常适合用于任务取消和优雅停机场景。
    // 创建了一个容量为 1 的广播通道，类型为bool
    // 这种模式常用于优雅停机，当需要通知多个任务同时停止执行时。
    // shutdown_tx 表示平滑退出的发送者，shutdown_rx表示广播接收者
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel::<bool>(1);
    // 模拟发送退出信号量，让tx停止发送，同时rx退出接收tx发送的消息
    tokio::spawn(async move {
        // 等待2s后，发送退出信号
        tokio::time::sleep(Duration::from_secs(2)).await;
        shutdown_tx.send(true).expect("failed to send value");
    });

    let mut i: i64 = 0;
    tokio::spawn(async move {
        println!("worker exec begin");
        loop {
            let stop = shutdown_rx.try_recv().unwrap_or(false);
            if stop {
                println!("worker will exit loop");
                drop(tx); // 关闭发送着通道，这样接收者rx就会停止接收消息
                break;
            }

            // 发送消息到通道中
            let msg = format!("hello,index:{}", i);
            i += 1;
            let _ = tx.send(msg).await.expect("failed to send msg");
        }
    });

    // 通过异步方式接收tx发送者发到通道中的值
    let handler = tokio::spawn(async move {
        // 这里会一直阻塞式的接收消息，除非tx发送者已关闭或退出，例如: drop(tx) 关闭发送通道
        while let Some(val) = rx.recv().await {
            println!("recv worker val is {}", val);
        }
    });
    // 等待tokio开启的异步任务Future执行完毕
    handler.await.unwrap();
    println!("worker shutdown success");
}

// 定义异步闭包函数
async fn print_num(i: i32) {
    for i in 0..i {
        let c = || {
            println!("current i:{}", i);
        };

        // 这里执行定义的闭包
        c();
    }
}
