// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(once_cell_try)]
#![feature(let_chains)]

use std::sync::{Once, RwLock};

use rodio::{OutputStream, OutputStreamHandle};
use streaming::{Client, song::Song};

static CLIENT: RwLock<Option<Client>> = RwLock::new(None);

static INIT: Once = Once::new();
static mut STREAM: Option<(OutputStream, OutputStreamHandle)> = None;

#[tauri::command]
fn hello() -> String {
    let client_ref = CLIENT.read().unwrap();
    let client = client_ref.as_ref().unwrap();
    format!("{}", client.get_volume())
}

#[tauri::command]
async fn search(input: String) -> Result<Vec<Song>, ()> {
    streaming::search(input).await.map_err(|_| ())
}

#[tauri::command]
fn toggle() {
    let mut client_ref = CLIENT.write().unwrap();
    let client = client_ref.as_mut().unwrap();
    client.toggle();
}

#[tauri::command]
fn prev() {
    let mut client_ref = CLIENT.write().unwrap();
    let client = client_ref.as_mut().unwrap();
    client.prev();
}

#[tauri::command]
fn skip() {
    let mut client_ref = CLIENT.write().unwrap();
    let client = client_ref.as_mut().unwrap();
    client.skip();
}


#[tauri::command]
fn get_songs() -> Vec<Song> {
    let client_ref = CLIENT.read().unwrap();
    let client = client_ref.as_ref().unwrap();
    client.get_songs()
}


#[tauri::command]
fn play_song(
    id: String,
    name: String,
    artist: String,
    album: String,
    ) {
    let mut client_ref = CLIENT.write().unwrap();
    let client = client_ref.as_mut().unwrap();
    println!("{id}");
    let song = Song { 
        id,
        name,
        artist,
        album,
    };
    client.add_to_queue(&song);
    let songs = client.get_songs();

    for i in songs {
        println!("song name: {}", i.name)
    }
}

fn main() {
    // init the stream
    unsafe {
        STREAM = Some(OutputStream::try_default().unwrap());
    }

    tauri::Builder::default()
        .setup(|_| {
                INIT.call_once(|| {

                    let temp = unsafe { &STREAM.as_ref().unwrap().1 };
                    let mut client_mut = CLIENT.write().unwrap();
                    *client_mut = tauri::async_runtime::block_on( async { 
                        Some(Client::new(temp).await)
                    });

                    client_mut.as_mut().unwrap().init();
                });
                Ok(())
        })
        .invoke_handler(tauri::generate_handler![hello, search, play_song, prev, toggle, skip, get_songs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
