use ::bevy::prelude::*;
use bevy_health_bar3d::prelude::{HealthBarPlugin, Percentage};

use crate::Configs::World::WorldConfig;



#[derive(Bundle)]
pub struct ObjectBundle
{
    pub position: Position,
    pub speed: Speed,
    pub destination: Destination,
}



pub struct ObjectPlugin;
impl Plugin for ObjectPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Update, objevt_move);
        app.add_plugins(HealthBarPlugin::<Health>::default());
    }
}
#[derive(Component)]
pub struct Time {pub time: f32}
#[derive(Component)]
pub struct Position {pub position: Vec2}
#[derive(Component)]
pub struct Speed {pub speed: f32}
#[derive(Component)]
pub struct Destination {pub destination: Vec2}



pub fn objevt_move(
    mut query: Query<(&mut Position, &Speed, &Destination)>,
    time: Res<::bevy::prelude::Time>,
    world: Res<WorldConfig>,
)
{
    if world.simulation
    {
        for mut entity in query.iter_mut()
        {
            let mut direction = entity.2.destination - entity.0.position;
            if direction.length() > entity.1.speed * time.delta_seconds() {
                direction *= entity.1.speed * time.delta_seconds() / direction.length();
            }
            entity.0.position += direction;
        }
    }
}

#[derive(::bevy::ecs::component::Component, Reflect)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}
impl Percentage for Health {
    fn value(&self) -> f32 {
        return self.current / self.max;
    }
}