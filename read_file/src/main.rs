
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {


    let f = File::open("./data/content.txt").expect("this didn't work");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(e) => panic!("Error:{}", e),
    //     //Err(e) => println!("Error:{}", e),
    //     // substitute this placeholder for the two possible patterns from the result type
    //     // PATTERN => EXPRESSION,
    //     // PATTERN => EXPRESSION,
    // };

    let reader = BufReader::new(f);
    let mut count = 0;

    for line in reader.lines(){
        if let Ok(tested_line) = line{
            println!("{}",tested_line);
            count = count + 1;
        }
        
    }
    println!();
    println!("The file contains {} lines", count);

    // let mut line = String::new();
    // let len = reader.read_line(&mut line);
    // println!("First line is {} bytes long", len);
    //Ok(())


   

//     let mut reader = BufReader::new(f);
//     //let mut file_content = String::new();




//     //f.read_to_string(&mut file_content).expect("could not read file");
//     //println!("File is {:?}",f)

}
