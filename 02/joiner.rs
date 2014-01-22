use std::os;
use std::io::File;

fn main() {
  let args: ~[~str] = os::args();
  if args.len() != 3 {
    println!("Usage: {:s} <inputfile>", args[0]);
  } else {
    let first_path = Path::new(args[1].clone());
    let second_path = Path::new(args[2].clone());
    let first_file = File::open(&first_path);
    let second_file = File::open(&second_path);
    match ((first_file, second_file)) {
      (Some(mut first), Some(mut second)) => {
        let first_bytes: ~[u8] = first.read_to_end();
        let second_bytes: ~[u8] = second.read_to_end();
        let output_file = File::create(&Path::new("message"));
        match (output_file) {
          Some(output) => {
            join(first_bytes, second_bytes, output);
          },
          _ => fail!("Error opening output file"),

        }
      },
      (_,_) => fail!("Error opening message files")
    }
  }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
  let mut ret = ~[];
  for i in range(0, a.len()) {
    ret.push(a[i] ^ b[i]);
  }
  ret
}

fn join(first_bytes: &[u8], second_bytes: &[u8], mut output_file: File) {
  let decrypted_bytes = xor(first_bytes, second_bytes);
  output_file.write(decrypted_bytes);
}
