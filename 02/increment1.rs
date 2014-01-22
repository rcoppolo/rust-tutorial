fn main() {
   let p = ~[1, 2, 3];
   let q = increment(p);
   for &x in q.iter() {
      print!("{:d} ", x);
   }
   println("");
}

fn increment(input: &[int]) -> ~[int] {
  let mut out = ~[];
  for i in range(0, input.len()) {
    out.push(input[i] + 1)
  }
  out
}

