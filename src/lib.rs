use std::io::{Read, Write};
use std::net::TcpStream;

use rspotify::model::{SearchResult, FullTrack};
use rspotify::prelude::BaseClient;
use rspotify::{ClientCredsSpotify, Credentials};

pub fn buffer(song: &str) -> std::io::Result<Vec<u8>> {
    let mut stream = TcpStream::connect("127.0.0.1:6969")?;

    println!("id: {song}");
    stream.write(song.as_bytes())?;

    let mut buffer: Vec<u8> = Vec::new();

    stream.read_to_end(&mut buffer)?;

    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(buffer)
}

// fn search(input: &str) -> Vec<(String, Option<TrackId<'static>>)>{
pub fn search(input: &str) -> Vec<FullTrack> {
    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().unwrap();

    let mut names: Vec<FullTrack> = Vec::new();
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
            names = n.items;
        }
        _ => {}
    }

    names
}
