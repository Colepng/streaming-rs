use std::{io::Write, usize};

use rodio::OutputStream;
use streaming::{search, Client, playlist::Playlist};
use futures::executor::block_on;

fn main() -> std::io::Result<()> {
    block_on(main_async())
}

async fn main_async() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut client = Client::new(stream_handle).await;

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
                let result = search(input.trim()).await;
                input.clear();

                // TODO improve
                for (index, track) in result.iter().enumerate() {
                    let name = &track.name;
                    let artist = &track.artist;
                    println!("{index} {name} by {artist} ");
                }

                print!("song_number: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;
                let song_number: usize = input.trim().parse::<usize>().unwrap();
                input.clear();

                // maybe serlize FullTrack and pased that throught instead of the id
                client.add_to_queue(result[song_number].clone());
            }
            "search song" => {
                input.clear();

                print!("search: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;

                let songs = client.search_local(input.trim()).await;
                input.clear();

                for (index, song) in songs.iter().enumerate() {
                    println!("{} {} by {}", index, song.name, song.artist);
                }

                print!("song_number: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;
                let song_number: usize = input.trim().parse::<usize>().unwrap();
                input.clear();

                client.add_to_queue(songs[song_number].clone());
            }
            "skip" => {
                input.clear();

                client.skip();
            }
            "prev" => {
                input.clear();

                client.prev();
            }
            "clear" => {
                client.clear();
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
                let current_pos = client.pos();

                for (index, song) in songs.into_iter().enumerate() {
                    if index != current_pos {
                        println!("{} {} by {}", index, song.name, song.artist);
                    } else {
                        println!("{} {} by {} *", index, song.name, song.artist);
                    }
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
            "add current song" => {
                input.clear();

                if let Some(song) = client.current_song() {
                    client.add_song(&song).await;
                    println!("{} by {}", song.name, song.artist);
                } else {
                    println!("no song currently playing");
                }
            }
            "remove song" => {
                input.clear();

                print!("search: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;

                let songs = client.search_local(input.trim()).await;
                input.clear();

                for (index, song) in songs.iter().enumerate() {
                    println!("{} {} by {}", index, song.name, song.artist);
                }

                print!("song_number: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;
                let song_number: usize = input.trim().parse::<usize>().unwrap();
                input.clear();

                client.remove_song(&songs[song_number]).await;
            }
            "list songs" => {
                input.clear();

                let songs = client.search_local("").await;

                for (index, song) in songs.iter().enumerate() {
                    println!("{} {} by {}", index, song.name, song.artist);
                }
            }
            "download local song" => {
                input.clear();

                print!("search: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;

                let songs = client.search_local(input.trim()).await;
                input.clear();

                for (index, song) in songs.iter().enumerate() {
                    println!("{} {} by {}", index, song.name, song.artist);
                }

                print!("song_number: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;
                let song_number: usize = input.trim().parse::<usize>().unwrap();
                input.clear();

                // maybe serlize FullTrack and pased that throught instead of the id
                client.download(&songs[song_number]).await;
            }
            "save current playlist" => {
                input.clear();

                print!("name: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;

                client.save_current_playlist(&input.trim());
                input.clear();
            }
            "load" => {
                input.clear();

                print!("name: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;

                client.load_playlist(&input.trim());
                input.clear();
            }
            "make playlist" => {
                input.clear();
                println!("enter done when finshed entering songs");
                let mut playlist = Playlist::new();

                loop {
                    print!("name: ");
                    stdout.flush()?;

                    stdin.read_line(&mut input)?;
                    if input.trim() == "done" {
                        input.clear();
                        break;
                    }
                    let search_results = client.search_local(input.trim()).await;
                    input.clear();
                    for (index, song) in search_results.iter().enumerate() {
                        println!("{} {} by {}", index, song.name, song.artist);
                    }

                    print!("song_number: ");
                    stdout.flush()?;

                    stdin.read_line(&mut input)?;
                    let song_number: usize = input.trim().parse::<usize>().unwrap();
                    input.clear();

                    playlist.add_to_queue(search_results[song_number].clone());
                }
                print!("playlist name: ");
                stdout.flush()?;

                stdin.read_line(&mut input)?;
                playlist.save(&input);
                input.clear();
            }
            _ => {
                input.clear();
                println!("command not found");
            }
        }
    }
}
