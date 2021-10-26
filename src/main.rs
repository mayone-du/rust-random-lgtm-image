use dotenv::dotenv;
use image::{GenericImageView, Rgba};
use imageproc::drawing::draw_text_mut;
use rand::{thread_rng, Rng};
use reqwest;
use rusttype::{Font, Scale};
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
    let req_url = format!("https://image.lgtmoon.dev/{}", 144447);
    println!("\nRequest url is {}\n", req_url);
    let res = match reqwest::get(req_url).await {
        Ok(response) => response,
        Err(_e) => {
            panic!("Error!")
        }
    };

    // レスポンスのステータスコードを取得
    let status_code = res.status();

    if status_code == 200 {
        // レスポンスをbyteとして保存し、新しいファイルに書き込む
        let image_bytes = res.bytes().await.expect("Error! to bytes");
        let mut buffer = File::create("lgtm.jpg").expect("Error! file create");
        buffer.write_all(&image_bytes).expect("Write byte error!");
    } else if status_code == 403 {
        panic!("LTGMOON Request Error!");
    }

    // 別スレッドで非同期実行
    let handle = thread::spawn(|| async {
        get_pixabay_image().await;
    });
    handle.join().expect("join error").await;
}

async fn get_pixabay_image() {
    // .envの値を読み込む
    dotenv().ok();
    // PixabayAPIから画像検索結果を取得
    let pixabay_api_key = env::var("PIXABAY_API_KEY").expect("Enviroment Variables Error!");
    // TODO: 単語の指定
    let pixabay_url = format!(
        "https://pixabay.com/api/?key={}&q={}+{}&image_type=photo",
        pixabay_api_key, "cat", "money"
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

    let val = &images[0].to_string();
    let pixabay_req_url = rem_first_and_last(val);

    let res = match reqwest::get(pixabay_req_url).await {
        Ok(response) => response,
        Err(_e) => {
            panic!("Error!")
        }
    };

    let status = reqwest::get(pixabay_req_url).await.expect("error").status();

    println!("\n{}\n", status);
    let image_bytes = res.bytes().await.expect("byte error");
    let mut buffer = File::create("pixabay.jpg").expect("Error! file create");
    buffer.write_all(&image_bytes).expect("file write error");

    // テキストを描画

    // 背景用画像を取得
    let mut image = match image::open("pixabay.jpg") {
        Ok(image) => image,
        Err(err) => panic!("画像を読み取れませんでした。{:?}", err),
    };
    let height = image.height();
    let width = image.width();
    // 描画するためのフォントを取得
    let font = Vec::from(include_bytes!("../assets/font/orkney-bold.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    // 文字のサイズは画像の横幅に応じて決定
    // 4文字だから、0.8分になるため、あとで横丁製のために0.1分引く
    let (float_size, font_size) = (width as f32 * 0.2, (width as f32 * 0.2).ceil() as u32);
    let scale = Scale {
        x: float_size,
        y: float_size,
    };

    // xは画像の横幅を2で割り、文字サイズの2文字分更に引く
    let (position_x, position_y) = (
        (width / 2 - font_size * 2 / 2 - (font_size as f32 * 0.1) as u32),
        (height / 2 - font_size / 2),
    );

    // テキストを画像に描画
    draw_text_mut(
        &mut image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        // Rgba([0u8, 0u8, 0u8, 0u8]),
        position_x,
        position_y,
        scale,
        &font,
        "LGTM",
    );

    // 日付をファイル名にして画像を保存
    let save_path = format!("generated/pixabay.jpg");

    // 画像を保存
    image.save(save_path).unwrap();
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
