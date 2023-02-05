use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct MyWriter<T> {
    writer: T,
}

impl <W: Write> MyWriter<W> {

    pub fn new(writer: W) -> Self {
        Self{
            writer
        }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }

}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let bw = BufWriter::new(stream);
    let mut writer = MyWriter::new(bw);
    writer.write("hi rust!").unwrap();
}