use ::bevy::prelude::*;
use bevy_health_bar3d::{configuration::{BarOrientation, BarSettings, ColorScheme, ForegroundColor, Percentage}, plugin::HealthBarPlugin};
use crate::
{
    Random::get_random_f32,
    Configs::
    {
        Player::
        {
            Layer,
            PlayerConfig,
        },
        World::WorldConfig,
    },
    Object::*,
    Levels::
    {
        Software::Packet::*,
        Hardware::Processor::Processor,
    }
};



pub struct AppPlugin;
impl Plugin for AppPlugin
{
    fn build(&self, app: &mut bevy::prelude::App)
    {
        if !app.is_plugin_added::<ObjectPlugin>() {
            app.add_plugins(ObjectPlugin);
        }
        app.add_plugins(HealthBarPlugin::<App_Queue>::default());
        app.insert_resource(
            ColorScheme::<App_Queue>::new().foreground_color(ForegroundColor::Static(Color::rgb(0., 0., 1.).into())),
        );
        app.add_systems(Update, visibility);
        app.add_systems(Update, update_protection);
        app.add_systems(Update, update_position);
        app.add_systems(Update, receive_packets);
        app.add_systems(Update, work_on_packets);
        app.add_event::<App_Place>();
        app.add_systems(Update, place);
        app.add_event::<App_Remove>();
        app.add_systems(Update, remove);
    }
}



#[derive(Component)]
pub struct App
{
    pub enabledCache: bool,
    pub workTime: f32,
}
pub struct App_Packet_Info
{
    pub to: Vec2,
    pub timestamp: f32,
    pub expire: f32,
    pub ddos: bool,
}
#[derive(Component)]
pub struct App_Queue
{
    packets: Vec<App_Packet_Info>,
}
impl TypePath for App_Queue
{
    fn crate_name() -> Option<&'static str> {Some("ResilientGame")}
    fn module_path() -> Option<&'static str> {Some("Softwarlevel::App")}
    fn short_type_path() -> &'static str {"App_Queue"}
    fn type_ident() -> Option<&'static str> {Some("App_Queue")}
    fn type_path() -> &'static str {"App_Queue"}
}
impl Percentage for App_Queue {fn value(&self) -> f32 {self.packets.len() as f32 / App_Capacity as f32}}
pub const App_Hight : f32 = Packet_Hight + 1.;
pub const App_Health: f32 = 100.;
pub const App_Damage: f32 = App_Health / 10.;
pub const App_WorkingTime: f32 = 0.3;
pub const App_Capacity: usize = 10;
pub const App_ProtectionTime: f32 = 10.;
pub const App_Recovery: f32 = App_Health * 2. / App_ProtectionTime;
#[derive(Bundle)]
pub struct AppBundle
{
    pub app: App,
    pub queue: App_Queue,
    pub queue_settings: BarSettings<App_Queue>,
    pub object: ObjectBundle,
    pub sprite: SpriteBundle,
    pub health: Health,
    pub health_settings: BarSettings<Health>,
}



