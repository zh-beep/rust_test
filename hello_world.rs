// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}
fn main() {
    // Statements here are executed when the compiled binary is called
	let mut var = 1.0;
	var = 3.0;
	let tup = (3,true);
	println!("{}",reverse(tup));
    // Print text to the console
    //println!("Hello World!");
	//println!("{}",var);
}
