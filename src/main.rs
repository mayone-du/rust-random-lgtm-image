use dotenv::dotenv;
use rand::{thread_rng, Rng};
use reqwest;
use serde_json::value;
use std::env;
use std::fs::File;
use std::io::Write;

// TODO: 普通の画像をランダムにダウンロードし、中央にLGTMの文字を出力して画像に書き出す

// LGTM画像をランダムにダウンロードする
#[tokio::main]
async fn main() {
    // .envの値を読み込む
    dotenv().ok();

    let mut rng = thread_rng();
    // ランダムな整数を生成
    let rand_num: u32 = rng.gen_range(140000..150000);
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

        buffer.write_all(&image_bytes).expect("Write byte error!");
    } else if status_code == 403 {
        panic!("LTGMOON Request Error!");
    }

    // PixabayAPIから画像検索結果を取得
    let pixabay_api_key = env::var("PIXABAY_API_KEY").expect("Enviroment Variables Error!");
    let pixabay_url = format!(
        "https://pixabay.com/api/?key={}&q={}+{}&image_type=photo",
        pixabay_api_key, "cat", "dog"
    );
    let pixabay_res_text = reqwest::get(pixabay_url)
        .await
        .expect("Pixabay request error")
        .text()
        .await
        .expect("to text error");

    let value: value::Value = serde_json::from_str(&pixabay_res_text).expect("");

    println!("\n{:?}\n", value);
}
