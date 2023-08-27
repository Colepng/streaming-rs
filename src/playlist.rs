use std::io::Cursor;

use rodio::{Decoder, Sink};
use rspotify::model::FullTrack;

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
    pub fn add_to_queue(&mut self, song: &FullTrack) {
        self.songs.push(Song::new(song));
    }
    pub fn remove_from_queue(&mut self, pos: usize, sink: &mut Sink) {
        self.songs.remove(pos);
        if self.pos == pos as isize {
            sink.clear();
            sink.play();
        }
    }
    pub fn play(&mut self, sink: &mut Sink, offset: isize) {
        self.pos += offset;
        println!("pos: {}", self.pos);
        sink.append(
            Decoder::new_mp3(Cursor::new(
                // if i use an rc I might be able to mutate the underlying data for partial loading
                buffer(&self.songs.get(self.pos as usize).expect("invaild postion").id)
                    .expect("buffering failed"),
            ))
            .unwrap(),
        );
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
}
