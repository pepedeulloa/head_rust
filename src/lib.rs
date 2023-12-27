use std::{path::PathBuf, io::{BufReader, BufRead, Read}, error::Error, fs::File};

use clap::Parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
 /// Path of the file to read
 #[arg(id="FILE")]
 pub files: Vec<PathBuf>,

 /// Print specified bytes of the file
 #[arg(short='c', long="bytes", default_value_t = 0)]
 pub bytes: usize,

 /// Print specified lines of the file
 #[arg(short='n', long="lines", default_value_t = 0)]
 pub lines: usize
}

pub fn get_args () -> Result< Cli , Box<dyn Error>>{
 let cli = Cli::parse();

 Ok(cli)
}
pub fn open_file (filename: PathBuf, bytes: usize) -> Result<BufReader<File>,Box<dyn Error>> {
 
 let file = File::open(filename)?;
 let reader: BufReader<File>;
 if bytes > 0 {
  reader = BufReader::with_capacity(bytes,file);
  return Ok(reader);
 } else {
  reader = BufReader::new(file);
  return Ok(reader);
 }

}

pub fn read_by_bytes (reader: BufReader<File>, bytes: usize) {
 let mut count = 0;

 for byte in reader.bytes() {
  if count == bytes {break;}
  print!("{}", byte.unwrap() as char);
  count += 1;
 }
}

pub fn read_by_lines (reader: BufReader<File>, lines: usize) {
 
 let mut count = 0;
 
 for line in reader.lines() {
  if count == lines {break;}
  println!("{}", line.unwrap());
  count += 1;
 }

}

pub fn read_file(reader: BufReader<File>, bytes: usize, lines: usize) {
 if bytes == 0 && lines == 0 {
  read_by_lines(reader, 10);
 } else if bytes > 0 {
  read_by_bytes(reader, bytes);
 } else {
  read_by_lines(reader, lines)
 }
}