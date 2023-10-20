use air_rs::*;
use pest::Parser; 
use std::fs; 

fn main() {    
    let unparsed_file = fs::read_to_string("Clutch.air").expect("cannot read file"); 
    let result = air_rs::parse(&unparsed_file).unwrap(); 
    for (u64, action) in result {
       // dbg!(action); 
    }
}
