use crate::rng;
use reqwest;
use std::fs::File;
use std::io::Write;

pub async fn get_lgtmoon_image() {
  let lgtmoon_base_url = "https://image.lgtmoon.dev/";
  // ランダムな整数を生成して、それを画像のURLに結合する
  let rand_num = rng::rand_num_gen(140_000..150_000);
  let req_url = format!("{}{}", lgtmoon_base_url, rand_num);
  println!("\nRequest url is {}\n", req_url);
  let res = match reqwest::get(req_url).await {
    Ok(response) => response,
    Err(e) => {
      panic!("LGTMOON Request Error! {}", e)
    }
  };

  // レスポンスのステータスコードを取得
  let status_code = res.status();

  if status_code == 200 {
    // レスポンスをbyteとして保存し、新しいファイルに書き込む
    let image_bytes = res.bytes().await.unwrap();
    let mut buffer = File::create("lgtm.jpg").unwrap();
    buffer.write_all(&image_bytes).unwrap();
  } else if status_code == 403 {
    // エラーだった場合はスキップする
    println!("LTGMOON Request Error!");
  }
}
