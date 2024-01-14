use crate::default_io;
use std::collections::HashSet;
use std::io::Write;

fn dfs(adj: &[Vec<usize>], a: usize) -> HashSet<usize> {
  let mut nodes = HashSet::new();

  let mut stack = vec![a];

  while let Some(n) = stack.pop() {
    if nodes.insert(n) {
      for &next in adj[n].iter() {
        stack.push(next);
      }
    }
  }

  nodes
}

fn solve() -> std::io::Result<()> {
  let (mut input, mut output) = default_io();

  let n: usize = input.token();
  let m: u32 = input.token();

  let mut adj = vec![vec![]; n + 1];

  for _ in 0..m {
    let a: usize = input.token();
    let b: usize = input.token();
    adj[a].push(b);
    adj[b].push(a);
  }

  let mut count = 0;
  let mut visited = vec![false; n + 1];
  let mut bridges = vec![];

  for a in 1..=n {
    if visited[a] {
      continue;
    }
    count += 1;
    let hs = dfs(&adj, a);
    for &n in hs.iter() {
      visited[n] = true;
    }
    bridges.push(a);
  }

  writeln!(output, "{}", count - 1)?;
  for w in bridges.windows(2) {
    writeln!(output, "{} {}", w[0], w[1])?;
  }

  Ok(())
}
