use rand::{thread_rng, Rng};
use reqwest;

use std::fs::File;
use std::io::Write;

pub async fn get_lgtmoon_image() {
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
}
