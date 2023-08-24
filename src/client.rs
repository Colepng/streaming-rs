use std::io::{Cursor, Read, Write};
use std::net::TcpStream;

use rodio::{Decoder, OutputStream, Sink};

fn main() -> std::io::Result<()> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let song = buffer("6e56CkYWD3JO6ovFZBA8po")?;

    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = Decoder::new_mp3(Cursor::new(song.clone())).unwrap();

    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}

fn buffer(song: &str) -> std::io::Result<Vec<u8>> {
    let mut stream = TcpStream::connect("127.0.0.1:6969")?;

    stream.write(song.as_bytes())?;

    let mut buffer: Vec<u8> = Vec::new();

    stream.read_to_end(&mut buffer)?;

    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(buffer)
}
