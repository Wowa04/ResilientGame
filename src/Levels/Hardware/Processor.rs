use ::bevy::prelude::*;
use crate::
{
    Configs::{Player::{Layer, PlayerConfig}, World::WorldConfig},
    Levels::Software::App::{self, App_Hight},
    Object::*,
};

use super::Disk;



pub struct ProcessorPlugin;
impl Plugin for ProcessorPlugin
{
    fn build(&self, app: &mut bevy::prelude::App)
    {
        if !app.is_plugin_added::<ObjectPlugin>() {
            app.add_plugins(ObjectPlugin);
        }
        app.add_systems(Update, visibility);
        app.add_systems(Update, update_position);
        app.add_systems(Update, update_app);
        app.add_systems(Update, update_disks);
        app.add_event::<Processor_Place>();
        app.add_systems(Update, place);
        app.add_event::<Processor_Remove>();
        app.add_systems(Update, remove);
    }
}



#[derive(Component)]
pub struct Processor
{
    pub app: Entity,
    pub disks: Vec<Entity>,
    pub disks_time: f32
}
pub struct Processor_Packet_Info
{
    pub to: Vec2,
    pub timestamp: f32,
    pub expire: f32,
    pub ddos: bool,
}
pub const Processor_Hight : f32 = App_Hight + 1.;
pub const Processor_Health: f32 = 100.;
pub const Processor_Damage: f32 = Processor_Health / 10.;
pub const Processor_WorkingTime: f32 = 0.3;
pub const Processor_Capacity: usize = 10;
pub const Processor_ProtectionTime: f32 = 10.;
pub const Processor_Recovery: f32 = Processor_Health * 2. / Processor_ProtectionTime;
#[derive(Bundle)]
pub struct ProcessorBundle
{
    pub app: Processor,
    pub object: ObjectBundle,
    pub sprite: SpriteBundle,
}




pub fn visibility(
    mut query: Query<&mut Visibility, With<Processor>>,
    player: Res<PlayerConfig>,
){
    if player.layer == Layer::Hardware {
        for mut processor in query.iter_mut()
        {
            *processor = Visibility::Visible;
        }
    } else {
        for mut processor in query.iter_mut()
        {
            *processor = Visibility::Hidden;
        }
    }
}
pub fn update_position(mut query: Query<(&mut Transform, &Position), With<Processor>>)
{
    for mut entity in query.iter_mut()
    {
        entity.0.translation = entity.1.position.extend(Processor_Hight);
    }
}
pub fn update_app(processors: Query<&Processor>, mut apps: Query<(Entity, &mut App::App)>)
{
    for processor in processors.iter()
    {
        for mut app in apps.iter_mut()
        {
            if app.0 == processor.app {
                app.1.enabledCache = processor.disks.len() > 0;
            }
        }
    }
}
pub fn update_disks(mut commands: Commands, mut processors: Query<&mut Processor>, time: Res<::bevy::prelude::Time>, world: Res<WorldConfig>)
{
    if world.simulation
    {
        for mut processor in processors.iter_mut()
        {
            processor.disks_time += time.delta_seconds();
            while (processor.disks_time > world.get_disk_failtime()) && (processor.disks.len() > 0)
            {
                processor.disks_time -= world.get_disk_failtime();
                match processor.disks.pop()
                {
                    Some(disk) => commands.get_entity(disk).and_then(|mut entity| {entity.despawn(); Some(())}),
                    None => Some(()),
                };
            }
        }
    }
}



#[derive(Event)]
pub struct Processor_Place
{
    pub pos: Vec2,
}
pub fn place(
    mut reader: EventReader<Processor_Place>,
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
    let disk_id = Disk::spawn(commands, asset_server, pos + Vec2::new(200., 0.));
    let app_id = App::spawn(commands, asset_server, pos);
    commands.spawn(ProcessorBundle
    {
        app: Processor {
            app: app_id,
            disks: vec![disk_id],
            disks_time: 0.,
        },
        object: ObjectBundle
        {
            position   : Position    {position   : pos},
            destination: Destination {destination: pos},
            speed      : Speed       {speed      : 0f32},
        },
        sprite: SpriteBundle {
            texture: asset_server.load("images/processor.png"),
            transform: Transform {
                scale: Vec3::new(0.25, 0.25, 1f32),
                ..Default::default()
            },
            ..default()
        },
    }).id()
}
#[derive(Event)]
pub struct Processor_Remove;
pub fn remove(mut reader: EventReader<Processor_Remove>, mut commands: Commands, query: Query<Entity, With<Processor>>)
{
    for _ in reader.read() {
        for entity in query.iter()
        {
            commands.entity(entity).despawn();
        }
    }
}
