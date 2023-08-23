use std::fs::File;
use std::io::{Read, Write, BufReader, Cursor};
use std::net::TcpStream;

use rodio::{OutputStream, Decoder, Sink, Source};

fn main() -> std::io::Result<()> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:6969")?;

    let mut buffer: Vec<u8> = Vec::new();
    // 576 + 32 bits + 9, 17 or 32 bytes.
    // 580 + 9, 17 or 32
    // 589 or 597 or 612
    
    stream.read_to_end(&mut buffer)?;

    println!("done");

    let mut temp = File::create("temp.mp3")?;
    temp.write(&mut buffer)?;

    let source = Decoder::new_mp3(Cursor::new(buffer)).unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
