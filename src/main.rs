#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]



use ::bevy::prelude::*;
use bevy_egui::EguiPlugin;


pub mod Configs;
pub mod Levels;
pub mod Random;
pub mod Object;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(Configs::ConfigPlugin)
        .add_plugins(Levels::LevelsPlugin)
        .add_systems(Startup, setup)
        .run();
}


pub fn setup(
    mut writer_server: EventWriter<Levels::Company::Server::Server_Place>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn(Camera2dBundle::default());
    
    let server_position = Vec2::new(0f32, 0f32);
    writer_server.send(Levels::Company::Server::Server_Place {
        pos: server_position,
    });

    commands.spawn(AudioBundle {
        source: asset_server.load("music/background.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}
