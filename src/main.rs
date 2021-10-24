use rand::{thread_rng, Rng};
use reqwest;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
    let mut rng = thread_rng();
    let rand_num: u32 = rng.gen_range(130000..150000);
    let req_url = format!("https://image.lgtmoon.dev/{}", rand_num);
    println!("request url is {}", req_url);
    let res = match reqwest::get(req_url).await {
        Ok(response) => {
            println!("OK. Response is : {:?}", response);
            response
        }
        Err(_e) => {
            panic!("Error!")
        }
    };

    let image_bytes = res.bytes().await.expect("Error! to bytes");
    let mut buffer = File::create("hoge.jpeg").expect("Error! file create");

    let mut pos = 0;

    while pos < image_bytes.len() {
        let bytes_written = buffer.write(&image_bytes[pos..]).expect("Error!");
        pos += bytes_written;
    }
}
