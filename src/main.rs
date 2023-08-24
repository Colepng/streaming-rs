use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::thread;
use rspotify::ClientCredsSpotify;
use rspotify::model::TrackId;
use rspotify::Credentials;
use rspotify::prelude::*;

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
    // maybe an Arc<spotify>
    let mut song_buffer = [0u8; 100];
    let num_bytes_read = stream.read(&mut song_buffer)?;
    let song = String::from_utf8(song_buffer[0..num_bytes_read].to_vec()).unwrap();

    // setup spotify api
    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().unwrap();

    let (path, _spotify) = get_song_path_from_id(&song, spotify);

    // get_song_path(&song)?;
    println!("{:#?}", path);
    let mut file = BufReader::new(File::open(path)?);
    
    let mut buffer: Vec<u8> = Vec::new();
    
    file.read_to_end(&mut buffer)?;
    stream.write(&buffer)?;
    
    println!("done");
    
    Ok(())
}

fn get_song_path_from_id(id: &str, spotify: ClientCredsSpotify) -> (PathBuf, ClientCredsSpotify) {
    let id = TrackId::from_id(id).unwrap();
    let track = spotify.track(id).unwrap();

    let name = &track.artists[0].name;
    let album = &track.album.name;
    let song_name = &track.name;

    println!("Music/{name}/{album}/{song_name}.mp3");
    (format!("Music/{name}/{album}/{song_name}.mp3").into(), spotify)
}
