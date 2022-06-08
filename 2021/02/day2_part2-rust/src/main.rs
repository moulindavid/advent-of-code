use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("input.txt")
    .expect("file not found!");
    let  buf_reader = BufReader::new(file);
   
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in buf_reader.lines() {
        let lineUnwraped = line.unwrap();
        let mut iter = lineUnwraped.split_whitespace();
        let command = iter.next().unwrap();
        let value = iter.next().unwrap().parse::<i32>().unwrap();
        match command { 
        "forward"=> {
            horizontal += value;
            depth += value * aim;
        },
        "down"=> aim+= value,
        "up"=> aim -= value,
        &_ => todo!(),
        };
    }
    println!("{}", horizontal);
    println!("{}", depth);
    println!("{}", horizontal * depth);
    println!("{}", aim);
}
