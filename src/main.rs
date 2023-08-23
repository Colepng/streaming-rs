use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::io::prelude::*;

use rodio::{OutputStream, Decoder, Sink};

const PORT: u16 = 6969;

fn main() -> std::io::Result<()> {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(ip, PORT);

    let listener = TcpListener::bind(socket)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) =>{
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {println!("{:?}", e);}
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()>{


    
    // Get a output stream handle to the default physical sound device
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    // let file = File::open("output_no_meta.mp3").unwrap();
    // Decode that sound file into a source
    // let source = Decoder::new_mp3(file).unwrap();
    
    // let sink = Sink::try_new(&stream_handle).unwrap();
    
    // sink.append(source);
    // sink.sleep_until_end();
    let mut file = BufReader::new(File::open("output_no_meta.mp3").unwrap());
    //
    let meta = mp3_metadata::read_from_file("output_no_meta.mp3").unwrap();
    
    
    let frame = &meta.frames[0];
    let bitrate = frame.bitrate as u32 * 1000;
    let samp_rate = frame.sampling_freq;
    let samples = frame.size;
    
    let bps = samples as f64 / 8.0;
    
    let fsize = (bps * bitrate as f64 / samp_rate as f64 ) + if frame.padding {1.} else {0.};
    println!("{}", fsize);

    let mut buffer = [0u8; 288];
    // let mut buffer: Vec<u8> = Vec::new();
    let mut empty = false;
    let mut temp = 0;
    while !empty {
        match file.read(&mut buffer)? {
            n if n != 0 => {
                stream.write(&buffer[0..n])?;
                println!("sending {} bytes", n);
                temp += 1;
                if temp >= 400 {
                    empty = true;
                }
            },
            _ => empty = true,
        };
    }

    println!("done");

    Ok(())
}
