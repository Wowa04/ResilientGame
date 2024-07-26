use ::bevy::prelude::*;
use crate::
{
    Configs::Player::{Layer, PlayerConfig},
    Levels::Software::{Internet, Packet::*}, Object::*,
};



pub struct EthernetPlugin;
impl Plugin for EthernetPlugin
{
    fn build(&self, app: &mut ::bevy::prelude::App)
    {
        if !app.is_plugin_added::<ObjectPlugin>() {
            app.add_plugins(ObjectPlugin);
        }
        app.add_systems(Update, visibility);
        app.add_systems(Update, update_position);
        app.add_event::<Ethernet_Place>();
        app.add_systems(Update, place);
        app.add_event::<Ethernet_Remove>();
        app.add_systems(Update, remove);
    }
}


#[derive(Component)]
pub struct Ethernet
{
    pub internet: Entity,
}
pub const Ethernet_Hight: f32 = Packet_Hight + 1.;
#[derive(Bundle)]
pub struct EthernetBundle
{
    pub ethernet: Ethernet,
    pub time    : crate::Object::Time,
    pub object  : ObjectBundle,
    pub sprite  : SpriteBundle,
}




pub fn visibility(
    mut query: Query<&mut Visibility, With<Ethernet>>,
    player: Res<PlayerConfig>,
){
    if player.layer == Layer::Hardware {
        for mut ethernet in query.iter_mut()
        {
            *ethernet = Visibility::Visible;
        }
    } else {
        for mut ethernet in query.iter_mut()
        {
            *ethernet = Visibility::Hidden;
        }
    }
}
pub fn update_position(mut query: Query<(&mut Transform, &Position), With<Ethernet>>)
{
    for mut entity in query.iter_mut()
    {
        entity.0.translation = entity.1.position.extend(Ethernet_Hight);
    }
}



#[derive(Event)]
pub struct Ethernet_Place
{
    pub pos: Vec2,
    pub to : Vec2,
}
pub fn place(mut reader: EventReader<Ethernet_Place>, mut commands: Commands, asset_server: Res<AssetServer>)
{
    for event in reader.read() {
        spawn(&mut commands, &asset_server, event.pos, event.to);
    }
}
pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: Vec2,
    to : Vec2,
) -> Entity
{
    let internet_id = Internet::spawn(commands, asset_server, pos, to);
    commands.spawn(EthernetBundle {
        ethernet: Ethernet {
            internet: internet_id,
        },
        time: crate::Object::Time {time: 0f32},
        object: ObjectBundle {
            position   : Position    {position   : pos},
            speed      : Speed       {speed      : 0f32},
            destination: Destination {destination: pos},
        },
        sprite: SpriteBundle {
            texture: asset_server.load("images/ethernet.png"),
            transform: Transform {
                scale: Vec3::new(0.1, 0.1, 1f32),
                ..default()
            },
            ..default()
        },
    }).id()
}
#[derive(Event)]
pub struct Ethernet_Remove;
pub fn remove(mut reader: EventReader<Ethernet_Remove>, mut commands: Commands, query: Query<Entity, With<Ethernet>>)
{
    for _ in reader.read() {
        for entity in query.iter()
        {
            commands.entity(entity).despawn();
        }
    }
}
