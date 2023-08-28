use std::path::{Path, PathBuf};

use rspotify::model::FullTrack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    // track: FullTrack,
    pub id: String,
    pub name: String,
    pub artist: String,
    pub album: String,
}

impl Song {
    pub fn new(song: &FullTrack) -> Self {
        Self {
            // track: song.clone(),
            id: song.id.as_ref().unwrap().to_string(),
            name: song.name.to_owned(),
            artist: song.artists[0].name.to_owned(),
            album: song.album.name.to_owned(),
        }
    }
    pub fn is_downloaded(&self) -> bool {
        // path subject to change
        Path::new(format!("Library/{}/{}/{}.mp3", self.artist, self.album, self.name).as_str()).exists()
    }
    pub fn path(&self) -> PathBuf {
        PathBuf::from(format!("Library/{}/{}/{}.mp3", self.artist, self.album, self.name))
    }
    pub fn dir(&self) -> PathBuf {
        PathBuf::from(format!("Library/{}/{}/", self.artist, self.album))
    }
}
