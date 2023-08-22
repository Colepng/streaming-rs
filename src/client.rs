use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6969")?;
    // let mut buffer: String = String::new();
    // loop {
    stream.write("hello".as_bytes())?;
    stream.write("close".as_bytes())?;
        // let n = stream.read_to_string(&mut buffer)?;
        // println!("{}", &buffer);
    // }
    Ok(())
}
