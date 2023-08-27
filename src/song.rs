use rspotify::model::FullTrack;

#[derive(Debug, Clone)]
pub struct Song {
    track: FullTrack,
    pub id: String,
    pub name: String,
    pub artist: String,
    pub album: String,
}

impl Song {
    pub fn new(song: &FullTrack) -> Self {
        Self {
            track: song.clone(),
            id: song.id.as_ref().unwrap().to_string(),
            name: song.name.to_owned(),
            artist: song.artists[0].name.to_owned(),
            album: song.album.name.to_owned(),
        }
    }
}
