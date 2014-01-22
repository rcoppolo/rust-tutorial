//fn main() {
//  let mut x = ~10;
//  increment(x);
//  println!("{:d}", *x);
//}
//
//fn increment(y: &mut int) {
//  *y = *y + 1;
//}

fn main() {
   let mut p = ~[1, 2, 3];
   incrementMut(p);
   for &x in p.iter() {
      print!("{:d} ", x);
   }
   println("");
}

fn incrementMut(x: &mut [int]) {
  for i in range(0, x.len()) {
    x[i] = x[i] + 1;
  }
}
