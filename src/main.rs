use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::thread;

use rodio::{Decoder, OutputStream, Sink};

const PORT: u16 = 6969;

fn main() -> std::io::Result<()> {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(ip, PORT);

    let listener = TcpListener::bind(socket)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut song_buffer = [0u8; 100];
    let temp = stream.read(&mut song_buffer)?;
    let song = String::from_utf8(song_buffer[0..temp].to_vec()).unwrap();

    let mut file = BufReader::new(File::open(song)?);

    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;
    stream.write(&buffer)?;

    println!("done");

    Ok(())
}
