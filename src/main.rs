use rand::{thread_rng, Rng};
use reqwest;

#[tokio::main]
async fn main() {
    let mut rng = thread_rng();
    let rand_num: u32 = rng.gen_range(130000..150000);
    let req_url = format!("https://image.lgtmoon.dev/{}", rand_num);
    println!("request url is {}", req_url);
    let res = match reqwest::get(req_url).await {
        Ok(response) => response,
        Err(_e) => {
            panic!("Error!")
        }
    };
    println!("{:?}", res);
}
