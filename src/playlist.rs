use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};

use rand::seq::SliceRandom;
use rodio::{Decoder, Sink};

use crate::buffer;
use crate::song::Song;

pub struct Playlist {
    pos: isize,
    songs: Vec<Song>,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            pos: -1,
            songs: Vec::new(),
        }
    }
    pub fn add_to_queue(&mut self, song: &Song) {
        self.songs.push(song.clone());
    }
    pub fn remove_from_queue(&mut self, pos: usize, sink: &mut Sink) {
        self.songs.remove(pos);
        if self.pos == pos as isize {
            sink.clear();
            sink.play();
        } else if self.pos > pos as isize {
            self.pos -= 1;
        }
    }
    pub fn play(&mut self, sink: &mut Sink, offset: isize) {
        self.pos += offset;
        let song = &self.songs.get(self.pos as usize).expect("invaild postion");
        if song.is_downloaded() {
            sink.append(
                Decoder::new_mp3(BufReader::new(File::open(song.path()).unwrap())).unwrap(),
            );
        } else {
            let temp = buffer(song).unwrap();
            sink.append(
                Decoder::new_mp3(
                    // if i use an rc I might be able to mutate the underlying data for partial loading
                    Cursor::new(temp),
                )
                .unwrap(),
            );
        };
    }
    pub fn play_next(&mut self, sink: &mut Sink) {
        self.play(sink, 1);
    }
    pub fn if_play(&self, sink: &Sink) -> bool {
        sink.len() == 0 && self.songs.len() > (self.pos + 1) as usize
    }
    pub fn skip(&mut self, sink: &mut Sink) {
        if self.songs.len() > (self.pos + 1) as usize {
            self.play(sink, 1);
            sink.skip_one();
        }
    }
    pub fn prev(&mut self, sink: &mut Sink) {
        if self.pos - 1 >= 0 {
            self.play(sink, -1);
            sink.skip_one();
        }
    }
    pub fn len(&self) -> usize {
        self.songs.len()
    }
    pub fn get_songs(&self) -> Vec<Song> {
        self.songs.clone()
    }
    pub fn current_song(&self) -> Option<Song> {
        if self.songs.len() != 0 {
            Some(self.songs[self.pos as usize].clone())
        } else {
            None
        }
    }
    pub fn get_pos(&self) -> usize {
        self.pos as usize
    }
    pub fn save(&self, name: &str) {
        let bytes = bitcode::serialize(&self.songs).unwrap();

        let mut file = File::create(format!("Playlist/{name}")).unwrap();
        file.write(&bytes).unwrap();
    }
    pub fn load(name: &str) -> Self {
        let mut file = File::open(format!("Playlist/{name}")).unwrap();
        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes).unwrap();

        let songs = bitcode::deserialize(&bytes).unwrap();

        Self { pos: -1, songs }
    }
    pub fn load_songs(&mut self, songs: Vec<Song>) {
        self.songs = songs;
        self.pos = -1;
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.songs.shuffle(&mut rng);
    }
}