pub fn visibility(
    mut query: Query<&mut Visibility, With<App>>,
    player: Res<PlayerConfig>,
){
    if player.layer == Layer::Software {
        for mut app in query.iter_mut()
        {
            *app = Visibility::Visible;
        }
    } else {
        for mut app in query.iter_mut()
        {
            *app = Visibility::Hidden;
        }
    }
}
pub fn update_position(mut query: Query<(&mut Transform, &Position), With<App>>)
{
    for mut entity in query.iter_mut()
    {
        entity.0.translation = entity.1.position.extend(App_Hight);
    }
}
pub fn update_protection(
    mut query: Query<(&mut App, &mut Handle<Image>, &mut Health)>,
    asset_server: Res<AssetServer>,
    time: Res<::bevy::prelude::Time>,
    mut player: ResMut<PlayerConfig>, 
    world : Res<WorldConfig>, 
){
    if world.simulation
    {
        player.antivirus -= time.delta_seconds();
        if player.antivirus < 0. {
            player.antivirus = 0.;
        }
        player.firewall  -= time.delta_seconds();
        if player.firewall < 0. {
            player.firewall = 0.;
        }
        match (player.antivirus > 0., player.firewall > 0.)
        {
            (false, false) => {for mut entity in query.iter_mut() {*entity.1 = asset_server.load("images/app.png"                 );}},
            (false, true ) => {for mut entity in query.iter_mut() {*entity.1 = asset_server.load("images/app_walled.png"          );}},
            (true , false) => {for mut entity in query.iter_mut() {*entity.1 = asset_server.load("images/app_protected.png"       ); entity.2.current += App_Recovery * time.delta_seconds(); if entity.2.current > entity.2.max {entity.2.current = entity.2.max;}}},
            (true , true ) => {for mut entity in query.iter_mut() {*entity.1 = asset_server.load("images/app_protected_walled.png"); entity.2.current += App_Recovery * time.delta_seconds(); if entity.2.current > entity.2.max {entity.2.current = entity.2.max;}}},
        }
    }
}
pub fn receive_packets(
    mut commands: Commands,
    packets: Query<(Entity, &Packet, &Position, &PacketType)>,
    mut apps: Query<(&Position, &App, &mut Health, &mut App_Queue, Entity)>, 
    mut processors: Query<&mut Processor>,
    player: Res<PlayerConfig>, 
    world : Res<WorldConfig>, 
){
    for packet in packets.iter()
    {
        for mut app in apps.iter_mut()
        {
            if player.firewall > 0. && packet.2.position.distance(app.0.position) < 120.{
                match packet.3 {
                    PacketType::DDoS => {
                        match commands.get_entity(packet.0) {Some(mut entity) => entity.despawn(), None => {},}
                    },
                    _ => {},
                }
            }
            if packet.2.position.distance(app.0.position) < 50. {
                match packet.3
                {
                    PacketType::User =>
                    {
                        if app.3.packets.len() < App_Capacity && app.1.enabledCache {
                            app.3.packets.insert(0, App_Packet_Info { to: packet.1.from, timestamp: 0., expire: 0., ddos: false });
                        }
                        match commands.get_entity(packet.0) {Some(mut entity) => entity.despawn(), None => {},}
                        for mut processor in processors.iter_mut()
                        {
                            if processor.app == app.4
                            {
                                processor.disks_time += world.get_disk_failtime() / world.get_disk_packets();
                            }
                        }
                    },
                    PacketType::Malware =>
                    {
                        match commands.get_entity(packet.0) {Some(mut entity) => entity.despawn(), None => {},}
                        if player.antivirus <= 0. {
                            app.2.current -= App_Damage;
                            if app.2.current < 0. {
                                app.2.current = 0.;
                            }
                        }
                    },
                    PacketType::DDoS =>
                    {
                        if app.3.packets.len() < App_Capacity && app.1.enabledCache {
                            app.3.packets.insert(0, App_Packet_Info { to: packet.1.from, timestamp: 0., expire: 0., ddos: true });
                        }
                        match commands.get_entity(packet.0) {Some(mut entity) => entity.despawn(), None => {},}
                        for mut processor in processors.iter_mut()
                        {
                            if processor.app == app.4
                            {
                                processor.disks_time += world.get_disk_failtime() / world.get_disk_packets();
                            }
                        }
                    },
                    PacketType::Reply => {},
                }
            }
        }
    }
}
pub fn work_on_packets(
    mut writer_packet: EventWriter<Packet_Place>,
    mut apps: Query<(&Position, &mut App, &mut App_Queue, &Health)>,
    time: Res<::bevy::prelude::Time>,
    world : Res<WorldConfig>, 
){
    if world.simulation
    {
        for mut app in apps.iter_mut()
        {
            app.1.workTime -= time.delta_seconds();
            while app.1.workTime <= 0.
            {
                app.1.workTime += App_WorkingTime;
                match app.2.packets.pop()
                {
                    Some(packet_info) =>
                    {
                        if !packet_info.ddos {
                            if get_random_f32() < app.3.value() {
                                writer_packet.send(Packet_Place {
                                    pos: app.0.position,
                                    to: packet_info.to,
                                    ptype: PacketType::Reply,
                                });
                            }
                        }
                    },
                    None => {},
                }
            }
        }
    }
}



#[derive(Event)]
pub struct App_Place
{
    pub pos: Vec2,
}
pub fn place(
    mut reader: EventReader<App_Place>,
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
    commands.spawn(AppBundle
    {
        app: App {
            enabledCache: true,
            workTime: 0.,
        },
        queue: App_Queue {
            packets: Vec::with_capacity(App_Capacity),
        },
        queue_settings: BarSettings::<App_Queue> {
            width: 5.,
            offset: 5.,
            orientation: BarOrientation::Vertical,
            ..Default::default()
        },
        object: ObjectBundle
        {
            position   : Position    {position   : pos},
            destination: Destination {destination: pos},
            speed      : Speed       {speed      : 0f32},
        },
        sprite: SpriteBundle {
            texture: asset_server.load("images/app.png"),
            transform: Transform {
                scale: Vec3::new(0.25, 0.25, 1f32),
                ..Default::default()
            },
            ..default()
        },
        health: Health {
            max: 100.,
            current: 100.,
        },
        health_settings: BarSettings::<Health> {
            width: 5.,
            offset: 4.,
            orientation: BarOrientation::Vertical,
            ..Default::default()
        },
    }).id()
}
#[derive(Event)]
pub struct App_Remove;
pub fn remove(mut reader: EventReader<App_Remove>, mut commands: Commands, query: Query<Entity, With<App>>)
{
    for _ in reader.read() {
        for entity in query.iter()
        {
            commands.entity(entity).despawn();
        }
    }
}
