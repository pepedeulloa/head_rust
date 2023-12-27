use head_rust::{Cli, get_args, read_file, open_file};
fn main() {

 let Cli{files, bytes, lines} = get_args().unwrap();

 if files.len() > 1 {
  for file in files {
   println!("\n===> {:?} <===", file);
   let reader = open_file(file, bytes).unwrap();
   read_file(reader, bytes, lines);
  }
 } else {
  for file in files {
   let reader = open_file(file, bytes).unwrap();
   read_file(reader, bytes, lines);
  }
 }
}
