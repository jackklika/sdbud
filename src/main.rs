mod gui;
mod music;
mod findsd;
mod error;

use id3::{Tag, TagLike};
use crate::findsd::list_sd_cards;
use crate::gui::render;
use crate::music::{get_music, MusicState};

pub type Result<T> = std::result::Result<T, error::Error>;

#[tokio::main]
async fn main() -> Result<()> {

    let sd_cards = list_sd_cards()?;
    println!("sd cards:");
    for card in sd_cards {
        println!("{:?}", card);
    }

    let albums = get_music().unwrap();
    //println!("{:#?}", albums);
    println!("{:#?}", albums.iter().map(|a| &a.title).collect::<Vec<_>>());


    let state = MusicState {albums_1: albums.into_iter().map(|a| a.title).collect(), albums_2: vec![]};

    render(state).unwrap();
    Ok(())
}