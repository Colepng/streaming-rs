use std::io::{Cursor, Read, Write};
use std::net::TcpStream;
use std::rc::Rc;

use rodio::{Decoder, OutputStream, Sink};
use rspotify::model::{SearchResult, FullTrack};
use rspotify::prelude::BaseClient;
use rspotify::{ClientCredsSpotify, Credentials};

fn main() -> std::io::Result<()> {
    // Get a output stream handle to the default physical sound device
    // Arc<Sink>
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    let sink = Sink::try_new(&stream_handle).unwrap();

    let sink = Rc::new(sink);

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut input = String::new();

    loop {
        print!("cmd: ");
        stdout.flush()?;

        stdin.read_line(&mut input)?;
        match input.trim() {
            "search" => {
                input.clear();

                print!("search: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;
                let result = search(input.trim());
                input.clear();

                // TODO improve
                for (index, track) in result.iter().enumerate() {
                    let name = &track.name;
                    let artist = &track.artists[0].name;
                    println!("{index} {name} by {artist} ");
                }

                print!("song_number: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;
                print!("{}", input);
                let song_number: usize = input.trim().parse::<usize>().unwrap();
                input.clear();

                // maybe serlize FullTrack and pased that throught instead of the id
                let song_id = &result[song_number].id.to_owned().unwrap().to_string();
                // if i use an rc I might be able to mutate the underlying data for partial loading
                let song = buffer(&song_id)?;

                let source = Decoder::new_mp3(Cursor::new(song.clone())).unwrap();

                sink.play();
                sink.append(source);
                println!("{}", sink.len());
            }
            "skip" => {
                input.clear();
 
                println!("sorry broken");
                // if sink.len() > 1 {
                //     println!("{}", sink.len());
                //     sink.skip_one();
                //     println!("{}", sink.len());
                //     sink.pause();
                // } else {
                //     //sink.clear();
                // }
            }
            "temp" => {
                input.clear();

                println!("{}", sink.len());
            }
            "status" => {
                input.clear();
                println!("is paused {}", sink.is_paused());
            }
            "pause" => {
                input.clear();

                sink.pause();
            }
            "play" => {
                input.clear();

                sink.play();
            }
            "set volume" => {
                input.clear();

                print!("volume: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;

                let volume = input.trim().parse::<f32>().unwrap();
                input.clear();

                sink.set_volume(volume);
            }
            "get volume" => {
                input.clear();

                println!("volume: {:.2}", sink.volume());
            }
            _ => {
                input.clear();
                println!("command not found");
            }
        }
    }
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
fn search(input: &str) -> Vec<FullTrack> {
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
