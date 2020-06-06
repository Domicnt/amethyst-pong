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
    },
    input::{
        InputBundle, 
        StringBindings
    }
};

//opens file of name "pong.rs" as a module
mod pong;
//imports Pong struct
use crate::pong::Pong;

//opens mod.rs in systems folder
mod systems;

fn main() -> amethyst::Result<()> {
    //start error logging
    amethyst::start_logger(Default::default());
    
    //set application root directory
    let app_root = application_root_dir()?;
    //display config file
    let display_config_path = app_root.join("config").join("display.ron");
    //binding config file
    let binding_path = app_root.join("config").join("bindings.ron");

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
    .with_bundle(TransformBundle::new())?
    .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?)?
    .with(systems::PaddleSystem, "paddle_system", &["input_system"])
    .with(systems::MoveBallsSystem, "ball_system", &[])
    .with(
        systems::BounceSystem,
        "collision_system",
        &["paddle_system", "ball_system"],
    );

    //assets directory
    let assets_dir = app_root.join("assets");
    
    //game
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    //run the game
    game.run();

    Ok(())
}