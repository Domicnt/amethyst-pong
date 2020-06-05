//import things from amethyst
use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::transform::{
        TransformBundle
    }
};

//opens file of name "pong.rs" as a module
mod pong;
//imports Pong struct
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    //start error logging
    amethyst::start_logger(Default::default());
    
    //set application root directory
    let app_root = application_root_dir()?;
    //display config file
    let display_config_path = app_root.join("config").join("display.ron");

    //game logic setup
    let game_data = GameDataBuilder::default()
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()),
    )?
    .with_bundle(
        TransformBundle::new()
    )?;

    //assets directory
    let assets_dir = app_root.join("assets");
    
    //game
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    //run the game
    game.run();

    Ok(())
}