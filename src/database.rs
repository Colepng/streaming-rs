use smol;
use sqlx::SqlitePool;

use crate::song::Song;

pub fn db() -> SqlitePool {
    smol::block_on(async { SqlitePool::connect("sqlite:library.db").await.unwrap() })
}

pub fn add_song(song: &Song, pool: &SqlitePool) {
    smol::block_on(async {
        let mut conn = pool.acquire().await.unwrap();

        let serialized = bitcode::serialize(song).unwrap();

        sqlx::query!(
            "INSERT INTO library (id, song_binary, name, album, artist)
            VALUES ($1, $2, $3, $4, $5)",
            song.id,
            serialized,
            song.name,
            song.album,
            song.artist,
        )
        .execute(&mut *conn)
        .await
        .unwrap()
    });
}

pub fn search_song(search: &str, pool: &SqlitePool) -> Vec<Song> {
    smol::block_on(async {
        let mut conn = pool.acquire().await.unwrap();

        let search = format!("%{}%", search);
        let results = sqlx::query!(
            "SELECT song_binary FROM library
            WHERE name LIKE $1 or album LIKE $1 or artist LIKE $1",
            search
        )
        .fetch_all(&mut *conn)
        .await
        .unwrap();

        results
            .iter()
            .map(|x| bitcode::deserialize(&x.song_binary).unwrap())
            .collect::<Vec<Song>>()
    })
}

pub fn remove_song(song: &Song, pool: &SqlitePool) {
    smol::block_on(async {
        let mut conn = pool.acquire().await.unwrap();

        sqlx::query!(
            "DELETE from library
            WHERE id=$1",
            song.id,
        )
        .execute(&mut *conn)
        .await
        .unwrap();
    })
}

pub fn song_added(song: &Song, pool: &SqlitePool) -> bool {
    smol::block_on(async {
        let mut conn = pool.acquire().await.unwrap();

        let temp = sqlx::query!(
            "SELECT COUNT(id) as id
            FROM library
            WHERE id=$1",
            song.id,
            ).fetch_all(&mut *conn).await.unwrap();

        temp[0].id == 1
    })
}
