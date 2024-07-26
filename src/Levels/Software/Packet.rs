use ::bevy::prelude::*;
use crate::
{
    Configs::{Player::{Layer, PlayerConfig}, World::WorldConfig}, Object::*
};



pub struct PacketPlugin;
impl Plugin for PacketPlugin
{
    fn build(&self, app: &mut ::bevy::prelude::App)
    {
        if !app.is_plugin_added::<ObjectPlugin>() {
            app.add_plugins(ObjectPlugin);
        }
        app.add_systems(Update, visibility);
        app.add_systems(Update, update);
        app.add_event::<Packet_Place>();
        app.add_systems(Update, place);
        app.add_event::<Packet_Remove>();
        app.add_systems(Update, remove);
    }
}



#[derive(Component)]
pub struct Packet
{
    pub from : Vec2,
}
pub const Packet_Hight: f32 = 0f32;
#[derive(Component, Clone, Copy)]
pub enum PacketType
{
    User,
    Malware,
    DDoS,
    Reply,
}
#[derive(Bundle)]
pub struct PacketBundle
{
    pub packet: Packet,
    pub p_type: PacketType,
    pub object: ObjectBundle,
    pub sprite: SpriteBundle,
}
#[derive(Event)]
pub struct Packet_Place
{
    pub pos  : Vec2,
    pub to   : Vec2,
    pub ptype: PacketType,
}
pub fn update(mut query: Query<(&mut Transform, &Position, &mut Speed), With<Packet>>, world: Res<WorldConfig>)
{
    for mut entity in query.iter_mut()
    {
        entity.0.translation = entity.1.position.extend(Packet_Hight);
        entity.2.speed       = world.get_packet_speed();
    }
}
pub fn visibility(
    mut query: Query<(&mut Visibility, &mut Handle<Image>, &PacketType), With<Packet>>,
    asset_server: Res<AssetServer>,
    player: Res<PlayerConfig>,
){
    match player.layer
    {
        Layer::Software =>
        {
            for mut packet in query.iter_mut()
            {
                *packet.0 = Visibility::Visible;
                *packet.1 = asset_server.load(match packet.2 {
                    PacketType::User    => "images/packet_user.png",
                    PacketType::Malware => "images/packet_malware.png",
                    PacketType::DDoS    => "images/packet_ddos.png",
                    PacketType::Reply   => "images/packet_reply.png",
                });
            }
        },
        Layer::Hardware => 
        {
            for mut packet in query.iter_mut()
            {
                *packet.0 = Visibility::Visible;
                *packet.1 = asset_server.load("images/packet_data.png");
            }
        },
        Layer::Company => {
            for mut packet in query.iter_mut()
            {
                *packet.0 = Visibility::Hidden;
            }
        },
    } 
}



pub fn place(
    mut reader: EventReader<Packet_Place>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world: Res<WorldConfig>,
){
    for event in reader.read() {
        commands.spawn(PacketBundle {
            packet: Packet{from: event.pos},
            p_type: event.ptype,
            object: ObjectBundle {
                position   : Position    {position   : event.pos},
                speed      : Speed       {speed      : world.get_packet_speed()},
                destination: Destination {destination: event.to},
            },
            sprite: SpriteBundle {
                texture: asset_server.load(match event.ptype {
                    PacketType::User    => "images/packet_user.png",
                    PacketType::Malware => "images/packet_malware.png",
                    PacketType::DDoS    => "images/packet_ddos.png",
                    PacketType::Reply   => "images/packet_reply.png",
                }),
                transform: Transform {
                    scale: Vec3::new(0.05, 0.05, 1f32),
                    ..default()
                },
                ..default()
            },
        });
    }
}
#[derive(Event)]
pub struct Packet_Remove;
pub fn remove(mut reader: EventReader<Packet_Remove>, mut commands: Commands, query: Query<Entity, With<Packet>>)
{
    for _ in reader.read() {
        for entity in query.iter()
        {
            commands.entity(entity).despawn();
        }
    }
}
