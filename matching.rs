fn main() {

  //let a = (true, 22);
  //let a = (true, 19);
  //let a = (false, 43);
  let a = (false, 0);

  match a {
    (true, x) if 20 < x && x < 26  => println("True and in range."),
    (true, _) => println("True and not in range."),
    (_, y) if 40 < y && y < 49 => println("False and in range."),
    (_,_) => println("No match here.")
  }

}
