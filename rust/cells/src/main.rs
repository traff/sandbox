#![feature(old_io)]
#![feature(old_path)]

#![feature(env)]

use std::env;
use std::old_io::File;

use std::collections::BitVec;
use std::collections::HashSet;

fn parse(line: String)->BitVec {
  let chars_to_trim: &[char] = &[10 as char, 13 as char];
  let trimmed: &str = line.trim_matches(chars_to_trim);
    
  let mut i = 0;
  let mut v = BitVec::from_elem(line.len(), false);
  for l in trimmed.chars() {
    // println!("{}", l as u8);
    v.set(i, if l == ' ' { false } else { true});
    i+=1;
  }
  return v;
}

fn serialize(v: &BitVec) -> String {
  let mut x  = "".to_string();
  for y in v {
    x.push(if y {'*'} else {' '})
  }
  x
}

fn mutate(v: &BitVec, cycle: bool)->BitVec {
  let mut x = BitVec::from_elem(v.len(), false);
  let mut i = 0;
  for i in 0..v.len() {
      let a = if (i>0) {v.get(i-1).unwrap()} else { if cycle {v.get(v.len()-1).unwrap()} else {false}};
      let b = if i<v.len()-1 {v.get(i+1).unwrap()} else { if cycle {v.get(0).unwrap()} else {false}};
      
      x.set(i, if (a as u8 + b as u8) != 1  { false } else { true });
  }
  x
}

fn go(v: &BitVec, s: &mut HashSet<BitVec>, cycle:bool)->usize {
    println!("{}", serialize(v));
    if s.contains(v) {
      s.len()
    } else {
      s.insert(v.clone());
      go(&mutate(v, cycle), s, cycle)
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: lenta <input_file>");
    } else {

    let cycle = args.len() > 2 && args[2].as_slice() == "cycle"; 

    // Create a path to the desired file
    let path = Path::new(&args[1]);
    let display = path.display();

    // Open the path in read-only mode, returns `IoResult<File>`
    let mut file = match File::open(&path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.desc),
        Ok(file) => file,
    };


    // Read the file contents into a string, returns `IoResult<String>`
    match file.read_to_string() {
        Err(why) => panic!("couldn't read {}: {}", display, why.desc),
        Ok(string) => println!("{}", go(&parse(string), &mut HashSet::new(), cycle)),
    }

    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
