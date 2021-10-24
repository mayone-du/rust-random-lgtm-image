use rand::{thread_rng, Rng};
use reqwest;
use std::fs::File;
use std::io::Write;

// TODO: 普通の画像をランダムにダウンロードし、中央にLGTMの文字を出力して画像に書き出す

// LGTM画像をランダムにダウンロードする
#[tokio::main]
async fn main() {
    let mut rng = thread_rng();
    // ランダムな整数を生成
    let rand_num: u32 = rng.gen_range(130000..150000);
    // リクエストするURLを定義
    let req_url = format!("https://image.lgtmoon.dev/{}", rand_num);
    println!("Request url is {}", req_url);
    let res = match reqwest::get(req_url).await {
        Ok(response) => {
            println!("OK. Response is : {:?}", response);
            response
        }
        Err(_e) => {
            panic!("Error!")
        }
    };

    // レスポンスのステータスコードを取得
    let status_code = res.status();

    if status_code == 200 {
        let image_bytes = res.bytes().await.expect("Error! to bytes");
        let mut buffer = File::create("lgtm.jpg").expect("Error! file create");

        // TODO: 解読
        let mut pos = 0;
        while pos < image_bytes.len() {
            let bytes_written = buffer.write(&image_bytes[pos..]).expect("Error!");
            pos += bytes_written;
        }
    } else if status_code == 403 {
        panic!("Request Error!");
    }
}
