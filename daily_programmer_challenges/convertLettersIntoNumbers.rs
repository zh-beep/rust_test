fn main() {
    //let x = 'a' as i32;
    //println!("{}",x);
    /*println!("{}", 65 as char);
    println!("{}", 'a' as i32);*/
    let string_val = "ok";
    println!("Converting the word {} into a value based on what place a letter is in the alphabet is: {}", string_val, convert(string_val));
	println!("hey");
}

fn convert(y: &str) -> i32{
    let mut val = 0;
    for a in y.chars(){
        val = val + (a as i32) - 97;
        //println!("{}",val);
    }
    return val
}
