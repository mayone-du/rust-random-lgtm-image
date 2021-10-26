use std::thread;
mod lgtmoon;
mod pixabay;

// LGTM画像をランダムにダウンロードする
#[tokio::main]
async fn main() {
    lgtmoon::get_lgtmoon_image().await;

    // 別スレッドで非同期実行
    let handle = thread::spawn(|| async {
        pixabay::get_pixabay_image().await;
    });
    handle.join().expect("join error").await;
}
