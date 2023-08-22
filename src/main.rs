use std::fs::File;
use std::io::BufReader;
use std::thread;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use std::net::{TcpStream, TcpListener, SocketAddrV4, Ipv4Addr};
use std::io::prelude::*;

const PORT: u16 = 6969;

fn main() -> std::io::Result<()> {
    // // Get a output stream handle to the default physical sound device
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // // Load a sound from a file, using a path relative to Cargo.toml
    // let file = BufReader::new(File::open("music.flac").unwrap());
    // // Decode that sound file into a source
    // let source = Decoder::new(file).unwrap();
    //
    // let sink = Sink::try_new(&stream_handle).unwrap();
    //
    // // sink.append(source);


    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(ip, PORT);

    let listener = TcpListener::bind(socket)?;

    // let (mut client, addr) = listener.accept()?;

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
    // println!("asd");


    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    // std::thread::sleep(std::time::Duration::from_secs(5));
    // sink.pause();
    // sink.sleep_until_end();
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()>{
    let mut buffer = String::new();

    while buffer.as_str() != "close" {

        // buffer = String::from("");
        //
        // println!("test");
        buffer = "".to_string();
        match stream.read_to_string(&mut buffer)? {
            // if it returs more then 1 byte
            n if n != 0=> {
               // client.write("hello world".as_bytes())?;
                // print!("{}", &buffer[0..n]);
                println!("{n} {}", &buffer);
                // print!("");
            }
            _ => {}
        }
    }

    Ok(())
}
