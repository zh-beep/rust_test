use std::fs::File;

fn main() {

    let f = File::open("./data/content.txt");

    //let a: () = 

    match f {
        Ok(file) => println!("File opened"),
        Err(e) => println!("Error:{}", e),
        // substitute this placeholder for the two possible patterns from the result type
        // PATTERN => EXPRESSION,
        // PATTERN => EXPRESSION,
    };
}
