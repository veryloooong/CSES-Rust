use std::collections::HashSet;

fn main(arr: &[usize]) -> usize {
  let mut nums = HashSet::new();
  arr.iter().for_each(|&x| {
    nums.insert(x);
  });
  nums.len()
}
