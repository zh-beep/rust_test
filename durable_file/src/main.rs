struct DurableFile {
    file: std::fs::File,
    needs_sync: bool,
}


impl DurableFile {
    pub fn new(file: std::fs::File) -> DurableFile {
        DurableFile{
            file: file,
            needs_sync: true, 
        }
    }
}

fn main() {
    println!("Hello, world!");

    //if a durablefile is dropped with an outstanding write, its drop panics
    
    //implement the write trait, if a write - sets the needs_sync, the flush method 
    //should clear it

}
