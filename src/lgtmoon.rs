use crate::rng;
use reqwest;
use std::fs::File;
use std::io::Write;

pub async fn get_lgtmoon_image() {
  let lgtmoon_base_url = "https://image.lgtmoon.dev/";
  let rand_num = rng::rand_num_gen(140_000..150_000);
  // リクエストするURLを定義
  let req_url = format!("{}{}", lgtmoon_base_url, rand_num);
  // let req_url = format!("{}{}", lgtmoon_base_url, 144447);
  println!("\nRequest url is {}\n", req_url);
  let res = match reqwest::get(req_url).await {
    Ok(response) => response,
    Err(e) => {
      panic!("Error! {}", e)
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
    // TODO: エラーだった場合はもう一度ランダムに取得するか、スキップする
    // let new_rand_num = rng::rand_num_gen(140_000..150_000);
    // let req_url = format!("{}{}", lgtmoon_base_url, new_rand_num);
    panic!("LTGMOON Request Error!");
  }
}
