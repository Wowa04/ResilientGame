use ::bevy::prelude::*;
use crate::
{
    Configs::Player::
    {
        Layer,
        PlayerConfig,
    },
    Object::*,
    Levels::Software::Packet::*,
};

use super::Processor::Processor;



pub struct DiskPlugin;
impl Plugin for DiskPlugin
{
    fn build(&self, app: &mut ::bevy::prelude::App)
    {
        if !app.is_plugin_added::<ObjectPlugin>() {
            app.add_plugins(ObjectPlugin);
        }
        app.add_systems(Update, visibility);
        app.add_systems(Update, update_position);
        app.add_event::<Disk_Place>();
        app.add_systems(Update, place);
        app.add_event::<Disk_Buy>();
        app.add_systems(Update, buy);
        app.add_event::<Disk_Remove>();
        app.add_systems(Update, remove);
    }
}


#[derive(Component)]
pub struct Disk {}
pub const Disk_Hight: f32 = Packet_Hight + 1.;
#[derive(Bundle)]
pub struct DiskBundle
{
    pub disk  : Disk,
    pub object: ObjectBundle,
    pub sprite: SpriteBundle,
}




pub fn visibility(
    mut query: Query<&mut Visibility, With<Disk>>,
    player: Res<PlayerConfig>,
){
    if player.layer == Layer::Hardware {
        for mut disk in query.iter_mut()
        {
            *disk = Visibility::Visible;
        }
    } else {
        for mut disk in query.iter_mut()
        {
            *disk = Visibility::Hidden;
        }
    }
}
pub fn update_position(mut query: Query<(&mut Transform, &Position), With<Disk>>)
{
    for mut entity in query.iter_mut()
    {
        entity.0.translation = entity.1.position.extend(Disk_Hight);
    }
}



#[derive(Event)]
pub struct Disk_Place
{
    pub pos: Vec2,
}
pub fn place(mut reader: EventReader<Disk_Place>, mut commands: Commands, asset_server: Res<AssetServer>)
{
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
    commands.spawn(DiskBundle {
        disk: Disk {},
        object: ObjectBundle {
            position   : Position    {position   : pos},
            speed      : Speed       {speed      : 0f32},
            destination: Destination {destination: pos},
        },
        sprite: SpriteBundle {
            texture: asset_server.load("images/disk.png"),
            transform: Transform {
                scale: Vec3::new(0.1, 0.1, 1f32),
                ..default()
            },
            ..default()
        },
    }).id()
}
#[derive(Event)]
pub struct Disk_Buy {}
pub fn buy(
    mut reader: EventReader<Disk_Buy>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Processor, &Position)>,
){
    for _ in reader.read()
    {
        for mut processor in query.iter_mut()
        {
            if processor.0.disks.len() <= 3 {
                let offset = (processor.0.disks.len() as f32 + 2.) * 100.;
                let disk = spawn(&mut commands, &asset_server, processor.1.position + Vec2::new(offset, 0.));
                processor.0.disks.push(disk);
            }
        }
    }
}
#[derive(Event)]
pub struct Disk_Remove;
pub fn remove(mut reader: EventReader<Disk_Remove>, mut commands: Commands, query: Query<Entity, With<Disk>>)
{
    for _ in reader.read() {
        for entity in query.iter()
        {
            commands.entity(entity).despawn();
        }
    }
}
