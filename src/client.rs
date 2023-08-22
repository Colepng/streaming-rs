use std::io::{Read, Write, BufReader, Cursor};
use std::net::TcpStream;

use rodio::{OutputStream, Decoder, Sink};

fn main() -> std::io::Result<()> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:6969")?;
    // let mut buffer: String = String::new();
    // loop {
    // stream.write("hello".as_bytes())?;
    
    let mut empty = false;

    let mut internal_buffer: Vec<u8> = Vec::new();
    internal_buffer.reserve(25000000);
    let mut song_buffer = Cursor::new(internal_buffer);
    let mut buffer = [0u8; 256];
    let mut recived = 0;
    while !empty {
        match stream.read(&mut buffer)? {
            n if n != 0 => {
                let internal_buffer = song_buffer.get_mut();
                internal_buffer.append(&mut Vec::from(buffer));
                buffer = [0u8; 256];
                println!("recived");
                recived += n;
                println!("{recived}");
                println!("{n}");
            },
            _ => {
                empty = true;
            },
        }
    }

    println!("done");

    // loop {
    // let buffer = BufReader::new(song_buffer);

        // let mut temp = [0u8; 25000000];
        // buffer.read(&mut temp);
        // println!("{:#?}", temp);
        let source = Decoder::new(song_buffer).unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        // let file = BufReader::new(File::open("music.flac").unwrap());
        // Decode that sound file into a source
        // let source = Decoder::new(file).unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();

        sink.append(source);
        sink.sleep_until_end();
    // }
    stream.write("close".as_bytes())?;
        // let n = stream.read_to_string(&mut buffer)?;
        // println!("{}", &buffer);
    // }
    Ok(())
}
