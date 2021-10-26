use std::thread;
mod lgtmoon;
mod pixabay;
mod rng;

// LGTM画像をランダムにダウンロードする
#[tokio::main]
async fn main() {
    lgtmoon::get_lgtmoon_image().await;

    // 別スレッドで非同期実行 (やる必要ないけど別スレッド使ってみたかった)
    let handle = thread::spawn(|| async {
        pixabay::get_pixabay_image().await;
    });
    handle.join().expect("join error").await;
}
