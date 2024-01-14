use crate::default_io;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Write;

fn solve() -> std::io::Result<()> {
  let (mut input, mut output) = default_io();
  // let (mut input, mut output) = from_file();

  let n: usize = input.token();
  let m: isize = input.token();

  const MOVES: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
  const DIRS: [char; 4] = ['D', 'U', 'R', 'L'];

  let grid: Vec<Vec<char>> = (0..n)
    .map(|_| input.token::<String>())
    .map(|x| x.chars().collect::<Vec<_>>())
    .collect();
  let mut start = (0, 0);

  for (i, row) in grid.iter().enumerate() {
    for (j, &ch) in row.iter().enumerate() {
      if ch == 'A' {
        start = (i as isize, j as isize);
        break;
      }
    }
  }

  let mut queue = VecDeque::new();
  let mut prev = HashMap::new();

  queue.push_back(start);
  prev.insert(start, start);

  while let Some(cur) = queue.pop_front() {
    for &d in MOVES.iter() {
      let new = (cur.0 + d.0, cur.1 + d.1);
      if new.0 < 0 || new.0 >= n as isize || new.1 < 0 || new.1 >= m {
        continue;
      }
      let ch = grid[new.0 as usize][new.1 as usize];
      if ch == 'B' {
        writeln!(output, "YES")?;
        prev.insert(new, cur);
        let mut pos = new;
        let mut path = VecDeque::new();
        while pos != start {
          path.push_front(pos);
          pos = *prev.get(&pos).unwrap();
        }
        writeln!(output, "{}", path.len())?;
        for &d in path.iter() {
          let m = (d.0 - pos.0, d.1 - pos.1);
          write!(
            output,
            "{}",
            DIRS[MOVES.iter().position(|&x| x == m).unwrap()]
          )?;
          pos = d;
        }
        write!(output, "\n")?;

        return Ok(());
      }
      if ch == '.' && !prev.contains_key(&new) {
        queue.push_back(new);
        prev.insert(new, cur);
      }
    }
  }

  writeln!(output, "NO")?;

  Ok(())
}
