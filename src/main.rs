//import things from amethyst
use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

//game state struct
pub struct Pong;
impl SimpleState for Pong {}

fn main() -> amethyst::Result<()> {
    //start error logging
    amethyst::start_logger(Default::default());
    
    //set application root directory
    let app_root = application_root_dir()?;
    //display config file
    let display_config_path = app_root.join("config").join("display.ron");

    //default game logic setup
    let game_data = GameDataBuilder::default();

    //assets directory
    let assets_dir = app_root.join("assets");
    
    //game
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    //run the game
    game.run();

    Ok(())
}