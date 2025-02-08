
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use id3::{Tag, TagLike};
use crate::gui::render;

#[derive(Debug)]
struct Song {
    order: Option<u32>,
    title: String,
    path: PathBuf,
    artist: Option<String>,
    album: Option<String>,
}

#[derive(Debug)]
pub struct Album {
    pub title: String,
    songs: Vec<Song>,
    cover: Option<PathBuf>,
}

pub struct MusicState {
    pub albums_1: Vec<String>,
    pub albums_2: Vec<String>,
}

impl Default for MusicState {
    fn default() -> Self {MusicState{albums_1: vec![], albums_2: vec![]}}
}

pub fn get_music() -> Result<Vec<Album>, Box<dyn std::error::Error>> {
    let base_dir = env::args().nth(1).unwrap_or_else(|| ".".into());
    let mut albums = Vec::new();

    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            albums.push(process_album(&path)?);
        }
    }

    Ok(albums)
}

fn process_album(dir: &Path) -> Result<Album, Box<dyn std::error::Error>> {
    let title = dir.file_name().unwrap().to_string_lossy().into_owned();
    let mut songs = Vec::new();
    let mut cover = None;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                match ext.to_lowercase().as_str() {
                    "mp3" => songs.push(process_song(&path)?),
                    "jpg" | "png" if cover.is_none() => cover = Some(path.clone()),
                    _ => {}
                }
            }
        }
    }
    songs.sort_by_key(|s| s.order.unwrap_or(0));
    Ok(Album { title, songs, cover })
}

fn process_song(path: &Path) -> Result<Song, Box<dyn std::error::Error>> {
    let file_name = path.file_name().unwrap().to_string_lossy();
    let order = file_name
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .ok();
    let mut title = file_name.to_string();
    let mut artist = None;
    let mut album = None;

    if let Ok(tag) = Tag::read_from_path(path) {
        if let Some(t) = tag.title() {
            title = t.to_string();
        }
        artist = tag.artist().map(String::from);
        album = tag.album().map(String::from);
    }

    Ok(Song {
        order,
        title,
        path: path.to_path_buf(),
        artist,
        album,
    })
}

