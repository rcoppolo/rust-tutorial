use std::os;

fn main() {
	if os::args().len() < 2 {
		println("Error: Please provide a number as argument.");
		return;
	}

	let i = from_str::<int>(os::args()[1]).unwrap();
  if i < 1 {
		println("Error: Please provide a positive number as argument.");
		return;
  }

	println!("Lowest number with {:d} Collatz steps: {:d}", i, collatz_steps(i));
}

fn collatz_steps(i: int) -> int {
  let mut j = 1;
  while collatz(j) != i {
    j += 1;
  }
  j
}

fn collatz(N: int) -> int {
	if N == 1 { return 0; }
	match N % 2 {
		0 => { 1 + collatz(N/2) }
		_ => { 1 + collatz(N*3+1) }
	}
}
