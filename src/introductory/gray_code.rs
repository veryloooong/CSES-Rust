fn print_arr(arr: &Vec<u32>) -> () {
  let mut i = arr.len() - 1;
  while i > 0 && i <= arr.len() {
    print!("{}", arr[i]);
    i -= 1;
  }
  print!("{}\n", arr[0]);
}

fn rightmost_set_bit(num: i32) -> usize {
  return num.trailing_zeros() as usize;
}

fn solve(n: u32) {
  let mut arr: Vec<u32> = vec![0; n as usize];

  print_arr(&arr);

  for p in 1..1 << n {
    let lsb = rightmost_set_bit(p);
    arr[lsb] = 1 - arr[lsb];
    print_arr(&arr);
  }
}
