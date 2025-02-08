mod gui;
mod music;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use id3::{Tag, TagLike};
use crate::gui::render;
use crate::music::{get_music, MusicState};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let albums = get_music().unwrap();
    //println!("{:#?}", albums);
    println!("{:#?}", albums.iter().map(|a| &a.title).collect::<Vec<_>>());


    let state = MusicState {albums_1: albums.into_iter().map(|a| a.title).collect(), albums_2: vec![]};

    render(state).unwrap();
    Ok(())
}