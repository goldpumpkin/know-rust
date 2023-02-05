use std::fs::File;
use std::io::{BufReader, Read, Result};

struct MyReader<T> {
    reader: T,
    buf: String,
}

impl<R> MyReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::with_capacity(1024),
        }
    }
}

impl<R> MyReader<R>
    where
        R: Read,
{
    pub fn process(&mut self) -> Result<usize> {
        self.reader.read_to_string(&mut self.buf)
    }
}

fn main() {
    let f = File::open("01_type_system/testresource/test.csv").unwrap();
    let mut reader = MyReader::new(BufReader::new(f));
    let size = reader.process().unwrap();
    println!("file size: {}", size);

    let content = reader.buf;
    println!("file content: {}", content)

}