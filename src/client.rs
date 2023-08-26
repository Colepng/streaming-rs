use std::io::{Cursor, Write};

use rodio::{Decoder, OutputStream, Sink};

mod playlist;

use streaming::{buffer, search};

fn main() -> std::io::Result<()> {
    // Get a output stream handle to the default physical sound device
    // Arc<Sink>
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    let sink = Sink::try_new(&stream_handle).unwrap();

    let sink = sink;

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
                let song_number: usize = input.trim().parse::<usize>().unwrap();
                input.clear();

                // maybe serlize FullTrack and pased that throught instead of the id
                let song_id = &result[song_number].id.to_owned().unwrap().to_string();
                // if i use an rc I might be able to mutate the underlying data for partial loading
                let song = buffer(&song_id)?;

                let source = Decoder::new(Cursor::new(song.clone())).unwrap();

                sink.append(source);
            }
            "skip" => {
                input.clear();
 
                sink.skip_one();
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
