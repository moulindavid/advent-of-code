use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main()-> std::io::Result<()> {
 //let file = File::open("input.txt")
 let file = File::open("input.txt")
 .expect("file not found!");
 let  buf_reader = BufReader::new(file);
 let mut lines = Vec::new();
 lines = buf_reader.lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();

 let mut timeIncreased = 0; 
 let mut prevSlidingSumValue = 0;


     // A counter variable
    let mut n = 0;

    // Loop while `n` is less than 101
    while n < lines.len() {
        //println!("{}", n);
        let curValue = lines.get(n).unwrap_or(&"0".to_string()).parse::<i32>().unwrap() + lines.get(n + 1).unwrap_or(&"0".to_string()).parse::<i32>().unwrap() + lines.get(n + 2).unwrap_or(&"0".to_string()).parse::<i32>().unwrap();
        if(curValue) > prevSlidingSumValue {
            timeIncreased += 1;
        }

        prevSlidingSumValue = curValue;
        n+=1;
    }
 
 println!("{}", timeIncreased - 1);
 Ok(())
}