use std::fs::{self, DirBuilder, File};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use database::{add_song, remove_song, search_song, song_added};
use playlist::Playlist;
use rodio::{OutputStreamHandle, Sink};
use rspotify::model::{FullTrack, SearchResult};
use rspotify::prelude::BaseClient;
use rspotify::{ClientCredsSpotify, Credentials};
use song::Song;

use sqlx::SqlitePool;

mod database;
// somehow hide all but save and new
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

#[derive(Debug)]
pub enum SearchError {
    Timeout,
    RequestToken,
}
// fn search(input: &str) -> Vec<(String, Option<TrackId<'static>>)>{
pub async fn search(input: String) -> Result<Vec<Song>, SearchError> {
    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);
    if spotify.request_token().is_ok() {
        let mut names: Vec<FullTrack> = Vec::new();
        let search_resault = spotify
            .search(
                input.as_str(),
                rspotify::model::SearchType::Track,
                None,
                None,
                None,
                None,
            )
            .map_err(|_| SearchError::Timeout)?;
        match search_resault {
            SearchResult::Tracks(n) => {
                names = n.items;
            }
            _ => {}
        }

        Ok(names
            .into_iter()
            .map(|x| Song::new(&x))
            .collect::<Vec<Song>>())
    } else {
        Err(SearchError::RequestToken)
    }
}

pub enum DownloadError {
    SongAlreadyDownloaded,
    SongNotInLibrary,
    DownloadingError,
}

#[derive(Clone)]
pub struct Client {
    playlist: Arc<Mutex<Playlist>>,
    sink: Arc<Mutex<Sink>>,
    db: SqlitePool,
}

impl Client {
    pub async fn new(stream_handle: &OutputStreamHandle) -> Self {
        Client {
            playlist: Arc::new(Mutex::new(Playlist::new())),
            sink: Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap())),
            db: database::db().await,
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
            drop(sink);
            drop(playlist);
            thread::sleep(Duration::from_millis(10));
        });
    }
    pub fn add_to_queue(&mut self, song: &Song) {
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
    pub fn play_n(&mut self, index: usize) {
        let mut playlist = self.playlist.lock().unwrap();
        let mut sink = self.sink.lock().unwrap();

        playlist.play(&mut sink, index);
        sink.skip_one();
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
    pub fn pos(&self) -> Option<usize> {
        let playlist = self.playlist.lock().unwrap();
        playlist.pos()
    }
    pub async fn add_song(&mut self, song: &Song) {
        add_song(song, &self.db).await;
    }
    pub async fn remove_song(&mut self, song: &Song) {
        remove_song(song, &self.db).await;
    }
    pub async fn search_local(&mut self, search: &str) -> Vec<Song> {
        search_song(search, &self.db).await
    }
    pub async fn download(&self, song: &Song) -> Result<(), DownloadError> {
        if !song_added(song, &self.db).await {
            Err(DownloadError::SongNotInLibrary)
        } else if !song.is_downloaded() {
            match buffer(song) {
                Ok(bytes) => {
                    DirBuilder::new()
                        .recursive(true)
                        .create(song.dir())
                        .unwrap();
                    let mut file = File::create(song.path()).unwrap();
                    file.write(&bytes).unwrap();
                    Ok(())
                }
                _ => Err(DownloadError::DownloadingError),
            }
        } else {
            Err(DownloadError::SongAlreadyDownloaded)
        }
    }
    pub fn delete(&self, song: &Song) -> io::Result<()> {
        fs::remove_file(song.path())
    }
    pub fn save_current_playlist(&self, name: &str) {
        let playlist = self.playlist.lock().unwrap();

        playlist.save(name);
    }
    pub fn load_playlist(&mut self, name: &str) {
        let mut playlist = self.playlist.lock().unwrap();
        let sink = self.sink.lock().unwrap();
        *playlist = Playlist::load(name);
        sink.clear();
        sink.play();
    }
    pub async fn shuffle_all_songs(&mut self) {
        let songs = self.search_local("").await;
        let mut playlist = self.playlist.lock().unwrap();
        let sink = self.sink.lock().unwrap();
        playlist.load_songs(songs);
        playlist.shuffle();
        sink.clear();
        sink.play();
    }
}

// bug tracker
// adding song already added
// 100% CPU usage
// crashes if first char in search is a space fix by using .trim_start() or .trim() on search
// query
// optimizations
// storing if a song is downloaded in db to cut down on I/O calls
