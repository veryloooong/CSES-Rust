use std::collections::HashSet;

fn main(n: usize, arr: &[usize]) -> usize {
  let mut nums = HashSet::new();
  let _ = arr.iter().take(n).map(|&x| nums.insert(x));
  nums.len()
}
