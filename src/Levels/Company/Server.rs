use ::bevy::prelude::*;

use crate::{Configs::Player::{Layer, PlayerConfig}, Levels::Hardware::{Ethernet, Processor}};



pub struct ServerPlugin;
impl Plugin for ServerPlugin
{
    fn build(&self, app: &mut ::bevy::prelude::App)
    {
        app.add_systems(Update, visibility);
        app.add_event::<Server_Place>();
        app.add_systems(Update, place);
        app.add_event::<Server_Remove>();
        app.add_systems(Update, remove);
    }
}



#[derive(Component)]
pub struct Server
{
    pub processor: Entity,
    pub ethernet : Entity,
}
#[derive(Bundle)]
pub struct ServerBundle
{
    pub server: Server,
    pub sprite: SpriteBundle,
}



pub fn visibility(
    mut query: Query<&mut Visibility, With<Server>>,
    player: Res<PlayerConfig>,
){
    if player.layer == Layer::Company {
        for mut server in query.iter_mut()
        {
            *server = Visibility::Visible;
        }
    } else {
        for mut server in query.iter_mut()
        {
            *server = Visibility::Hidden;
        }
    }
}
#[derive(Event)]
pub struct Server_Place
{
    pub pos: Vec2,
}
pub fn place(
    mut reader: EventReader<Server_Place>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    for event in reader.read() {
        spawn(&mut commands, &asset_server, event.pos);
    }
}
pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: Vec2,
) -> Entity
{
    let processor_id = Processor::spawn(commands, asset_server, pos);
    let ethernet_id = Ethernet::spawn(commands, asset_server, pos + Vec2::new(0., 300.), pos);
    commands.spawn(ServerBundle {
        server: Server {
            processor: processor_id,
            ethernet : ethernet_id,
        },
        sprite: SpriteBundle {
            texture: asset_server.load("images/server.png"),
            visibility: Visibility::Visible,
            transform: Transform {
                translation: pos.extend(3.),
                ..default()
            },
            ..default()
        },
    }).id()
}
#[derive(Event)]
pub struct Server_Remove;
pub fn remove(
    mut reader: EventReader<Server_Remove>,
    mut commands: Commands,
    query: Query<Entity, With<Server>>,
){
    for _ in reader.read() {
        for entity in query.iter()
        {
            commands.entity(entity).despawn();
        }
    }
}