use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main()-> std::io::Result<()> {
 let file = File::open("input.txt")
 //let file = File::open("input_test.txt")
 .expect("file not found!");
 let  buf_reader = BufReader::new(file);

 let mut timeIncreased = 0;
 let mut prevValue = 0;

 for line in buf_reader.lines() {
    let curValue = line?.parse::<i32>().unwrap();
    if(curValue) > prevValue {
        timeIncreased += 1;
    }
    prevValue = curValue;
 }
 println!("{}", timeIncreased - 1);
 Ok(())
}