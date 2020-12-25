//例子二
use futures::{ self, executor};

async fn learn_song() {
    println!("Learn song!");
}

async fn sing_song() {
    println!("Sing song!");
}

async fn dance() {
    println!("Dance!");
}

async fn learn_and_sing_song() {
    learn_song().await;
    sing_song().await;
}

async fn async_main() {
    let f1 = learn_and_sing_song();
    let f2 = dance();
    futures::join!(f1, f2);
}
fn main() {
    executor::block_on(async_main());
    println!("Hello, world!");
}

 