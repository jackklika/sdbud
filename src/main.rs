mod gui;
mod music;
mod findsd;
mod error;

use id3::{Tag, TagLike};
use tokio::runtime::Runtime;
use crate::findsd::list_sd_cards;
use crate::gui::render;
use crate::music::{get_music, MusicState};

pub type Result<T> = std::result::Result<T, error::Error>;


fn main() -> Result<()> {

    // Create the tokio runtime manually
    // https://github.com/emilk/egui/discussions/521#discussioncomment-3462382

    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                // todo make this some device daemon
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        })
    });

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