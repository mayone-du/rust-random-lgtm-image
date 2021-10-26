use dotenv::dotenv;
use rand::{thread_rng, Rng};
use reqwest;
use serde_json::value;
use std::env;
use std::fs::File;
use std::io::Write;
use std::thread;

// TODO: 普通の画像をランダムにダウンロードし、中央にLGTMの文字を出力して画像に書き出す

// LGTM画像をランダムにダウンロードする
#[tokio::main]
async fn main() {
    let mut rng = thread_rng();
    // ランダムな整数を生成
    let rand_num: u32 = rng.gen_range(140000..150000);
    // リクエストするURLを定義
    // let req_url = format!("https://image.lgtmoon.dev/{}", rand_num);
    let req_url = format!("https://image.lgtmoon.dev/{}", 144444);
    // let req_url = "https://pixabay.com/get/g69c5561f84596ea414033342e630a1347b2704d74efc5eb74836e79f6e38a4b46ba7c92f061da4dd6509fece0393e032ac6c33419d1a69f344831dbf33462eef_640.jpg";
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

    let handle = thread::spawn(|| async {
        get_pixabay_image().await;
    });
    handle.join().expect("join error").await;
    // get_pixabay_image().await;
}

async fn get_pixabay_image() {
    // .envの値を読み込む
    dotenv().ok();
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

    // 検索結果の画像URLを取得
    let value: value::Value = serde_json::from_str(&pixabay_res_text).expect("");
    let hits = value
        .get("hits")
        .expect("get pixabay error")
        .as_array()
        .expect("as array error");
    let mut i = 0;
    let mut images: Vec<&serde_json::Value> = vec![];
    while i < hits.len() {
        images.push(hits[i].get("webformatURL").expect("webformatURL is None"));
        i = i + 1;
    }

    println!("\nimages is {:?}\n", images);

    let val = &images[0].to_string();
    let pixabay_req_url = rem_first_and_last(val);

    let res = match reqwest::get(pixabay_req_url).await {
        Ok(response) => {
            println!("OK. Response is : {:?}", response);
            response
        }
        Err(_e) => {
            panic!("Error!")
        }
    };

    println!(
        "\n\n{:?}",
        reqwest::get(pixabay_req_url).await.expect("error").status()
    );

    let image_bytes = res.bytes();
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
