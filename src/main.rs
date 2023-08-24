use rspotify::model::TrackId;
use rspotify::prelude::*;
use rspotify::ClientCredsSpotify;
use rspotify::Credentials;
use std::fs::read_dir;
use std::fs::rename;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::Command;
use std::thread;

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

    let (path, mut spotify) = get_song_path_from_id(&song, spotify);

    if !path.try_exists()? {
        spotify = download(&song, spotify)?;
        move_file(&song, spotify)?;
    }

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
    println!("id {id}");
    let id = TrackId::from_id_or_uri(id).unwrap();
    let track = spotify.track(id).unwrap();

    let name = &track.artists[0].name;
    let album = &track.album.name;
    let song_name = &track.name;

    println!("Music/{name}/{album}/{song_name}.mp3");

    (
        format!("Music/{name}/{album}/{song_name}.mp3").into(),
        spotify,
    )
}

// downloads a song with spotdl
fn download(id: &str, spotify: ClientCredsSpotify) -> std::io::Result<ClientCredsSpotify> {
    let id = TrackId::from_id_or_uri(id).unwrap();
    let track = spotify.track(id).unwrap();

    let link = &track.external_urls.get("spotify").unwrap();

    let mut child = Command::new("spotdl")
        .args([link, "--output", "{title}.{output-ext}", "--format", "mp3"])
        .current_dir("Music")
        .spawn()?;

    child.wait()?;

    Ok(spotify)
}

fn move_file(id: &str, spotify: ClientCredsSpotify) -> std::io::Result<()> {
    let id = TrackId::from_id_or_uri(id).unwrap();
    let track = spotify.track(id).unwrap();

    let artist = &track.artists[0].name;
    let album = &track.album.name;
    let name = &track.name;

    for i in read_dir("Music")? {
        let path = i?.path();
        if path.is_file() {
            DirBuilder::new()
                .recursive(true)
                .create(format!("Music/{artist}/{album}"))?;
            rename(
                format!("Music/{name}.mp3"),
                format!("Music/{artist}/{album}/{name}.mp3"),
            )?;
        }
    }

    Ok(())
}
