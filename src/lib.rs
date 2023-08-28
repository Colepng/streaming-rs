use std::fs::{File, DirBuilder};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

use database::{add_song, remove_song, search_song, song_added};
use playlist::Playlist;
use rodio::{OutputStreamHandle, Sink};
use rspotify::model::{FullTrack, SearchResult};
use rspotify::prelude::BaseClient;
use rspotify::{ClientCredsSpotify, Credentials};
use song::Song;

use sqlx::SqlitePool;

mod database;
pub mod playlist;
pub mod song;

pub fn buffer(song: &Song) -> std::io::Result<Vec<u8>> {
    let mut stream = TcpStream::connect("127.0.0.1:6969")?;

    stream.write(song.id.as_bytes())?;

    let mut buffer: Vec<u8> = Vec::new();

    stream.read_to_end(&mut buffer)?;

    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(buffer)
}

// fn search(input: &str) -> Vec<(String, Option<TrackId<'static>>)>{
pub fn search(input: &str) -> Vec<Song> {
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
        .into_iter()
        .map(|x| Song::new(&x))
        .collect::<Vec<Song>>()
}

pub enum DownloadError {
    SongAlreadyDownloaded,
    SongNotInLibrary,
    DownloadingError,
}

pub struct Client {
    playlist: Arc<Mutex<Playlist>>,
    sink: Arc<Mutex<Sink>>,
    db: SqlitePool,
}

impl Client {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        Client {
            playlist: Arc::new(Mutex::new(Playlist::new())),
            sink: Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap())),
            db: database::db(),
        }
    }
    pub fn init(&mut self) {
        let playlist = self.playlist.clone();
        let sink = self.sink.clone();
        thread::spawn(move || loop {
            let mut playlist = playlist.lock().unwrap();
            let mut sink = sink.lock().unwrap();
            if playlist.if_play(&*sink) {
                playlist.play_next(&mut *sink);
            }
        });
    }
    pub fn add_to_queue(&mut self, song: Song) {
        let mut playlist = self.playlist.lock().unwrap();
        playlist.add_to_queue(song);
    }
    pub fn remove_from_queue(&mut self, pos: usize) {
        let mut playlist = self.playlist.lock().unwrap();
        let mut sink = self.sink.lock().unwrap();
        playlist.remove_from_queue(pos, &mut sink);
    }
    pub fn play_next(&mut self) {
        let mut playlist = self.playlist.lock().unwrap();
        let mut sink = self.sink.lock().unwrap();
        playlist.play_next(&mut *sink);
    }
    pub fn is_paused(&self) -> bool {
        let sink = self.sink.lock().unwrap();
        sink.is_paused()
    }
    pub fn pause(&self) {
        let sink = self.sink.lock().unwrap();
        sink.pause();
    }
    pub fn play(&self) {
        let sink = self.sink.lock().unwrap();
        sink.play();
    }
    pub fn toggle(&mut self) {
        let sink = self.sink.lock().unwrap();
        if sink.is_paused() {
            sink.play();
        } else {
            sink.pause();
        }
    }
    pub fn skip(&mut self) {
        let mut playlist = self.playlist.lock().unwrap();
        let mut sink = self.sink.lock().unwrap();
        playlist.skip(&mut *sink);
    }
    pub fn prev(&mut self) {
        let mut playlist = self.playlist.lock().unwrap();
        let mut sink = self.sink.lock().unwrap();
        playlist.prev(&mut *sink);
    }
    pub fn clear(&mut self) {
        let sink = self.sink.lock().unwrap();
        sink.clear();
        sink.play();
    }
    pub fn get_volume(&self) -> f32 {
        let sink = self.sink.lock().unwrap();
        sink.volume()
    }
    pub fn set_volume(&mut self, volume: f32) {
        let sink = self.sink.lock().unwrap();
        sink.set_volume(volume);
    }
    pub fn len(&self) -> usize {
        let playlist = self.playlist.lock().unwrap();
        playlist.len()
    }
    pub fn get_songs(&self) -> Vec<Song> {
        let playlist = self.playlist.lock().unwrap();
        playlist.get_songs()
    }
    pub fn current_song(&self) -> Option<Song> {
        let playlist = self.playlist.lock().unwrap();
        playlist.current_song()
    }
    pub fn pos(&self) -> usize {
        let playlist = self.playlist.lock().unwrap();
        playlist.get_pos()
    }
    pub fn add_song(&mut self, song: &Song) {
        add_song(song, &self.db);
    }
    pub fn remove_song(&mut self, song: &Song) {
        remove_song(song, &self.db);
    }
    pub fn search_local(&mut self, search: &str) -> Vec<Song> {
        search_song(search, &self.db)
    }
    pub fn download(&self, song: &Song) -> Result<(), DownloadError> {
        if !song_added(song, &self.db) {
            Err(DownloadError::SongNotInLibrary)
        } else if !song.is_downloaded() {
            match buffer(song) {
                Ok(bytes) => {
                    DirBuilder::new()
                        .recursive(true)
                        .create(song.dir()).unwrap();
                    let mut file = File::create(song.path()).unwrap();
                    file.write(&bytes).unwrap();
                    Ok(())
                }
                _ => {
                    Err(DownloadError::DownloadingError)
                }
            }
        } else {
            Err(DownloadError::SongAlreadyDownloaded)
        }
    }
}

// bug tracker
// adding song alrady added
// 100% cpu
// optomizartons
// storing if a song is downlaod in db to cut down and io calls
