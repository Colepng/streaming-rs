use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};

use rand::seq::SliceRandom;
use rodio::{Decoder, Sink};

use crate::buffer;
use crate::song::Song;

pub struct Playlist {
    pos: Option<usize>,
    songs: Vec<Song>,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            pos: None,
            songs: Vec::new(),
        }
    }
    pub fn add_to_queue(&mut self, song: &Song) {
        self.songs.push(song.clone());
    }
    pub fn remove_from_queue(&mut self, pos: usize, sink: &mut Sink) {
        if let Some(self_pos) = self.pos.as_mut() {
            self.songs.remove(pos);
            if *self_pos == pos {
                sink.clear();
                sink.play();
            } else if *self_pos > pos {
                *self_pos -= 1;
            }

            if *self_pos == 0 {
                self.pos = None;
            }
        }
    }

    pub fn play(&mut self, sink: &mut Sink, index: usize) {
        if let Some(pos) = self.pos.as_mut() {
            *pos = index;
            let song = &self.songs.get(*pos).expect("invaild postion");
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
    }

    pub fn play_next(&mut self, sink: &mut Sink) {
        if let Some(pos) = self.pos {
            self.play(sink, pos + 1);
        } else {
            self.pos = Some(0);
            self.play(sink, 0);
        }
    }
    pub fn if_play(&mut self, sink: &Sink) -> bool {
        if let Some(pos) = self.pos {
            sink.len() == 0 && self.songs.len() > pos + 1
        } else {
            sink.len() == 0 && self.songs.len() > 0
        }
    }
    pub fn skip(&mut self, sink: &mut Sink) {
        if let Some(pos) = self.pos {
            if self.songs.len() > pos + 1 {
                self.play(sink, pos + 1);
                sink.skip_one();
            }
        }
    }
    pub fn prev(&mut self, sink: &mut Sink) {
        if let Some(pos) = self.pos {
            if pos != 0 {
                self.play(sink, pos - 1);
                sink.skip_one();
            }
        }
    }
    pub fn len(&self) -> usize {
        self.songs.len()
    }
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
    pub fn get_songs(&self) -> Vec<Song> {
        self.songs.clone()
    }
    pub fn current_song(&self) -> Option<Song> {
        if let Some(pos) = self.pos {
            Some(self.songs[pos].clone())
        } else {
            None
        }
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

        Self { pos: None, songs }
    }
    pub fn load_songs(&mut self, songs: Vec<Song>) {
        self.songs = songs;
        self.pos = None;
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.songs.shuffle(&mut rng);
    }
}
