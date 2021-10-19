use std::fs::File;
use std::io::Write;

struct DurableFile {
    file: std::fs::File,
    needs_sync: bool,
}


impl DurableFile {
    pub fn new(file: std::fs::File) -> DurableFile {
        DurableFile{
            file: file,
            needs_sync: false, 
        }
    }

}

impl Write for DurableFile{
    fn write(&mut self, string: &[u8])-> Result<usize, std::io::Error> {
        self.file.write(string).expect("couldn't write within the impl");
        self.flush().expect("couldn't flush");
        Ok(string.len())
        }
    
    fn flush(&mut self) -> Result<(), std::io::Error>{

        self.file.sync_all().expect("couldn't sync");
        self.needs_sync = false;
        Ok(())

        //needs_sync becomes false


    }



}


impl Drop for DurableFile{
    fn drop(&mut self){

        if self.needs_sync == true {
            panic!("should've been flushed")
        }else{
            
        }

        //Ok(())
        

    
    }

}


fn main() {

    //if a durablefile is dropped with an outstanding write, its drop panics

    let f = File::create("test.txt").expect("Error with file");

    let mut a = DurableFile{
        file:f,
        needs_sync: true,
    };

    println!("{}",a.needs_sync);
    a.write(b"hey").expect("couldn't write to file");
    println!("{}",a.needs_sync)


    


    //let new = DurableFile::new(f);
    //f.write(b"a").expect("didn't write");

    /*let new = DurableFile{
        file: f,
        needs_sync:false,
    
    };*/

    

    //implement the write trait, if a write - sets the needs_sync, the flush method 
    //should clear it

}
