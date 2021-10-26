use rand::{thread_rng, Rng};

pub fn rand_num_gen(range: std::ops::Range<u32>) -> u32 {
  let mut rng = thread_rng();
  // ランダムな整数を生成
  rng.gen_range(range)
}
