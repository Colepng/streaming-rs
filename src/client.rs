use std::{io::Write, usize};

use rodio::OutputStream;
use streaming::{search, Client};

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut client = Client::new(stream_handle);

    client.init();

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
                let song_number: usize = input.trim().parse::<usize>().unwrap();
                input.clear();

                // maybe serlize FullTrack and pased that throught instead of the id
                client.add_to_queue(&result[song_number]);
            }
            "skip" => {
                input.clear();

                client.skip();
            }
            "prev" => {
                input.clear();

                client.prev();
            }
            "len" => {
                input.clear();

                println!("{}", client.len());
            }
            "status" => {
                input.clear();
                println!("is paused {}", client.is_paused());
            }
            "pause" => {
                input.clear();

                client.pause();
            }
            "play" => {
                input.clear();

                client.play();
            }
            "toggle" => {
                input.clear();

                client.toggle();
            }
            "set volume" => {
                input.clear();

                print!("volume: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;

                let volume = input.trim().parse::<f32>().unwrap();
                input.clear();

                client.set_volume(volume);
            }
            "get volume" => {
                input.clear();

                println!("volume: {:.2}", client.get_volume());
            }
            "list" => {
                // add indicator to show what song is playing in the playlist
                input.clear();

                let songs = client.get_songs();

                for (index, song) in songs.into_iter().enumerate() {
                    println!("{} {} by {}", index, song.name, song.artist);
                }
            }
            "remove" => {
                input.clear();

                print!("song number: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;
                
                let song_to_removed = input.trim().parse::<usize>().unwrap();
                input.clear();

                client.remove_from_queue(song_to_removed);
            }
            "current song" => {
                input.clear();
                
                if let Some(song) = client.current_song() {
                    println!("{} by {}", song.name, song.artist);
                } else {
                    println!("no song currently playing");
                }
                
            }
            _ => {
                input.clear();
                println!("command not found");
            }
        }
    }
}
