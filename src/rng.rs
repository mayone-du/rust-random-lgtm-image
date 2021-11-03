use rand::{thread_rng, Rng};

pub fn rand_num_gen(range: std::ops::Range<u32>) -> u32 {
  let mut rng = thread_rng();
  // ランダムな整数を生成
  rng.gen_range(range)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rand_num_gen() {
    let range = 1..10;
    let range_copy = range.clone();
    let num = rand_num_gen(range);
    assert!(range_copy.contains(&num));
    // not_contains
    assert_eq!(!range_copy.contains(&num), false);
  }
}
