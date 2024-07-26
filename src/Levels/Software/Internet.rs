use ::bevy::prelude::*;
use crate::
{
    Configs::
    {
        Player::
        {
            Layer,
            PlayerConfig,
        },
        World::WorldConfig
    },
    Object::*,
    Levels::Software::Packet::*,
};



pub struct InternetPlugin;
impl Plugin for InternetPlugin
{
    fn build(&self, app: &mut ::bevy::prelude::App)
    {
        if !app.is_plugin_added::<ObjectPlugin>() {
            app.add_plugins(ObjectPlugin);
        }
        app.add_systems(Update, visibility);
        app.add_systems(Update, update_position);
        app.add_systems(Update, send_packet);
        app.add_systems(Update, receive_packet);
        app.add_event::<Internet_Place>();
        app.add_systems(Update, place);
        app.add_event::<Internet_Remove>();
        app.add_systems(Update, remove);
    }
}


#[derive(Component)]
pub struct Internet
{
    time_user   : f32,
    time_malware: f32,
    time_ddos   : f32,
    to          : Vec2,
}
pub const Internet_Hight: f32 = Packet_Hight + 1.;
#[derive(Bundle)]
pub struct InternetBundle
{
    pub ethernet: Internet,
    pub object  : ObjectBundle,
    pub sprite  : SpriteBundle,
}



pub fn visibility(
    mut query: Query<&mut Visibility, With<Internet>>,
    player: Res<PlayerConfig>,
){
    if player.layer == Layer::Software {
        for mut internet in query.iter_mut()
        {
            *internet = Visibility::Visible;
        }
    } else {
        for mut internet in query.iter_mut()
        {
            *internet = Visibility::Hidden;
        }
    }
}
pub fn update_position(mut query: Query<(&mut Transform, &Position), With<Internet>>)
{
    for mut entity in query.iter_mut()
    {
        entity.0.translation = entity.1.position.extend(Internet_Hight);
    }
}
pub fn send_packet(
    mut writer_packet: EventWriter<Packet_Place>,
    mut query: Query<(&mut Internet, &Position)>,
    time: Res<::bevy::prelude::Time>,
    config: Res<WorldConfig>,
){
    if config.simulation
    {
        for mut ethernet in query.iter_mut()
        {
            ethernet.0.time_user += time.delta_seconds();
            ethernet.0.time_malware += time.delta_seconds();
            ethernet.0.time_ddos += time.delta_seconds();
            while ethernet.0.time_user > 0.
            {
                writer_packet.send(Packet_Place { pos: ethernet.1.position, to: ethernet.0.to, ptype: PacketType::User });
                ethernet.0.time_user -= config.get_packet_arrival_delay_user();
            }
            while ethernet.0.time_malware > 0.
            {
                writer_packet.send(Packet_Place { pos: ethernet.1.position, to: ethernet.0.to, ptype: PacketType::Malware });
                ethernet.0.time_malware -= config.get_packet_arrival_delay_malware();
            }
            while ethernet.0.time_ddos > 0.
            {
                writer_packet.send(Packet_Place { pos: ethernet.1.position, to: ethernet.0.to, ptype: PacketType::DDoS });
                ethernet.0.time_ddos -= config.get_packet_arrival_delay_ddos();
            }
        }
    }
}

pub fn receive_packet(
    mut commands: Commands,
    packets: Query<(Entity, &Packet, &Position, &PacketType)>,
    apps: Query<(&Position, &Internet)>,
    world: Res<WorldConfig>,
    mut player: ResMut<PlayerConfig>,
){
    for packet in packets.iter()
    {
        for app in apps.iter()
        {
            if packet.2.position.distance(app.0.position) < 1f32 {
                match packet.3
                {
                    PacketType::Reply =>
                    {
                        match commands.get_entity(packet.0) {Some(mut entity) => entity.despawn(), None => {},}
                        *player.mut_money() += world.get_money_per_request();
                    },
                    _ => {},
                }
            }
        }
    }
}



#[derive(Event)]
pub struct Internet_Place
{
    pub pos: Vec2,
    pub to : Vec2,
}
pub fn place(mut reader: EventReader<Internet_Place>, mut commands: Commands, asset_server: Res<AssetServer>)
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
    commands.spawn(InternetBundle {
        ethernet: Internet {
            time_user   : 0.,
            time_malware: 0.,
            time_ddos   : 0.,
            to          : to,
        },
        object: ObjectBundle {
            position   : Position    {position   : pos},
            speed      : Speed       {speed      : 0f32},
            destination: Destination {destination: pos},
        },
        sprite: SpriteBundle {
            texture: asset_server.load("images/internet2.png"),
            transform: Transform {
                scale: Vec3::new(0.05, 0.05, 1f32),
                ..default()
            },
            ..default()
        },
    }).id()
}
#[derive(Event)]
pub struct Internet_Remove;
pub fn remove(mut reader: EventReader<Internet_Remove>, mut commands: Commands, query: Query<Entity, With<Packet>>)
{
    for _ in reader.read() {
        for entity in query.iter()
        {
            commands.entity(entity).despawn();
        }
    }
}
