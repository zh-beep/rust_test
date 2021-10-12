
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use url::{Url};

fn parse_url(line:String) -> Option<Url>{
    let test = Url::parse(&line);
    match test{
        Ok(url) => { println!("{}", url); return Some(url)}
        Err(e) => return None,
    }
}

fn main() {


    let f = File::open("./data/content.txt").expect("this didn't work");



    let reader = BufReader::new(f);
    let mut count = 0;

    for line in reader.lines(){
        if let Ok(tested_line) = line{
            if !(tested_line.is_empty()){
                  parse_url(tested_line);  
                
            }

            count = count + 1;
        }
        
    }
    println!();
    println!("The file contains {} lines", count);
    

}
