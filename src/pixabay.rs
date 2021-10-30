use dotenv::dotenv;
use image::{GenericImageView, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use serde_json::value;
use std::env;
use std::fs::File;
use std::io::Write;

pub async fn get_pixabay_image() {
  // .envの値を読み込む
  dotenv().ok();
  let pixabay_api_key = env::var("PIXABAY_API_KEY").unwrap();

  let pixabay_url = format!(
    "https://pixabay.com/api/?key={}&q={}&image_type=photo",
    pixabay_api_key, "",
  );
  // PixabayAPIから画像検索結果を取得
  let pixabay_res_text = reqwest::get(pixabay_url)
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

  // 検索結果の画像URLを取得
  let value: value::Value = serde_json::from_str(&pixabay_res_text).unwrap();
  let hits = value.get("hits").unwrap().as_array().unwrap();
  let mut i = 0;
  let mut images: Vec<&serde_json::Value> = vec![];
  while i < hits.len() {
    images.push(hits[i].get("webformatURL").unwrap());
    i = i + 1;
  }

  // 画像URLの配列の一個目を取得し、画像を取得
  let img_url = &images[0].to_string();
  let pixabay_req_url = rem_first_and_last(img_url);

  let res = match reqwest::get(pixabay_req_url).await {
    Ok(response) => response,
    Err(_e) => {
      panic!("Error!")
    }
  };

  // ステータスコードが200でない場合は処理を終了
  let status = reqwest::get(pixabay_req_url).await.unwrap().status();
  if status != 200 {
    return;
  }
  let image_bytes = res.bytes().await.unwrap();
  let mut buffer = File::create("pixabay.jpg").unwrap();
  buffer.write_all(&image_bytes).unwrap();

  // 背景用画像を取得
  let mut image = match image::open("pixabay.jpg") {
    Ok(image) => image,
    Err(err) => panic!("画像を読み取れませんでした。{:?}", err),
  };
  // 画像の横幅、縦幅を取得
  let (width, height) = image.dimensions();
  // 描画するためのフォントを取得
  let font = Vec::from(include_bytes!("../assets/font/orkney-bold.ttf") as &[u8]);
  let font = Font::try_from_vec(font).unwrap();

  // 文字のサイズは画像の横幅に応じて決定
  let scale_size = width as f32 * 0.2;
  let scale = Scale {
    x: scale_size,
    y: scale_size,
  };

  // 一文字の大きさは横幅*0.2
  let font_size = (width as f32 * 0.2).ceil() as u32;

  // LGTMの4文字だから、0.8分になるため、あとで横調整のために0.1分引く
  // |0.1|0.8|0.1|のような横幅のイメージ
  // xは画像の横幅を2で割り、文字サイズの2文字分更に引く
  let (position_x, position_y) = (
    (width / 2 - font_size * 2 / 2 - (font_size as f32 * 0.1) as u32),
    (height / 2 - font_size / 2),
  );

  // テキストを画像に描画
  draw_text_mut(
    &mut image,
    Rgba([255u8, 255u8, 255u8, 255u8]),
    position_x,
    position_y,
    scale,
    &font,
    "LGTM",
  );

  // 画像を保存
  let save_path = format!("generated/pixabay.jpg");
  image.save(save_path).unwrap();
}

// 文字列をchar型に分解して最初と最後の文字列を削除
fn rem_first_and_last(value: &str) -> &str {
  let mut chars = value.chars();
  chars.next();
  chars.next_back();
  chars.as_str()
}
