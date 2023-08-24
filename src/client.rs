use std::io::{Cursor, Read, Write};
use std::net::TcpStream;

use rodio::{Decoder, OutputStream, Sink};
use rspotify::model::SearchResult;
use rspotify::prelude::BaseClient;
use rspotify::{ClientCredsSpotify, Credentials};

fn main() -> std::io::Result<()> {
    // Get a output stream handle to the default physical sound device
    // Arc<Sink>
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut input = String::new();
    print!("search: ");
    stdout.flush()?;

    stdin.read_line(&mut input)?;

    let result = search(input.as_str());

    for (index, (name, _)) in result.iter().enumerate() {
        println!("{index} {name}");
    }

    let mut input = String::new();
    print!("Enter the number of song to play: ");
    stdout.flush()?;

    stdin.read_line(&mut input)?;

    let song_number: usize = input.trim().parse::<usize>().unwrap();

    let song = buffer(result[song_number].1.as_str())?;

    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = Decoder::new_mp3(Cursor::new(song.clone())).unwrap();

    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}

fn buffer(song: &str) -> std::io::Result<Vec<u8>> {
    let mut stream = TcpStream::connect("127.0.0.1:6969")?;

    println!("id: {song}");
    stream.write(song.as_bytes())?;

    let mut buffer: Vec<u8> = Vec::new();

    stream.read_to_end(&mut buffer)?;

    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(buffer)
}

// fn search(input: &str) -> Vec<(String, Option<TrackId<'static>>)>{
fn search(input: &str) -> Vec<(String, String)> {
    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().unwrap();

    let mut names: Vec<(String, String)> = Vec::new();
    let search_resault = spotify
        .search(
            input,
            rspotify::model::SearchType::Track,
            None,
            None,
            None,
            None,
        )
        .unwrap();
    match search_resault {
        SearchResult::Tracks(n) => {
            names.reserve(n.items.len());
            for i in n.items {
                names.push((i.name, i.id.unwrap().into_static().to_string()));
            }
        }
        _ => {}
    }

    names
}
